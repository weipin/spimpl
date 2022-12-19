// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The trait for "identity scheme".

use super::types::NodeId;

pub trait Scheme {
    type PrivateKey;
    type PublicKey;
    type Signature;
    type SigningError;
    type VerifyingError;

    /// Public key length in bytes.
    const ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH: usize;

    /// Signature length in bytes.
    const ENR_REQUIRED_SIGNATURE_BYTE_LENGTH: usize;

    /// ECDH shared secret length in bytes.
    const DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH: usize;

    /// Returns the name of identity scheme.
    fn id() -> &'static [u8];

    /// Returns the key of the "public key" pair.
    fn public_key_key() -> &'static [u8];

    /// Converts bytes to a `Self::PublicKey`.
    fn value_to_public_key(value: &[u8]) -> Option<Self::PublicKey>;

    /// Converts a Self::PublicKey to bytes.
    fn public_key_to_value(public_key: &Self::PublicKey) -> Vec<u8>;

    /// Converts bytes to a `Self::PrivateKey`.
    fn value_to_private_key(value: &[u8]) -> Option<Self::PrivateKey>;

    /// Converts bytes to a `Self::Signature`.
    fn value_to_signature(value: &[u8]) -> Option<Self::Signature>;

    /// Converts a Self::Signature to bytes.
    fn signature_to_value(public_key: &Self::Signature) -> Vec<u8>;

    /// Creates a signature for `hash` with `private_key`.
    fn sign(
        hash: &[u8],
        private_key: &Self::PrivateKey,
    ) -> Result<Self::Signature, Self::SigningError>;

    /// Verifies `hash` with `signature` and `public_key`.
    fn verify(
        hash: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::VerifyingError>;

    /// Constructs a node ID.
    fn construct_node_id(public_key: &Self::PublicKey) -> NodeId;

    /// Creates a shared secret through Diffie-Hellman key agreement
    fn ecdh(point: &Self::PublicKey, scalar: &Self::PrivateKey) -> Vec<u8>;
}
