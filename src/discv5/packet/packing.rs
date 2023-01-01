// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::crypto::aesctr;
use crate::discv5::message;
use crate::discv5::message::protocol::core::Message;
use crate::discv5::packet::masked_header::{mask_header_data, MaskingIv};
use crate::enr::NodeId;

// packet        = masking-iv || masked-header || message
// masked-header = aesctr_encrypt(masking-key, masking-iv, header)
// masking-key   = dest-id[:16]
// masking-iv    = uint128   -- random data unique to packet
pub(crate) fn pack_message<M: Message>(
    message: &M,
    header_data_pt_in_ct_out: &mut [u8],
    dest_id: &NodeId,
    nonce: &message::Nonce,
    masking_iv: &MaskingIv,
    initiator_key: &[u8],
) -> Vec<u8> {
    let encrypted_message_data = message::encrypt(
        message,
        header_data_pt_in_ct_out,
        nonce,
        masking_iv,
        initiator_key,
    );

    println!(
        ">> encrypted_message_data: {}",
        hex::encode(&encrypted_message_data)
    );

    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&masking_iv.0);

    mask_header_data(dest_id, masking_iv, header_data_pt_in_ct_out);
    packet_data.extend_from_slice(header_data_pt_in_ct_out);
    packet_data.extend(encrypted_message_data);

    packet_data
}
