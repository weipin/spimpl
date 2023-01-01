// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::{
    AuthDataSize, FixedSizeAuthDataSource, VariableSizeAuthDataSource,
};
use crate::enr::{NodeId, Scheme};
use std::marker::PhantomData;
use std::mem;

// authdata      = authdata-head || id-signature || eph-pubkey || record
// authdata-head = src-id || sig-size || eph-key-size
// authdata-size = 34 + sig-size + eph-key-size + len(record)
// sig-size      = uint8     -- value: 64 for ID scheme "v4"
// eph-key-size  = uint8     -- value: 33 for ID scheme "v4"

type SigSize = u8;
type EphKeySize = u8;

pub(crate) struct HandshakeInitialMessageAuthData<'a, 'b, 'c, S: Scheme> {
    src_id: &'a NodeId,
    id_signature: &'b [u8],
    eph_pubkey: &'c [u8],
    phantom: PhantomData<S>,
}

impl<'a, 'b, 'c, S: Scheme> HandshakeInitialMessageAuthData<'a, 'b, 'c, S> {
    pub(crate) fn new(src_id: &'a NodeId, id_signature: &'b [u8], eph_pubkey: &'c [u8]) -> Self {
        debug_assert_eq!(id_signature.len(), S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        debug_assert_eq!(eph_pubkey.len(), S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);

        HandshakeInitialMessageAuthData {
            src_id,
            id_signature,
            eph_pubkey,
            phantom: Default::default(),
        }
    }
}

impl<S: Scheme> FixedSizeAuthDataSource for HandshakeInitialMessageAuthData<'_, '_, '_, S> {
    const SIZE: AuthDataSize = (mem::size_of::<NodeId>()
        + mem::size_of::<SigSize>()
        + mem::size_of::<EphKeySize>()
        + S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH
        + S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH) as AuthDataSize;

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>) {
        // authdata-head = src-id || sig-size || eph-key-size
        buffer.extend(self.src_id.0);
        buffer.push(u8::try_from(S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH).unwrap());
        buffer.push(u8::try_from(S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH).unwrap());
        // id-signature || eph-pubkey
        buffer.extend_from_slice(self.id_signature);
        buffer.extend_from_slice(self.eph_pubkey);
    }
}

// variable
pub(crate) struct HandshakeInitialMessageWithRecordAuthData<'a, 'b, 'c, 'd, S: Scheme> {
    src_id: &'a NodeId,
    id_signature: &'b [u8],
    eph_pubkey: &'c [u8],
    record_data: &'d [u8],
    phantom: PhantomData<S>,
}

impl<'a, 'b, 'c, 'd, S: Scheme> HandshakeInitialMessageWithRecordAuthData<'a, 'b, 'c, 'd, S> {
    pub(crate) fn new(
        src_id: &'a NodeId,
        id_signature: &'b [u8],
        eph_pubkey: &'c [u8],
        record_data: &'d [u8],
    ) -> Self {
        debug_assert_eq!(id_signature.len(), S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        debug_assert_eq!(eph_pubkey.len(), S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);

        HandshakeInitialMessageWithRecordAuthData {
            src_id,
            id_signature,
            eph_pubkey,
            record_data,
            phantom: Default::default(),
        }
    }
}

impl<S: Scheme> VariableSizeAuthDataSource
    for HandshakeInitialMessageWithRecordAuthData<'_, '_, '_, '_, S>
{
    fn size(&self) -> AuthDataSize {
        (mem::size_of::<NodeId>()
            + mem::size_of::<SigSize>()
            + mem::size_of::<EphKeySize>()
            + S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH
            + S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH) as AuthDataSize
            + AuthDataSize::try_from(self.record_data.len()).unwrap()
    }

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>) {
        // authdata-head = src-id || sig-size || eph-key-size
        buffer.extend(self.src_id.0);
        buffer.push(u8::try_from(S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH).unwrap());
        buffer.push(u8::try_from(S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH).unwrap());
        // id-signature || eph-pubkey || record
        buffer.extend_from_slice(self.id_signature);
        buffer.extend_from_slice(self.eph_pubkey);
        buffer.extend_from_slice(self.record_data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discv5::crypto::id_nonce_signing::{
        id_sign, id_signature_input_byte_length, ID_SIGNATURE_TEXT, ID_SIGNATURE_TEXT_BYTE_LENGTH,
    };
    use crate::discv5::message;
    use crate::discv5::message::protocol::ping::Ping;
    use crate::discv5::message::RequestId;
    use crate::discv5::packet::flag::Flag;
    use crate::discv5::packet::flag::Flag::Handshake;
    use crate::discv5::packet::header::{
        encode_fixed_size_header_to_buffer, encode_variable_size_header_to_buffer,
        fixed_size_encoded_header_byte_length,
    };
    use crate::discv5::packet::masked_header::MaskingIv;
    use crate::discv5::packet::packing::pack_message;
    use crate::enr;
    use crate::enr::Schemev4;
    use hex_literal::hex;
    use secp256k1::SECP256K1;
    use std::net::Ipv4Addr;

    #[test]
    fn test_packaging_ping_handshake() {
        let src_node_id_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_node_id = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let ping_req_id_data = hex!("00000001");
        let ping_enr_seq = 1;

        let ping = Ping {
            request_id: RequestId(ping_req_id_data.to_vec()),
            enr_seq: ping_enr_seq,
        };

        // id_signature_data
        let node_a_key_data =
            hex!("eef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f");
        let node_a_key = Schemev4::value_to_private_key(&node_a_key_data).unwrap();
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000001");
        let mut id_signature_input_buffer = [0; id_signature_input_byte_length::<Schemev4>()];
        id_signature_input_buffer[..ID_SIGNATURE_TEXT_BYTE_LENGTH]
            .copy_from_slice(ID_SIGNATURE_TEXT);
        let mut id_signature_output_buffer = [0; Schemev4::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH];
        let id_signature_data = id_sign::<Schemev4>(
            &mut id_signature_input_buffer,
            &challenge_data,
            &eph_pubkey_data,
            &enr::NodeId(dest_node_id),
            &node_a_key,
        )
        .unwrap();

        //
        let src_id = NodeId(src_node_id_data);

        let auth_data_source = HandshakeInitialMessageAuthData::<Schemev4>::new(
            &src_id,
            &id_signature_data,
            &eph_pubkey_data,
        );

        let nonce = message::Nonce(nonce_data);
        let mut header_data = Vec::with_capacity(fixed_size_encoded_header_byte_length::<
            HandshakeInitialMessageAuthData<Schemev4>,
        >());
        encode_fixed_size_header_to_buffer(
            &mut header_data,
            &auth_data_source,
            Flag::Handshake,
            &nonce,
        );

        let masking_iv = MaskingIv(hex!("00000000000000000000000000000000"));
        let initiator_key_data = hex!("4f9fac6de7567d1e3b1241dffe90f662");
        let package_data = pack_message(
            &ping,
            &mut header_data,
            &NodeId(dest_node_id),
            &nonce,
            &masking_iv,
            &initiator_key_data,
        );

        assert_eq!(
            hex::encode(package_data),
            // concat!(
            //     "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
            //     "ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
            //     "4c4f53245d08da4bb252012b2cba3f4f374a90a75cff91f142fa9be3e0a5f3ef",
            //     "268ccb9065aeecfd67a999e7fdc137e062b2ec4a0eb92947f0d9a74bfbf44dfb",
            //     "a776b21301f8b65efd5796706adff216ab862a9186875f9494150c4ae06fa4d1",
            //     "f0396c93f215fa4ef524f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d83",
            //     "9cf8"
            // )
            concat!(
                "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
                "ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
                "4c4f53245d08da4bb222b18102a2fec03e282ad74f3d3ca9fd1c0b179f86dbd6",
                "6f5f9130201c4bedb9bc32c51e466230c59ae4c61437cb7ae589910447bf37dc",
                "a7c3316df83efbdd485796706adff216ab862a9186875f9494150c4ae06fa4d1",
                "f0396c93f215fa4ef524f1eadf5f0f4126b716d8f2f3cf4f4d674d515d460900",
                "2b5f"
            )
        );
    }

    #[test]
    fn test_packaging_ping_handshake_with_record() {
        let src_node_id_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_node_id = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let ping_req_id_data = hex!("00000001");
        let ping_enr_seq = 1;

        let ping = Ping {
            request_id: RequestId(ping_req_id_data.to_vec()),
            enr_seq: ping_enr_seq,
        };

        // id_signature_data
        let node_a_key_data =
            hex!("eef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f");
        let node_a_key = Schemev4::value_to_private_key(&node_a_key_data).unwrap();
        let node_a_public_key = node_a_key.public_key(SECP256K1);
        let eph_pubkey_data =
            hex!("039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000");
        let mut id_signature_input_buffer = [0; id_signature_input_byte_length::<Schemev4>()];
        id_signature_input_buffer[..ID_SIGNATURE_TEXT_BYTE_LENGTH]
            .copy_from_slice(ID_SIGNATURE_TEXT);
        let mut id_signature_output_buffer = [0; Schemev4::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH];
        let id_signature_data = id_sign::<Schemev4>(
            &mut id_signature_input_buffer,
            &challenge_data,
            &eph_pubkey_data,
            &enr::NodeId(dest_node_id),
            &node_a_key,
        )
        .unwrap();

        //
        let src_id = NodeId(src_node_id_data);
        let record = enr::Builder::new()
            .with_seq(1)
            .with_ip4(Ipv4Addr::from(hex!("7f000001")))
            .sign_and_build::<Schemev4>(&node_a_key, &node_a_public_key)
            .unwrap();
        let record_data = record.rlp_data::<Schemev4>().unwrap();

        let auth_data_source = HandshakeInitialMessageWithRecordAuthData::<Schemev4>::new(
            &src_id,
            &id_signature_data,
            &eph_pubkey_data,
            &record_data,
        );

        let nonce = message::Nonce(nonce_data);
        let mut header_data = Vec::new();
        encode_variable_size_header_to_buffer(
            &mut header_data,
            &auth_data_source,
            Flag::Handshake,
            &nonce,
        );

        let masking_iv = MaskingIv(hex!("00000000000000000000000000000000"));
        let initiator_key_data = hex!("53b1c075f41876423154e157470c2f48");
        let package_data = pack_message(
            &ping,
            &mut header_data,
            &NodeId(dest_node_id),
            &nonce,
            &masking_iv,
            &initiator_key_data,
        );

        assert_eq!(
            hex::encode(package_data),
            // concat!(
            //     "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
            //     "ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
            //     "4c4f53245d08da4bb23698868350aaad22e3ab8dd034f548a1c43cd246be9856",
            //     "2fafa0a1fa86d8e7a3b95ae78cc2b988ded6a5b59eb83ad58097252188b902b2",
            //     "1481e30e5e285f19735796706adff216ab862a9186875f9494150c4ae06fa4d1",
            //     "f0396c93f215fa4ef524e0ed04c3c21e39b1868e1ca8105e585ec17315e755e6",
            //     "cfc4dd6cb7fd8e1a1f55e49b4b5eb024221482105346f3c82b15fdaae36a3bb1",
            //     "2a494683b4a3c7f2ae41306252fed84785e2bbff3b022812d0882f06978df84a",
            //     "80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e",
            //     "4539717307a0208cd208d65093ccab5aa596a34d7511401987662d8cf62b1394",
            //     "71"
            // )
            concat!(
                "00000000000000000000000000000000088b3d4342774649305f313964a39e55",
                "ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
                "4c4f53245d08da4bb2c9739df21f599113149bb36fc159560358ab505b42ae00",
                "38c08fc1471db072fff5fe03f66ba346701b3be393e0d03ba714573f06c1ec08",
                "a97f164214371a3e3f5796706adff216ab862a9186875f9494150c4ae06fa4d1",
                "f0396c93f215fa4ef524e0ed04c38843b978618de68161c6b1cdf47420a834f2",
                "94ca22b931e51b615d9408f8e9b49bc54eb42a75bf165419c2da8dfb85eb4056",
                "e2c1e55ba7a09948fa3b19e161fed84785e2bbff3b022812d0882f06978df84a",
                "80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e",
                "4539717307a0208cd208d65093ccab5aa5a6b24bf02321a3e845116513e96780",
                "10"
            )
        );
    }
}
