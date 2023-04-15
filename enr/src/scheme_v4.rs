// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Shared code for implementations of scheme v4.

use crate::Schemev4Secp256k1;

/// Default implementations of scheme v4.
///
/// Uses rust-secp256k1 by default for its performance.
/// See "benches/record_from_address" for details.
pub type Schemev4 = Schemev4Secp256k1;

// Name of the identity scheme v4
pub(crate) const SCHEME_V4_ID: &[u8] = b"v4";

// Compressed secp256k1 public key, 33 bytes
pub(crate) const ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH: usize = 33;

// Signature encoded as the concatenation of the r and s signature values (the
// recovery ID v is omitted), 64 bytes.
pub(crate) const ENR_REQUIRED_SIGNATURE_BYTE_LENGTH: usize = 64;

// Secret through elliptic-curve Diffie-Hellman key agreement, compressed,
// 33 bytes
pub(crate) const DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH: usize = 33;

// Testing only, replaces `OsRng` for deterministic output.
#[cfg(test)]
pub(crate) struct MockOsRng;

#[cfg(test)]
impl rand::CryptoRng for MockOsRng {}

#[cfg(test)]
impl rand::RngCore for MockOsRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!();
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!();
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        debug_assert_eq!(dest.len(), MOCK_ECDSA_NONCE_ADDITIONAL_DATA.len());
        dest.copy_from_slice(&MOCK_ECDSA_NONCE_ADDITIONAL_DATA);
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
        unimplemented!();
    }
}

#[cfg(test)]
pub(crate) const MOCK_ECDSA_NONCE_ADDITIONAL_DATA: [u8; 32] = [
    // 0xbaaaaaad...
    0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad,
    0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad,
];
