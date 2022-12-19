// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::core::Message;
use crate::discv5::auth_data::core::{AuthDataSize, AuthDataSource};
use crate::discv5::message::RequestId;
use crate::enr;
use bytes::BytesMut;
use fastrlp::{Encodable, RlpEncodable};

#[derive(Debug, RlpEncodable)]
struct Ping {
    request_id: RequestId,
    enr_seq: enr::SequenceNumber,
}

impl Message for Ping {
    const TYPE: u8 = 0x01;

    fn encode_to_data(&self, data: &mut Vec<u8>) {
        let mut bytes = BytesMut::new();
        self.encode(&mut bytes);
        data.extend(bytes.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discv5::packet::flag::Flag;
    use crate::discv5::packet::header::encode_header_to_buffer;
    use crate::discv5::packet::masked_header::{mask_header_data, MaskingIv};
    use crate::discv5::packet::packing::pack_message;
    use crate::discv5::{device, message};
    use crate::enr::{NodeId, Scheme, Schemev4};
    use hex_literal::hex;
    use secp256k1::SECP256K1;
    use crate::discv5::crypto::keys_deriving::{derive_keys, KDF_INFO_BYTE_LENGTH, KDF_INFO_PREFIX, KDF_INFO_PREFIX_BYTE_LENGTH};

    #[test]
    fn test_packaging_ping() {
        let src_node_id_data = hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_node_id = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let read_key_data = hex!("00000000000000000000000000000000");
        let ping_req_id_data = hex!("0000000000000001");
        let ping_enr_seq = 2;

        // initiator_key
        // let ephemeral_key_data =
        //     hex!("0288ef00023598499cb6c940146d050d2b1fb914198c327f76aad590bead68b6");
        // let dest_pubkey_data =
        //     hex!("0317931e6e0840220642f230037d285d122bc59063221ef3226b1f403ddc69ca91");
        // let node_id_a_data =
        //     hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        // let node_id_b_data =
        //     hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        // let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000");
        //
        // let node_a_key_data = hex!("eef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f");
        // let node_b_key_data = hex!("66fb62bfbd66b9177a138c1e5cddbe4f7c30c343e94e68df8769459cb1cde628");
        // let node_b_key = Schemev4::value_to_private_key(&node_b_key_data).unwrap();
        // let node_b_public_key = node_b_key.public_key(SECP256K1);
        //
        // let ephemeral_key = Schemev4::value_to_private_key(&ephemeral_key_data).unwrap();
        // let dest_pubkey = node_b_public_key;
        // let node_id_a = NodeId(node_id_a_data);
        // let node_id_b = NodeId(node_id_b_data);
        // let mut kdf_info_buffer = [0; KDF_INFO_BYTE_LENGTH];
        // kdf_info_buffer[..KDF_INFO_PREFIX_BYTE_LENGTH].copy_from_slice(KDF_INFO_PREFIX);
        // let mut output_keying_material_buffer = [0; 32];
        // derive_keys::<Schemev4>(
        //     &mut kdf_info_buffer,
        //     &mut output_keying_material_buffer,
        //     &dest_pubkey,
        //     &ephemeral_key,
        //     &node_id_a,
        //     &node_id_b,
        //     &challenge_data,
        // );
        // let initiator_key = &output_keying_material_buffer[..16];

        // let nonce = message::Nonce(nonce_data);
        // let ping = Ping {
        //     request_id: RequestId(ping_req_id_data.try_into().unwrap()),
        //     enr_seq: ping_enr_seq,
        // };
        //
        // let mut header_data = vec![];
        // let auth_data_source = device::context::Context {
        //     node_id: NodeId(src_node_id_data),
        // };
        // encode_header_to_buffer(
        //     &mut header_data,
        //     &auth_data_source,
        //     Flag::OrdinaryMessage,
        //     &nonce,
        // );
        //
        // let package_data = pack_message(
        //     &ping,
        //     &mut header_data,
        //     &NodeId(dest_node_id),
        //     &nonce,
        //     &MaskingIv(hex!("00000000000000000000000000000000")),
        //     &initiator_key
        // );
        //
        // assert_eq!(
        //     package_data,
        //     hex!(
        //         "00000000000000000000000000000000088b3d4342774649325f313964a39e55
        //          ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3
        //          4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
        //     )
        // );
    }
}
