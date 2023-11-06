// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::{NodeId, SeqNum};

use crate::packet::constants::{
    MIN_PACKET_BYTE_LENGTH, PROTOCOL_ID, VERSION, WHOAREYOU_AUTHDATA_SIZE_BYTES,
};
use crate::packet::flag::Flag;
use crate::packet::types::{IdNonce, MaskingIv};
use crate::types::Nonce;

use super::common::pack_header;

pub fn pack(
    nonce: &Nonce,
    dest_node_id: &NodeId,
    masking_iv: &MaskingIv,
    id_nonce: &IdNonce,
    enr_seq: SeqNum,
) -> Vec<u8> {
    let mut header_pt_in_ct_out = build_header(nonce, id_nonce, enr_seq);

    let mut output = vec![];
    pack_header(
        masking_iv,
        dest_node_id,
        header_pt_in_ct_out.as_mut(),
        &mut output,
    );

    debug_assert_eq!(output.len(), MIN_PACKET_BYTE_LENGTH);
    output
}

fn build_header(nonce: &Nonce, id_nonce: &IdNonce, enr_seq: SeqNum) -> Vec<u8> {
    let mut output = vec![];

    output.extend(PROTOCOL_ID);
    output.extend(VERSION);
    output.push(Flag::Whoareyou.value());
    output.extend(nonce.bytes());
    output.extend(WHOAREYOU_AUTHDATA_SIZE_BYTES);
    output.extend(id_nonce.bytes());
    output.extend(enr_seq.to_be_bytes());

    output
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_packing() {
        let nonce_data = hex!("0102030405060708090a0b0c");
        let id_nonce_data = hex!("0102030405060708090a0b0c0d0e0f10");
        let enr_seq = 0;
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let masking_iv_data = hex!("00000000000000000000000000000000");

        let nonce = Nonce::from_array(nonce_data);
        let id_nonce = IdNonce::from_slice(&id_nonce_data);
        let dest_node_id = NodeId::from_slice(&dest_node_id_data);
        let masking_iv = MaskingIv::from_slice(&masking_iv_data);

        let packed = pack(&nonce, &dest_node_id, &masking_iv, &id_nonce, enr_seq);
        // Whoareyou has the mininmum packet size.
        assert_eq!(packed.len(), MIN_PACKET_BYTE_LENGTH);
        assert_eq!(
            packed,
            hex!(
                "00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad"
                "1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d"
            )
        );
    }
}
