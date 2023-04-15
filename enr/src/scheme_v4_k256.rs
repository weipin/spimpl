// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [k256][1] implementation of scheme v4.
//!
//! [1]: https://github.com/RustCrypto/elliptic-curves/tree/master/k256

#[cfg(test)]
use crate::scheme_v4::MockOsRng as OsRng;
#[cfg(not(test))]
use rand::rngs::OsRng;

use k256::ecdsa;
use k256::ecdsa::signature::hazmat::{PrehashVerifier, RandomizedPrehashSigner};

use crate::predefined_keys::SCHEME_V4_KEY_OF_PUBLIC_KEY;
use crate::scheme_v4::{
    DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH, ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH,
    ENR_REQUIRED_SIGNATURE_BYTE_LENGTH, SCHEME_V4_ID,
};
use crate::{NodeId, Scheme};

/// k256 implementation of scheme v4.
pub struct Schemev4K256;

impl Scheme for Schemev4K256 {
    type PrivateKey = ecdsa::SigningKey;
    type PublicKey = ecdsa::VerifyingKey;
    type Signature = ecdsa::Signature;
    type Error = ecdsa::Error;

    const ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH: usize = ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH;
    const ENR_REQUIRED_SIGNATURE_BYTE_LENGTH: usize = ENR_REQUIRED_SIGNATURE_BYTE_LENGTH;
    const DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH: usize =
        DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH;

    fn id() -> &'static [u8] {
        SCHEME_V4_ID
    }

    fn key_of_public_key() -> &'static [u8] {
        SCHEME_V4_KEY_OF_PUBLIC_KEY
    }

    fn new_public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, Self::Error> {
        assert_eq!(bytes.len(), Self::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);
        ecdsa::VerifyingKey::from_sec1_bytes(bytes)
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        let bytes = public_key.to_sec1_bytes().to_vec();
        debug_assert_eq!(bytes.len(), Self::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);
        bytes
    }

    fn new_private_key_from_bytes(bytes: &[u8]) -> Result<Self::PrivateKey, Self::Error> {
        ecdsa::SigningKey::from_slice(bytes)
    }

    fn new_signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature, Self::Error> {
        assert_eq!(bytes.len(), Self::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        ecdsa::Signature::from_slice(bytes)
    }

    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8> {
        let bytes = signature.to_vec();
        debug_assert_eq!(bytes.len(), Self::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        bytes
    }

    fn sign(hash: &[u8], private_key: &Self::PrivateKey) -> Result<Self::Signature, Self::Error> {
        private_key.sign_prehash_with_rng(&mut OsRng, hash)
    }

    fn verify(
        hash: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(public_key.verify_prehash(hash, signature).is_ok())
    }

    fn new_node_id(_public_key: &Self::PublicKey) -> NodeId {
        unimplemented!()
    }

    fn ecdh(_point: &Self::PublicKey, _scalar: &Self::PrivateKey) -> Vec<u8> {
        unimplemented!()
    }

    fn new_public_key_from_private_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        *private_key.verifying_key()
    }
}
