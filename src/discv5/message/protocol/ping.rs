// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::core::Message;
use crate::discv5::auth_data::core::{AuthDataSize, FixedSizeAuthDataSource};
use crate::discv5::message::RequestId;
use crate::enr;
use bytes::BytesMut;
use fastrlp::{Encodable, RlpEncodable};

#[derive(Debug, RlpEncodable)]
pub(crate) struct Ping {
    pub(crate) request_id: RequestId,
    pub(crate) enr_seq: enr::SequenceNumber,
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
    use crate::discv5::auth_data::handshake::HandshakeInitialMessageAuthData;
    use crate::discv5::crypto::keys_deriving::{
        derive_keys, KDF_INFO_BYTE_LENGTH, KDF_INFO_PREFIX, KDF_INFO_PREFIX_BYTE_LENGTH,
    };
    use crate::discv5::packet::flag::Flag;
    use crate::discv5::packet::header::{
        encode_fixed_size_header_to_buffer, fixed_size_encoded_header_byte_length,
    };
    use crate::discv5::packet::masked_header::{mask_header_data, MaskingIv};
    use crate::discv5::packet::packing::pack_message;
    use crate::discv5::{device, message};
    use crate::enr::{NodeId, Scheme, Schemev4};
    use hex_literal::hex;
    use secp256k1::SECP256K1;

    #[test]
    fn test_packaging_ping() {
        let src_node_id_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_node_id = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let ping_req_id_data = hex!("00000001");
        let ping_enr_seq = 2;

        // let initiator_key = hex!("4f9fac6de7567d1e3b1241dffe90f662");
        let initiator_key = hex!("00000000000000000000000000000000");

        let nonce = message::Nonce(nonce_data);
        let ping = Ping {
            request_id: RequestId(ping_req_id_data.to_vec()),
            enr_seq: ping_enr_seq,
        };

        let mut header_data = Vec::with_capacity(fixed_size_encoded_header_byte_length::<
            HandshakeInitialMessageAuthData<Schemev4>,
        >());
        let auth_data_source = device::context::Context {
            node_id: NodeId(src_node_id_data),
        };
        encode_fixed_size_header_to_buffer(
            &mut header_data,
            &auth_data_source,
            Flag::Ordinary,
            &nonce,
        );

        let package_data = pack_message(
            &ping,
            &mut header_data,
            &NodeId(dest_node_id),
            &nonce,
            &MaskingIv(hex!("00000000000000000000000000000000")),
            &initiator_key,
        );

        assert_eq!(
            hex::encode(package_data),
            "00000000000000000000000000000000088b3d4342774649325f313964a39e55ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d34c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
        );
    }
}
