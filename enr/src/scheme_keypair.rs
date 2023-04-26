// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Scheme;

/// Wraps a private key and its associated public key.
pub struct SchemeKeyPair<S: Scheme> {
    private_key: S::PrivateKey,
    public_key: S::PublicKey,
}

impl<S: Scheme> SchemeKeyPair<S> {
    /// Creates a `SchemeKeyPair` from `private_key`.
    pub fn from_private_key(private_key: S::PrivateKey) -> Self {
        let public_key = S::new_public_key_from_private_key(&private_key);
        SchemeKeyPair {
            private_key,
            public_key,
        }
    }

    /// Returns a reference to the private key.
    pub fn private_key(&self) -> &S::PrivateKey {
        &self.private_key
    }

    /// Returns a reference to the public key.
    pub fn public_key(&self) -> &S::PublicKey {
        &self.public_key
    }
}
