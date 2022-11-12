// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::storage_content_with_signature_rlp::MAXIMUM_BASE64_ENCODED_BYTE_LENGTH;
use super::storage_content_with_signature_rlp::MAXIMUM_RLP_ENCODED_BYTE_LENGTH;

pub(crate) fn maximum_base64_encoded_output() -> Vec<u8> {
    vec![0; MAXIMUM_BASE64_ENCODED_BYTE_LENGTH]
}

pub(crate) fn maximum_rlp_encoded_output() -> Vec<u8> {
    vec![0; MAXIMUM_RLP_ENCODED_BYTE_LENGTH]
}
