// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;

use rand::{CryptoRng, Rng};

use super::constants::STATIC_HEADER_BYTE_LENGTH;

pub type StaticHeader = [u8; STATIC_HEADER_BYTE_LENGTH];

pub type MaskingIvType = [u8; 16];

#[derive(Debug)]
pub struct MaskingIv<'a>(Cow<'a, MaskingIvType>);

impl<'a> MaskingIv<'a> {
    pub fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        MaskingIv(Cow::Owned(csprng.gen()))
    }

    pub fn from_slice(slice: &'a MaskingIvType) -> Self {
        MaskingIv(Cow::Borrowed(slice))
    }

    pub fn from_array(array: MaskingIvType) -> Self {
        MaskingIv(Cow::Owned(array))
    }

    pub fn bytes(&self) -> &MaskingIvType {
        &self.0
    }
}

pub(crate) type AuthDataSize = u16;
pub type AuthData = Vec<u8>;

// id-nonce      = uint128   -- random bytes
pub type IdNonceType = [u8; 16];

#[derive(Debug)]
pub struct IdNonce<'a>(Cow<'a, IdNonceType>);

impl<'a> IdNonce<'a> {
    pub fn from_slice(slice: &'a IdNonceType) -> Self {
        IdNonce(Cow::Borrowed(slice))
    }

    pub fn from_array(array: IdNonceType) -> Self {
        IdNonce(Cow::Owned(array))
    }

    pub fn bytes(&self) -> &IdNonceType {
        &self.0
    }
}
