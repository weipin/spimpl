// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::NodeId;

use crate::messages::Message;
use crate::packet::constants::{ORDINARY_MESSAGE_AUTHDATA_SIZE_BYTES, PROTOCOL_ID, VERSION};
use crate::packet::flag::Flag;
use crate::packet::types::MaskingIv;
use crate::types::Nonce;

use super::common::{pack_header, pack_message};
use super::error::Error;

pub fn pack<'a, T: Message<'a>>(
    message: &T,
    nonce: &Nonce,
    src_node_id: &NodeId,
    dest_node_id: &NodeId,
    masking_iv: &MaskingIv,
    initiator_key: &[u8; 16],
) -> Result<Vec<u8>, Error> {
    let header = build_header(nonce, src_node_id);
    let mut header_pt_in_ct_out = header.clone();
    let mut output = vec![];
    pack_header(
        masking_iv,
        dest_node_id,
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

fn build_header(nonce: &Nonce, src_node_id: &NodeId) -> Vec<u8> {
    let mut output = vec![];

    output.extend(PROTOCOL_ID);
    output.extend(VERSION);
    output.push(Flag::OrdinaryMessage.value());
    output.extend(nonce.bytes());
    output.extend(ORDINARY_MESSAGE_AUTHDATA_SIZE_BYTES);
    output.extend(src_node_id.bytes());

    output
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use enr::NodeIdType;
    use hex_literal::hex;

    use crate::messages::{self, Ping, Type};
    use crate::packet::constants::{MAX_PACKET_BYTE_LENGTH, STATIC_HEADER_BYTE_LENGTH};
    use crate::packet::{aesgcm, MaskingIvType};
    use crate::types::RequestId;

    use super::*;

    #[test]
    fn test_packing() {
        let src_node_id_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let initiator_key = hex!("00000000000000000000000000000000");
        let masking_iv_data = hex!("00000000000000000000000000000000");
        let request_id_data = hex!("00000001");
        let enr_seq = 2;

        let src_node_id = NodeId::from_slice(&src_node_id_data);
        let dest_node_id = NodeId::from_slice(&dest_node_id_data);
        let nonce = Nonce::from_array(nonce_data);
        let masking_iv = MaskingIv::from_slice(&masking_iv_data);
        let request_id = RequestId::from_slice(&request_id_data).unwrap();
        let ping = Ping {
            request_id,
            enr_seq,
        };

        let packed = pack(
            &ping,
            &nonce,
            &src_node_id,
            &dest_node_id,
            &masking_iv,
            &initiator_key,
        )
        .unwrap();
        assert_eq!(
            hex::encode(&packed),
            concat!(
                "00000000000000000000000000000000088b3d4342774649325f313964a39e55",
                "ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3",
                "4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
            ),
        );
    }

    #[test]
    fn test_max_packet_byte_length() {
        const MAX_ENCODED_MESSAGE_BYTE_LENGTH: usize = MAX_PACKET_BYTE_LENGTH
            - size_of::<MaskingIvType>()
            - STATIC_HEADER_BYTE_LENGTH
            - size_of::<NodeIdType>()
            - aesgcm::TAG_BYTE_LENGTH;
        const CONTENT_BYTE_LEN: usize = 1186;
        let echo = Echo {
            content: vec![6; CONTENT_BYTE_LEN],
        };
        let encoded = messages::encode(&echo);
        assert_eq!(encoded.len(), MAX_ENCODED_MESSAGE_BYTE_LENGTH);

        let src_node_id_data =
            hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let dest_node_id_data =
            hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let nonce_data = hex!("ffffffffffffffffffffffff");
        let initiator_key = hex!("00000000000000000000000000000000");
        let masking_iv_data = hex!("00000000000000000000000000000000");

        let src_node_id = NodeId::from_slice(&src_node_id_data);
        let dest_node_id = NodeId::from_slice(&dest_node_id_data);
        let nonce = Nonce::from_array(nonce_data);
        let masking_iv = MaskingIv::from_slice(&masking_iv_data);

        let packed = pack(
            &echo,
            &nonce,
            &src_node_id,
            &dest_node_id,
            &masking_iv,
            &initiator_key,
        )
        .unwrap();
        assert_eq!(packed.len(), MAX_PACKET_BYTE_LENGTH);

        let echo = Echo {
            content: vec![6; CONTENT_BYTE_LEN + 1],
        };
        assert_eq!(
            pack(
                &echo,
                &nonce,
                &src_node_id,
                &dest_node_id,
                &masking_iv,
                &initiator_key,
            )
            .unwrap_err(),
            Error::PacketTooLarge
        );
    }

    // A dummy Message type with arbitrary content for packet byte length testing.
    #[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
    struct Echo {
        content: Vec<u8>,
    }

    impl<'a> Message<'a> for Echo {
        const TYPE: Type = Type::Ping;

        const MIN_DATA_BYTE_LENGTH: usize = 0;
    }
}
