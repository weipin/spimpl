// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rand::{CryptoRng, Rng};

pub struct MaskingIv([u8; 16]);

impl MaskingIv {
    pub fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        MaskingIv(csprng.gen())
    }

    pub(crate) fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub(crate) fn bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

pub(crate) type AuthDataSize = u16;

#[derive(Debug)]
pub struct IdNonce(pub [u8; 16]);
