// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::packet::masked_header::{mask_header_data, MaskingIv};
use crate::enr::NodeId;

pub(crate) fn pack_whoareyou(
    header_data_pt_in_ct_out: &mut [u8],
    dest_id: &NodeId,
    masking_iv: &MaskingIv,
) -> Vec<u8> {
    // For WHOAREYOU packets, the message is empty.
    let mut packet_data = Vec::new();
    packet_data.extend_from_slice(&masking_iv.0);

    mask_header_data(dest_id, masking_iv, header_data_pt_in_ct_out);
    packet_data.extend_from_slice(header_data_pt_in_ct_out);

    packet_data
}
