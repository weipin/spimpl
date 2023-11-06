// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rand::{CryptoRng, Rng};

// nonce         = uint96    -- nonce of message
pub type NonceType = [u8; 12];

#[derive(Debug, PartialEq)]
pub struct Nonce(NonceType);

impl Nonce {
    pub fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        Nonce(csprng.gen())
    }

    pub const fn from_array(array: NonceType) -> Self {
        Nonce(array)
    }

    pub fn bytes(&self) -> &[u8; 12] {
        &self.0
    }
}
