// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::FixedSizeAuthDataSource;
use crate::discv5::crypto::aesgcm;
use crate::discv5::message;
use crate::discv5::message::protocol::core::Message;
use crate::discv5::packet::flag::Flag;
use crate::discv5::packet::header::{
    encode_fixed_size_header_to_buffer, fixed_size_encoded_header_byte_length,
};
use crate::discv5::packet::masked_header::MaskingIv;

// message       = aesgcm_encrypt(initiator-key, nonce, message-pt, message-ad)
// message-pt    = message-type || message-data
// message-ad    = masking-iv || header
pub(crate) fn encrypt<M: Message>(
    message: &M,
    header_data: &[u8],
    nonce: &message::Nonce,
    masking_iv: &MaskingIv,
    initiator_key: &[u8],
) -> Vec<u8> {
    let mut message_pt_data = Vec::new();
    message_pt_data.push(M::TYPE);
    message.encode_to_data(&mut message_pt_data);

    let mut message_ad_data = Vec::new();
    message_ad_data.extend_from_slice(&masking_iv.0);
    message_ad_data.extend_from_slice(header_data);

    aesgcm::encrypt(
        initiator_key,
        &nonce.0,
        &message_ad_data,
        &mut message_pt_data,
    );
    message_pt_data
}
