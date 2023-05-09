// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rand::{CryptoRng, Rng};

// nonce         = uint96    -- nonce of message
#[derive(Debug, PartialEq)]
pub struct Nonce([u8; 12]);

impl Nonce {
    pub fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        Nonce(csprng.gen())
    }

    pub(crate) fn from_bytes(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    pub fn bytes(&self) -> &[u8; 12] {
        &self.0
    }
}
