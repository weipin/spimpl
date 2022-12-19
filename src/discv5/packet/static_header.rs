// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::flag::Flag;
use crate::discv5::auth_data::core::{AuthDataSize, AuthDataSource};
use crate::discv5::message;

pub(crate) struct StaticHeaderData;

pub(crate) const STATIC_HEADER_DATA_BYTE_LENGTH: usize = PROTOCOL_ID.len()
    + VERSION.len()
    + std::mem::size_of::<Flag>()
    + std::mem::size_of::<message::Nonce>()
    + std::mem::size_of::<AuthDataSize>();

// static-header = protocol-id || version || flag || nonce || authdata-size
impl StaticHeaderData {
    #[inline]
    pub(crate) fn append_data_to_buffer(
        buffer: &mut Vec<u8>,
        flag: Flag,
        nonce: &message::Nonce,
        size: AuthDataSize,
    ) {
        buffer.extend_from_slice(PROTOCOL_ID);
        buffer.extend_from_slice(&VERSION);
        buffer.push(flag as u8);
        buffer.extend(nonce.0);
        buffer.extend(size.to_be_bytes());
    }
}

// protocol-id   = "discv5"
const PROTOCOL_ID: &[u8] = b"discv5";

// version       = 0x0001
const VERSION: [u8; 2] = [0, 1];
