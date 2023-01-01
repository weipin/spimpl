// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::flag::Flag;
use super::static_header::StaticHeaderData;
use crate::discv5::auth_data::core::{
    AuthDataSize, FixedSizeAuthDataSource, VariableSizeAuthDataSource,
};
use crate::discv5::message;
use crate::discv5::packet::static_header::STATIC_HEADER_DATA_BYTE_LENGTH;

// header        = static-header || authdata
pub(crate) fn fixed_size_encoded_header_byte_length<A: FixedSizeAuthDataSource>() -> usize {
    STATIC_HEADER_DATA_BYTE_LENGTH + A::SIZE as usize
}

pub(crate) fn encode_fixed_size_header_to_buffer<A: FixedSizeAuthDataSource>(
    buffer: &mut Vec<u8>,
    auth_data_source: &A,
    flag: Flag,
    nonce: &message::Nonce,
) {
    StaticHeaderData::append_data_to_buffer(buffer, flag, nonce, A::SIZE);
    auth_data_source.append_data_to_buffer(buffer);
}

// let mut header_data = Vec::with_capacity(encoded_header_byte_length::<A>());
// encode_header_to_buffer(auth_data, flag, &nonce, &mut header_data);

pub(crate) fn encode_variable_size_header_to_buffer<A: VariableSizeAuthDataSource>(
    buffer: &mut Vec<u8>,
    auth_data_source: &A,
    flag: Flag,
    nonce: &message::Nonce,
) {
    StaticHeaderData::append_data_to_buffer(buffer, flag, nonce, auth_data_source.size());
    auth_data_source.append_data_to_buffer(buffer);
}
