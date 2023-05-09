// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use crate::packet::constants::{PROTOCOL_ID, STATIC_HEADER_BYTE_LENGTH, VERSION};
use crate::packet::types::AuthDataSize;
use crate::packet::Flag;
use crate::types::Nonce;

use super::error::Error;

pub(crate) fn unpack_static_header(bytes: &[u8]) -> Result<(Flag, Nonce, AuthDataSize), Error> {
    debug_assert_eq!(bytes.len(), STATIC_HEADER_BYTE_LENGTH);

    let (protocol_id_slice, remaining) = bytes.split_at(PROTOCOL_ID.len());
    if protocol_id_slice != PROTOCOL_ID {
        return Err(Error::InvalidProtocolId);
    }

    let (version_slice, remaining) = remaining.split_at(2);
    let version: [u8; 2] = version_slice.try_into().unwrap();
    if version != VERSION {
        return Err(Error::InvalidVersion);
    }

    let (flag_slice, remaining) = remaining.split_at(1);
    let flag = Flag::from_u8(*flag_slice.first().unwrap()).ok_or(Error::InvalidFlag)?;

    let (nonce_slice, remaining) = remaining.split_at(size_of::<Nonce>());
    let nonce = Nonce::from_bytes(nonce_slice.try_into().unwrap());

    if remaining.len() != size_of::<AuthDataSize>() {
        return Err(Error::InvalidAuthDataSize);
    }

    let auth_data_size = AuthDataSize::from_be_bytes(remaining.try_into().unwrap());
    Ok((flag, nonce, auth_data_size))
}
