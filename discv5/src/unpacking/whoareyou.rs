// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::SequenceNumber;

use crate::packet::constants::{STATIC_HEADER_BYTE_LENGTH, WHOAREYOU_AUTHDATA_SIZE};
use crate::packet::IdNonce;

use super::Error;

pub fn unpack(static_header: &[u8], auth_data: &[u8]) -> Result<(IdNonce, SequenceNumber), Error> {
    debug_assert_eq!(static_header.len(), STATIC_HEADER_BYTE_LENGTH);
    if auth_data.len() != WHOAREYOU_AUTHDATA_SIZE as usize {
        return Err(Error::InvalidAuthDataSize);
    }

    let (id_nonce_slice, enr_seq_slice) = auth_data.split_at(size_of::<IdNonce>());
    let id_nonce = IdNonce(id_nonce_slice.try_into().unwrap());
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
        let dest_node_id = NodeId(dest_node_id_data);
        let package_data = hex!(
            "00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad"
            "1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d"
        );

        let (_, flag, _, static_header, auth_data, _) =
            unpack(&dest_node_id, &package_data).unwrap();
        assert_eq!(flag, Flag::Whoareyou);

        let (id_nonce, enr_seq) = unpack_whoareyou(&static_header, &auth_data).unwrap();
        assert_eq!(id_nonce.0, hex!("0102030405060708090a0b0c0d0e0f10"));
        assert_eq!(enr_seq, 0);
    }
}
