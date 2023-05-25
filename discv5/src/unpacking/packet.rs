// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::NodeId;

use crate::packet::constants::{
    MAX_PACKET_BYTE_LENGTH, MIN_PACKET_BYTE_LENGTH, STATIC_HEADER_BYTE_LENGTH,
};
use crate::packet::types::StaticHeader;
use crate::packet::{aesctr, Flag, MaskingIv, MaskingIvType};
use crate::types::Nonce;

use super::error::Error;
use super::static_header::unpack_static_header;

// (masking-iv, flag, nonce, static_header, auth_data, encrypted_message_data)
#[allow(clippy::type_complexity)]
pub fn unpack<'a, 'b>(
    node_id: &NodeId,
    bytes: &'a [u8],
) -> Result<
    (
        MaskingIv<'a>,
        Flag,
        Nonce<'b>,
        StaticHeader,
        Vec<u8>,
        &'a [u8],
    ),
    Error,
> {
    if bytes.len() < MIN_PACKET_BYTE_LENGTH {
        return Err(Error::PacketTooSmall);
    }
    if bytes.len() > MAX_PACKET_BYTE_LENGTH {
        return Err(Error::PacketTooLarge);
    }

    let (masking_iv_slice, remaining) = bytes.split_at(size_of::<MaskingIvType>());
    let masking_iv = MaskingIv::from_slice(masking_iv_slice.try_into().unwrap());

    let mut cipher = aesctr::new_cipher(
        &node_id.bytes()[..16].try_into().unwrap(),
        masking_iv_slice.try_into().unwrap(),
    );

    let (masked_static_header_slice, remaining) = remaining.split_at(STATIC_HEADER_BYTE_LENGTH);
    let mut static_header = [0; STATIC_HEADER_BYTE_LENGTH];
    static_header.copy_from_slice(masked_static_header_slice);
    aesctr::apply_keystream(&mut cipher, &mut static_header);

    let (flag, nonce, auth_data_size) = unpack_static_header(&static_header)?;

    if remaining.len() < auth_data_size as usize {
        return Err(Error::InvalidAuthDataBytes);
    }
    let (auth_data_slice, message) = remaining.split_at(auth_data_size as usize);

    let mut auth_data = auth_data_slice.to_vec();
    aesctr::apply_keystream(&mut cipher, auth_data.as_mut_slice());

    let nonce = Nonce::from_array(*nonce.bytes());
    Ok((masking_iv, flag, nonce, static_header, auth_data, message))
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_unpack_ping_message_packet() {
        let node_id_data = hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let node_id = NodeId::from_slice(&node_id_data);
        let packet_data = hex!(
            "00000000000000000000000000000000088b3d4342774649325f313964a39e55"
            "ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
            "4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
        );

        let (_, _, _, _, auth_data, encrypted_message_data) =
            unpack(&node_id, &packet_data).unwrap();
        assert_eq!(
            hex::encode(&auth_data),
            "aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb"
        );
        assert_eq!(
            hex::encode(encrypted_message_data),
            "b84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
        );
    }
}
