// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use sha3::{Digest, Keccak256};

use crate::content::ContentRlpEncoded;
use crate::Scheme;

impl ContentRlpEncoded {
    /// Creates a `S::Signature` for a `ContentRlpEncoded` using `private_key`.
    pub(crate) fn sign<S: Scheme>(
        &self,
        private_key: &S::PrivateKey,
    ) -> Result<S::Signature, S::Error> {
        let hash = Keccak256::digest(&self.0);
        S::sign(&hash, private_key)
    }

    /// Verifies if `signature` is valid with a `ContentRlpEncoded` using
    /// `public_key`.
    ///
    /// Returns `true` if the signature is valid.
    pub(crate) fn verify<S: Scheme>(
        &self,
        signature: &S::Signature,
        public_key: &S::PublicKey,
    ) -> Result<bool, S::Error> {
        let hash = Keccak256::digest(&self.0);
        S::verify(&hash, signature, public_key)
    }
}
