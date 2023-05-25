// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::{NodeIdType, Scheme, SequenceNumber};

use crate::types::NonceType;

use super::flag::Flag;
use super::types::{AuthDataSize, IdNonceType};

pub(crate) const ORDINARY_MESSAGE_AUTHDATA_SIZE: AuthDataSize =
    size_of::<NodeIdType>() as AuthDataSize;
pub(crate) const ORDINARY_MESSAGE_AUTHDATA_SIZE_BYTES: &[u8; 2] =
    &ORDINARY_MESSAGE_AUTHDATA_SIZE.to_be_bytes();

pub(crate) const WHOAREYOU_AUTHDATA_SIZE: AuthDataSize =
    (size_of::<IdNonceType>() + size_of::<SequenceNumber>()) as AuthDataSize;
pub(crate) const WHOAREYOU_AUTHDATA_SIZE_BYTES: &[u8; 2] = &WHOAREYOU_AUTHDATA_SIZE.to_be_bytes();

pub(crate) const STATIC_HEADER_BYTE_LENGTH: usize = PROTOCOL_ID.len()
    + VERSION.len()
    + size_of::<Flag>()
    + size_of::<NonceType>()
    + size_of::<AuthDataSize>();

pub(crate) const fn size_of_handshake_message_authdata_fixed_part<S: Scheme>() -> AuthDataSize {
    // authdata      = authdata-head || id-signature || eph-pubkey || record
    // authdata-head = src-id || sig-size || eph-key-size
    let size = size_of::<NodeIdType>()
        + size_of::<u8>()
        + size_of::<u8>()
        + S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH
        + S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH;
    debug_assert!(size <= AuthDataSize::MAX as usize);
    size as AuthDataSize
}

pub(crate) const PROTOCOL_ID: &[u8] = b"discv5";

pub(crate) const VERSION: [u8; 2] = [0, 1];

// The maximum size of any packet is 1280 bytes
pub(crate) const MAX_PACKET_BYTE_LENGTH: usize = 1280;

// The minimum size of any packet is 63 bytes
pub(crate) const MIN_PACKET_BYTE_LENGTH: usize = 63;

#[cfg(test)]
mod tests {
    use enr::Schemev4;

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(ORDINARY_MESSAGE_AUTHDATA_SIZE, 32);
        assert_eq!(WHOAREYOU_AUTHDATA_SIZE, 24);

        // 34 + 64 + 33 = 131
        assert_eq!(
            size_of_handshake_message_authdata_fixed_part::<Schemev4>(),
            131
        );
    }
}
