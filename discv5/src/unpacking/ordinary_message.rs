// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::NodeId;

use crate::messages::Message;
use crate::packet::constants::ORDINARY_MESSAGE_AUTHDATA_SIZE;
use crate::packet::types::StaticHeader;
use crate::packet::{aesgcm, MaskingIv};
use crate::types::Nonce;

use super::error::Error;

pub fn unpack<'a, M: Message<'a>>(
    masking_iv: &MaskingIv,
    read_key: &[u8; 16],
    nonce: &Nonce,
    static_header: &StaticHeader,
    auth_data: &[u8],
    encrypted_message_data: &'a [u8],
) -> Result<(NodeId<'static>, Vec<u8>), Error> {
    if auth_data.len() != ORDINARY_MESSAGE_AUTHDATA_SIZE as usize {
        return Err(Error::InvalidAuthDataSize);
    }

    let mut ad = vec![];
    ad.extend(masking_iv.bytes());
    ad.extend(static_header);
    ad.extend(auth_data);

    if encrypted_message_data.len() < aesgcm::ct_byte_length(M::MIN_DATA_BYTE_LENGTH) {
        return Err(Error::InvalidMessageByteLength);
    }
    let mut message_data = encrypted_message_data.to_vec();
    if !aesgcm::decrypt(read_key, nonce.bytes(), &ad, &mut message_data) {
        return Err(Error::MessageDecryptingFailed);
    }

    if message_data.is_empty() {
        return Err(Error::InvalidMessageByteLength);
    }

    let node_id = NodeId::from_array(auth_data.try_into().unwrap());
    Ok((node_id, message_data))
}

#[cfg(test)]
mod tests {
    use enr::NodeId;
    use hex_literal::hex;

    use crate::messages::{decode_ping, decode_type, Ping, Type};
    use crate::packet::Flag;
    use crate::types::RequestId;
    use crate::unpacking::unpack;

    use super::unpack as unpack_ordinary_message;

    #[test]
    fn test_unpack_ping_message_packet() {
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let dest_node_id = NodeId::from_slice(&dest_node_id_data);
        let read_key = hex!("00000000000000000000000000000000");
        let ping = Ping {
            request_id: RequestId::from_slice(&hex!("00000001")).unwrap(),
            enr_seq: 2,
        };
        let packet_data = hex!(
            "00000000000000000000000000000000088b3d4342774649325f313964a39e55"
            "ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
            "4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
        );

        let (masking_iv, flag, nonce, static_header, auth_data, encrypted_message_data) =
            unpack(&dest_node_id, &packet_data).unwrap();
        assert_eq!(flag, Flag::OrdinaryMessage);

        let (src_node_id, message_data) = unpack_ordinary_message::<Ping>(
            &masking_iv,
            &read_key,
            &nonce,
            &static_header,
            &auth_data,
            encrypted_message_data,
        )
        .unwrap();
        assert_eq!(
            src_node_id.bytes(),
            &hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb")
        );

        let (message_type, message_rlp_encoded) = decode_type(&message_data).unwrap();
        assert_eq!(message_type, Type::Ping);
        let decoded_ping = decode_ping(message_rlp_encoded).unwrap();
        assert_eq!(decoded_ping, ping);
    }
}
