// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::NodeId;

use crate::messages::{self, Message};
use crate::packet::constants::{MAX_PACKET_BYTE_LENGTH, MIN_PACKET_BYTE_LENGTH};
use crate::packet::{aesctr, aesgcm, MaskingIv};
use crate::types::Nonce;

use super::error::Error;

pub(crate) fn pack_header(
    masking_iv: &MaskingIv,
    dest_id: &NodeId,
    header_in_out: &mut [u8],
    output: &mut Vec<u8>,
) {
    aesctr::encrypt(
        dest_id.0[..16].try_into().unwrap(),
        masking_iv.bytes(),
        header_in_out,
    );
    output.extend(masking_iv.bytes());
    output.extend(header_in_out.as_ref());
}

pub(crate) fn pack_message<T: Message>(
    initiator_key: &[u8; 16],
    nonce: &Nonce,
    masking_iv: &MaskingIv,
    header: &[u8],
    message: &T,
    output: &mut Vec<u8>,
) -> Result<(), Error> {
    let mut message_pt_in_ct_out = messages::encode(message);

    let mut message_ad = masking_iv.bytes().to_vec();
    message_ad.extend(header);

    aesgcm::encrypt(
        initiator_key,
        nonce.bytes(),
        &message_ad,
        &mut message_pt_in_ct_out,
    );

    output.extend(message_pt_in_ct_out);

    // See whoareyou.rs for a test of "mininmum packet size".
    debug_assert!(output.len() >= MIN_PACKET_BYTE_LENGTH);

    if output.len() > MAX_PACKET_BYTE_LENGTH {
        return Err(Error::PacketTooLarge);
    }

    Ok(())
}
