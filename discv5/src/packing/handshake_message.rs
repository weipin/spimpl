// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::{NodeId, RecordRlpEncoded, Scheme};

use crate::messages::Message;
use crate::packet::constants::{
    size_of_handshake_message_authdata_fixed_part, PROTOCOL_ID, VERSION,
};
use crate::packet::flag::Flag;
use crate::packet::types::{AuthDataSize, MaskingIv};
use crate::packing::common::{pack_header, pack_message};
use crate::types::Nonce;

use super::error::Error;

#[allow(clippy::too_many_arguments)]
pub fn pack<'a, T: Message<'a>, S: Scheme>(
    message: &T,
    nonce: &Nonce,
    src_id: &NodeId,
    dest_id: &NodeId,
    masking_iv: &MaskingIv,
    initiator_key: &[u8; 16],
    id_signature: &[u8],
    eph_pubkey: &[u8],
) -> Result<Vec<u8>, Error> {
    debug_assert_eq!(id_signature.len(), S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
    debug_assert_eq!(eph_pubkey.len(), S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);

    let header = build_header::<S>(nonce, src_id, id_signature, eph_pubkey);
    let mut header_pt_in_ct_out = header.clone();
    let mut output = vec![];
    pack_header(
        masking_iv,
        dest_id,
        header_pt_in_ct_out.as_mut(),
        &mut output,
    );
    pack_message(
        initiator_key,
        nonce,
        masking_iv,
        &header,
        message,
        &mut output,
    )?;

    Ok(output)
}

#[allow(clippy::too_many_arguments)]
pub fn pack_with_record<'a, T: Message<'a>, S: Scheme>(
    message: &T,
    nonce: &Nonce,
    src_id: &NodeId,
    dest_id: &NodeId,
    masking_iv: &MaskingIv,
    initiator_key: &[u8; 16],
    id_signature: &[u8],
    eph_pubkey: &[u8],
    record: &RecordRlpEncoded,
) -> Result<Vec<u8>, Error> {
    debug_assert_eq!(id_signature.len(), S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
    debug_assert_eq!(eph_pubkey.len(), S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);

    let header = build_header_with_record::<S>(nonce, src_id, id_signature, eph_pubkey, record);
    let mut header_pt_in_ct_out = header.clone();
    let mut output = vec![];
    pack_header(
        masking_iv,
        dest_id,
        header_pt_in_ct_out.as_mut(),
        &mut output,
    );
    pack_message(
        initiator_key,
        nonce,
        masking_iv,
        &header,
        message,
        &mut output,
    )?;

    Ok(output)
}

fn build_header<S: Scheme>(
    nonce: &Nonce,
    src_id: &NodeId,
    id_signature: &[u8],
    eph_pubkey: &[u8],
) -> Vec<u8> {
    let mut output = vec![];

    output.extend(PROTOCOL_ID);
    output.extend(VERSION);
    output.push(Flag::HandshakeMessage.value());
    output.extend(nonce.bytes());
    output.extend(size_of_handshake_message_authdata_fixed_part::<S>().to_be_bytes());

    output.extend(src_id.bytes());
    output.push(u8::try_from(S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH).unwrap());
    output.push(u8::try_from(S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH).unwrap());
    output.extend(id_signature);
    output.extend(eph_pubkey);

    output
}

fn build_header_with_record<S: Scheme>(
    nonce: &Nonce,
    src_id: &NodeId,
    id_signature: &[u8],
    eph_pubkey: &[u8],
    record: &RecordRlpEncoded,
) -> Vec<u8> {
    let mut output = vec![];

    output.extend(PROTOCOL_ID);
    output.extend(VERSION);
    output.push(Flag::HandshakeMessage.value());
    output.extend(nonce.bytes());
    let auth_data_size = size_of_handshake_message_authdata_fixed_part::<S>()
        .checked_add(AuthDataSize::try_from(record.bytes().len()).unwrap())
        .unwrap();
    output.extend(auth_data_size.to_be_bytes());

    output.extend(src_id.bytes());
    output.push(u8::try_from(S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH).unwrap());
    output.push(u8::try_from(S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH).unwrap());
    output.extend(id_signature);
    output.extend(eph_pubkey);
    output.extend(record.bytes());

    output
}

#[cfg(test)]
mod tests {
    use enr::Schemev4;
    use hex_literal::hex;

    use crate::messages::Ping;
    use crate::types::RequestId;

    use super::*;

    #[test]
    fn test_packing() {
        let src_id_data = hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_id_data = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let initiator_key = hex!("4f9fac6de7567d1e3b1241dffe90f662");
        let request_id_data = hex!("00000001");
        let masking_iv_data = hex!("00000000000000000000000000000000");
        let enr_seq = 1;
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");

        // discv5_id_signature: `ping_handshake_example_id_nonce_signing_without_extra_entropy`
        let id_signature_data = hex!("c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db");

        let src_id = NodeId::from_slice(&src_id_data);
        let dest_id = NodeId::from_slice(&dest_id_data);
        let nonce = Nonce::from_array(nonce_data);
        let masking_iv = MaskingIv::from_slice(&masking_iv_data);
        let request_id = RequestId::from_slice(&request_id_data).unwrap();
        let ping = Ping {
            request_id,
            enr_seq,
        };

        let packed = pack::<_, Schemev4>(
            &ping,
            &nonce,
            &src_id,
            &dest_id,
            &masking_iv,
            &initiator_key,
            &id_signature_data,
            &eph_pubkey_data,
        )
        .unwrap();
        assert_eq!(
            hex::encode(&packed),
            concat!(
                "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
                "ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
                "4c4f53245d08da4bb252012b2cba3f4f374a90a75cff91f142fa9be3e0a5f3ef",
                "268ccb9065aeecfd67a999e7fdc137e062b2ec4a0eb92947f0d9a74bfbf44dfb",
                "a776b21301f8b65efd5796706adff216ab862a9186875f9494150c4ae06fa4d1",
                "f0396c93f215fa4ef524f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d83",
                "9cf8"
            )
        );
    }

    #[test]
    fn test_packing_with_record() {
        let src_id_data = hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_id_data = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let initiator_key = hex!("53b1c075f41876423154e157470c2f48");
        let request_id_data = hex!("00000001");
        let masking_iv_data = hex!("00000000000000000000000000000000");
        let enr_seq = 1;
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");

        // discv5_id_signature: `ping_handshake_with_record_example_id_nonce_signing_without_extra_entropy`
        let id_signature_data = hex!("a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555");

        let src_id = NodeId::from_slice(&src_id_data);
        let dest_id = NodeId::from_slice(&dest_id_data);
        let nonce = Nonce::from_array(nonce_data);
        let masking_iv = MaskingIv::from_slice(&masking_iv_data);
        let request_id = RequestId::from_slice(&request_id_data).unwrap();
        let ping = Ping {
            request_id,
            enr_seq,
        };

        // eth_enr_v4: `discv5_example_record_without_extra_entropy`
        let record_rlp_encoded_data = hex!("f87db84017e1b073918da32d640642c762c0e2781698e4971f8ab39a77746adad83f01e76ffc874c5924808bbe7c50890882c2b8a01287a0b08312d1d53a17d517f5eb2701826964827634826970847f00000189736563703235366b31a10313d14211e0287b2361a1615890a9b5212080546d0a257ae4cff96cf534992cb9");
        let record_rlp_encoded = RecordRlpEncoded::from_slice(&record_rlp_encoded_data).unwrap();

        let packed = pack_with_record::<_, Schemev4>(
            &ping,
            &nonce,
            &src_id,
            &dest_id,
            &masking_iv,
            &initiator_key,
            &id_signature_data,
            &eph_pubkey_data,
            &record_rlp_encoded,
        )
        .unwrap();
        assert_eq!(
            hex::encode(&packed),
            concat!(
                "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
                "ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
                "4c4f53245d08da4bb23698868350aaad22e3ab8dd034f548a1c43cd246be9856",
                "2fafa0a1fa86d8e7a3b95ae78cc2b988ded6a5b59eb83ad58097252188b902b2",
                "1481e30e5e285f19735796706adff216ab862a9186875f9494150c4ae06fa4d1",
                "f0396c93f215fa4ef524e0ed04c3c21e39b1868e1ca8105e585ec17315e755e6",
                "cfc4dd6cb7fd8e1a1f55e49b4b5eb024221482105346f3c82b15fdaae36a3bb1",
                "2a494683b4a3c7f2ae41306252fed84785e2bbff3b022812d0882f06978df84a",
                "80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e",
                "4539717307a0208cd208d65093ccab5aa596a34d7511401987662d8cf62b1394",
                "71",
            )
        );
    }
}
