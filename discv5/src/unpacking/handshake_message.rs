// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::{NodeId, RecordRlpEncoded, Scheme};

use crate::packet::constants::{
    size_of_handshake_message_authdata_fixed_part, STATIC_HEADER_BYTE_LENGTH,
};
use crate::packet::{aesgcm, MaskingIv};
use crate::types::Nonce;

use super::Error;

#[allow(clippy::type_complexity)]
pub fn unpack<S: Scheme>(
    masking_iv: &MaskingIv,
    read_key: &[u8; 16],
    nonce: &Nonce,
    static_header: &[u8],
    auth_data: &[u8],
    encrypted_message_data: &[u8],
) -> Result<(NodeId, S::Signature, S::PublicKey, Vec<u8>), Error> {
    debug_assert_eq!(static_header.len(), STATIC_HEADER_BYTE_LENGTH);
    if auth_data.len() != size_of_handshake_message_authdata_fixed_part::<S>() as usize {
        return Err(Error::InvalidAuthDataSize);
    }
    let (src_node_id, id_signature, eph_pubkey, _) = unpack_authdata_fixed_part::<S>(auth_data)?;

    let mut ad = vec![];
    ad.extend(masking_iv.bytes());
    ad.extend(static_header);
    ad.extend(auth_data);

    let mut message_data = encrypted_message_data.to_vec();
    if !aesgcm::decrypt(read_key, nonce.bytes(), &ad, &mut message_data) {
        return Err(Error::MessageDecryptingFailed);
    }

    if message_data.is_empty() {
        return Err(Error::InvalidMessageByteLength);
    }

    Ok((src_node_id, id_signature, eph_pubkey, message_data))
}

#[allow(clippy::type_complexity)]
pub fn unpack_with_record<S: Scheme>(
    masking_iv: &MaskingIv,
    read_key: &[u8; 16],
    nonce: &Nonce,
    static_header: &[u8],
    auth_data: &[u8],
    encrypted_message_data: &[u8],
) -> Result<
    (
        NodeId,
        S::Signature,
        S::PublicKey,
        RecordRlpEncoded,
        Vec<u8>,
    ),
    Error,
> {
    debug_assert_eq!(static_header.len(), STATIC_HEADER_BYTE_LENGTH);
    if auth_data.len() <= size_of_handshake_message_authdata_fixed_part::<S>() as usize {
        return Err(Error::InvalidAuthDataSize);
    }
    let (src_node_id, id_signature, eph_pubkey, remaining_authdata) =
        unpack_authdata_fixed_part::<S>(auth_data)?;
    let record_rlp_encoded = RecordRlpEncoded::from_vec(remaining_authdata.to_vec())
        .map_err(Error::EnrDecodingFailed)?;

    let mut ad = vec![];
    ad.extend(masking_iv.bytes());
    ad.extend(static_header);
    ad.extend(auth_data);

    let mut message_data = encrypted_message_data.to_vec();
    if !aesgcm::decrypt(read_key, nonce.bytes(), &ad, &mut message_data) {
        return Err(Error::MessageDecryptingFailed);
    }

    if message_data.is_empty() {
        return Err(Error::InvalidMessageByteLength);
    }

    Ok((
        src_node_id,
        id_signature,
        eph_pubkey,
        record_rlp_encoded,
        message_data,
    ))
}

#[inline]
#[allow(clippy::type_complexity)]
fn unpack_authdata_fixed_part<S: Scheme>(
    bytes: &[u8],
) -> Result<(NodeId, S::Signature, S::PublicKey, &[u8]), Error> {
    let fixed_part_len = size_of_handshake_message_authdata_fixed_part::<S>() as usize;
    debug_assert!(bytes.len() >= fixed_part_len);

    let (node_id_slice, remaining) = bytes.split_at(size_of::<NodeId>());
    let signature_size = u8::from_be_bytes([remaining[0]]);
    if signature_size as usize != S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH {
        return Err(Error::EnrDecodingFailed(
            enr::Error::SignatureDataWithInvalidByteLength,
        ));
    }
    let pubkey_size = u8::from_be_bytes([remaining[1]]);
    if pubkey_size as usize != S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH {
        return Err(Error::EnrDecodingFailed(
            enr::Error::PublicKeyDataWithInvalidByteLength,
        ));
    }

    let (signature_data, remaining) =
        remaining[2..].split_at(S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
    let signature = S::new_signature_from_bytes(signature_data)
        .map_err(|e| Error::EnrDecodingFailed(enr::Error::InvalidSignatureData(format!("{e}"))))?;

    let (pubkey_data, remaining) = remaining.split_at(S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);
    let pubkey = S::new_public_key_from_bytes(pubkey_data)
        .map_err(|e| Error::EnrDecodingFailed(enr::Error::InvalidPublicKeyData(format!("{e}"))))?;

    Ok((
        NodeId(node_id_slice.try_into().unwrap()),
        signature,
        pubkey,
        remaining,
    ))
}

#[cfg(test)]
mod tests {
    use enr::{NodeId, Scheme, Schemev4};
    use hex_literal::hex;

    use crate::messages::{decode_ping, decode_type, Ping, Type};
    use crate::packet::Flag;
    use crate::types::RequestId;
    use crate::unpacking::unpack;

    use super::{
        unpack as unpack_handshake_message,
        unpack_with_record as unpack_handshake_message_with_record,
    };

    #[test]
    fn test_unpack_handshake_message_package() {
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let dest_node_id = NodeId(dest_node_id_data);
        let read_key = hex!("4f9fac6de7567d1e3b1241dffe90f662");
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");
        let ping = Ping {
            request_id: RequestId::from_vec(hex!("00000001").to_vec()).unwrap(),
            enr_seq: 1,
        };
        // discv5_id_signature: `ping_handshake_example_id_nonce_signing_without_extra_entropy`
        let id_signature_data = hex!("c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db");
        let package_data = hex!(
            "00000000000000000000000000000000088b3d4342774649305f313964a39e55"
            "ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
            "4c4f53245d08da4bb252012b2cba3f4f374a90a75cff91f142fa9be3e0a5f3ef"
            "268ccb9065aeecfd67a999e7fdc137e062b2ec4a0eb92947f0d9a74bfbf44dfb"
            "a776b21301f8b65efd5796706adff216ab862a9186875f9494150c4ae06fa4d1"
            "f0396c93f215fa4ef524f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d83"
            "9cf8"
        );

        let (masking_iv, flag, nonce, static_header, auth_data, encrypted_message_data) =
            unpack(&dest_node_id, &package_data).unwrap();
        assert_eq!(flag, Flag::HandshakeMessage);

        let (src_node_id, id_signature, eph_pubkey, message_data) =
            unpack_handshake_message::<Schemev4>(
                &masking_iv,
                &read_key,
                &nonce,
                &static_header,
                &auth_data,
                encrypted_message_data,
            )
            .unwrap();
        assert_eq!(
            Schemev4::signature_to_bytes(&id_signature),
            id_signature_data
        );
        assert_eq!(Schemev4::public_key_to_bytes(&eph_pubkey), eph_pubkey_data);
        assert_eq!(
            src_node_id.0,
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb")
        );

        let (message_type, message_rlp_encoded) = decode_type(&message_data).unwrap();
        assert_eq!(message_type, Type::Ping);
        let decoded_ping = decode_ping(message_rlp_encoded).unwrap();
        assert_eq!(decoded_ping, ping);
    }

    #[test]
    fn test_unpack_handshake_message_package_with_record() {
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let dest_node_id = NodeId(dest_node_id_data);
        let read_key = hex!("53b1c075f41876423154e157470c2f48");
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");
        let ping = Ping {
            request_id: RequestId::from_vec(hex!("00000001").to_vec()).unwrap(),
            enr_seq: 1,
        };
        // discv5_id_signature: `ping_handshake_with_record_example_id_nonce_signing_without_extra_entropy`
        let id_signature_data = hex!("a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555");
        // eth_enr_v4: `discv5_example_record_without_extra_entropy`
        let record_rlp_encoded_data = hex!("f87db84017e1b073918da32d640642c762c0e2781698e4971f8ab39a77746adad83f01e76ffc874c5924808bbe7c50890882c2b8a01287a0b08312d1d53a17d517f5eb2701826964827634826970847f00000189736563703235366b31a10313d14211e0287b2361a1615890a9b5212080546d0a257ae4cff96cf534992cb9");
        let package_data = hex!(
            "00000000000000000000000000000000088b3d4342774649305f313964a39e55"
            "ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
            "4c4f53245d08da4bb23698868350aaad22e3ab8dd034f548a1c43cd246be9856"
            "2fafa0a1fa86d8e7a3b95ae78cc2b988ded6a5b59eb83ad58097252188b902b2"
            "1481e30e5e285f19735796706adff216ab862a9186875f9494150c4ae06fa4d1"
            "f0396c93f215fa4ef524e0ed04c3c21e39b1868e1ca8105e585ec17315e755e6"
            "cfc4dd6cb7fd8e1a1f55e49b4b5eb024221482105346f3c82b15fdaae36a3bb1"
            "2a494683b4a3c7f2ae41306252fed84785e2bbff3b022812d0882f06978df84a"
            "80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e"
            "4539717307a0208cd208d65093ccab5aa596a34d7511401987662d8cf62b1394"
            "71"
        );

        let (masking_iv, flag, nonce, static_header, auth_data, encrypted_message_data) =
            unpack(&dest_node_id, &package_data).unwrap();
        assert_eq!(flag, Flag::HandshakeMessage);

        let (src_node_id, id_signature, eph_pubkey, record_rlp_encoded, message_data) =
            unpack_handshake_message_with_record::<Schemev4>(
                &masking_iv,
                &read_key,
                &nonce,
                &static_header,
                &auth_data,
                encrypted_message_data,
            )
            .unwrap();
        assert_eq!(
            src_node_id.0,
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb")
        );
        assert_eq!(
            Schemev4::signature_to_bytes(&id_signature),
            id_signature_data
        );
        assert_eq!(Schemev4::public_key_to_bytes(&eph_pubkey), eph_pubkey_data);
        assert_eq!(record_rlp_encoded.bytes(), record_rlp_encoded_data);

        let (message_type, message_rlp_encoded) = decode_type(&message_data).unwrap();
        assert_eq!(message_type, Type::Ping);
        let decoded_ping = decode_ping(message_rlp_encoded).unwrap();
        assert_eq!(decoded_ping, ping);
    }
}
