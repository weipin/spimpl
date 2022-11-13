// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements the struct representing content RLP.

use super::scheme::Scheme;
use super::storage::Storage;
use super::storage_rlp_decoding::RlpDecodingError;
use sha3::{Digest, Keccak256};

#[derive(PartialEq)]
pub(crate) struct StorageContentRlp(pub(crate) Vec<u8>);

impl Storage {
    pub(crate) fn update_public_key_and_encode_content_to_rlp<S: Scheme>(
        &mut self,
        public_key: &S::PublicKey,
    ) -> StorageContentRlp {
        debug_assert!(self.id.is_none() || self.id.unwrap() == S::id());
        self.id = Some(S::id());

        self.public_key_value = Some(S::public_key_to_value(public_key));

        self.encode_content_to_rlp::<S>()
    }

    pub(crate) fn encode_content_to_rlp<S: Scheme>(&self) -> StorageContentRlp {
        debug_assert!(self.id.is_some());
        debug_assert!(self.public_key_value.is_some());

        StorageContentRlp(self.to_rlp::<S>(false))
    }
}

impl StorageContentRlp {
    /// Signs the content RLP.
    pub(crate) fn sign<S: Scheme>(
        &self,
        private_key: &S::PrivateKey,
    ) -> Result<S::Signature, S::SigningError> {
        let hash = Keccak256::digest(&self.0);
        S::sign(&hash, private_key)
    }
}

impl StorageContentRlp {
    /// Verifies the content RLP.
    pub(crate) fn verify<S: Scheme>(
        &self,
        signature_value: &[u8],
        public_key_value: &[u8],
    ) -> Result<bool, RlpDecodingError> {
        let signature = S::value_to_signature(signature_value)
            .ok_or(RlpDecodingError::InvalidSignatureValue)
            .unwrap();
        let public_key = S::value_to_public_key(public_key_value)
            .ok_or(RlpDecodingError::InvalidPublicKeyValue)
            .unwrap();

        let hash = Keccak256::digest(&self.0);
        let verified = S::verify(&hash, &signature, &public_key)
            .map_err(|_| RlpDecodingError::VerifyingError)?;
        Ok(verified)
    }
}
