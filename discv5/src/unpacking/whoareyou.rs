// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::SequenceNumber;

use crate::packet::constants::WHOAREYOU_AUTHDATA_SIZE;
use crate::packet::types::IdNonceType;
use crate::packet::IdNonce;

use super::Error;

pub fn unpack<'a>(
    auth_data: &'a [u8],
    encrypted_message_data: &[u8],
) -> Result<(IdNonce<'a>, SequenceNumber), Error> {
    if !encrypted_message_data.is_empty() {
        return Err(Error::InvalidMessageByteLength);
    }

    if auth_data.len() != WHOAREYOU_AUTHDATA_SIZE as usize {
        return Err(Error::InvalidAuthDataSize);
    }

    let (id_nonce_slice, enr_seq_slice) = auth_data.split_at(size_of::<IdNonceType>());
    let id_nonce = IdNonce::from_slice(id_nonce_slice.try_into().unwrap());
    let enr_seq = SequenceNumber::from_be_bytes(enr_seq_slice.try_into().unwrap());
    Ok((id_nonce, enr_seq))
}

#[cfg(test)]
mod tests {
    use enr::NodeId;
    use hex_literal::hex;

    use crate::packet::Flag;
    use crate::unpacking::unpack;

    use super::unpack as unpack_whoareyou;

    #[test]
    fn test_unpack_whoareyou() {
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let dest_node_id = NodeId::from_slice(&dest_node_id_data);
        let packet_data = hex!(
            "00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad"
            "1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d"
        );

        let (_, flag, _, _, auth_data, _) = unpack(&dest_node_id, &packet_data).unwrap();
        assert_eq!(flag, Flag::Whoareyou);

        let (id_nonce, enr_seq) = unpack_whoareyou(&auth_data, &[]).unwrap();
        assert_eq!(id_nonce.bytes(), &hex!("0102030405060708090a0b0c0d0e0f10"));
        assert_eq!(enr_seq, 0);
    }
}
