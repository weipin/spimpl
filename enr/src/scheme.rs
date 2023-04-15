// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The trait for "identity scheme".
//!
//! # TODO
//!
//! Enforce byte length through `[u8; N]`, for example:
//!
//! ```text
//! pub trait Scheme {
//!     const ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH: usize;
//!
//!     fn new_public_key_from_bytes(
//!         bytes: [u8; ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH],
//!     ) -> Result<Self::PublicKey, Self::Error>;
//! }
//! ```
//!
//! The current code uses assertions instead of an external crate such as
//! generic_array. This is a decision of compromise for code readability.
//!
//! See "Associated constants in traits can not be used in const generics":
//! https://github.com/rust-lang/rust/issues/60551

use core::fmt::Debug;
use std::fmt::Display;

use crate::NodeId;

/// The trait for "identity scheme".
pub trait Scheme {
    /// Private key type of the underlying implementation.
    type PrivateKey;
    /// Public key type of the underlying implementation.
    type PublicKey;
    /// Signature type of the underlying implementation.
    type Signature;
    /// Error type of the underlying implementation.
    type Error: Debug + Display;

    /// Public key length in bytes required by the scheme.
    const ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH: usize;

    /// Signature length in bytes required by the scheme.
    const ENR_REQUIRED_SIGNATURE_BYTE_LENGTH: usize;

    /// ECDH shared secret length in bytes required by discv5.
    const DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH: usize;

    /// Returns the name of the identity scheme.
    fn id() -> &'static [u8];

    /// Returns the key of the "public key" pair.
    fn key_of_public_key() -> &'static [u8];

    /// Creates a `Self::PublicKey` from `bytes`.
    fn new_public_key_from_bytes(bytes: &[u8]) -> Result<Self::PublicKey, Self::Error>;

    /// Returns the byte representation of `Self::PublicKey`.
    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8>;

    /// Creates a `Self::PrivateKey` from `bytes`.
    fn new_private_key_from_bytes(bytes: &[u8]) -> Result<Self::PrivateKey, Self::Error>;

    /// Creates a `Self::Signature` from `bytes`.
    fn new_signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature, Self::Error>;

    /// Returns the byte representation of `Self::Signature`.
    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8>;

    /// Constructs a `Self::Signature` for `hash` with `private_key`.
    fn sign(hash: &[u8], private_key: &Self::PrivateKey) -> Result<Self::Signature, Self::Error>;

    /// Verifies `hash` with `signature` and `public_key`.
    fn verify(
        hash: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::Error>;

    /// Constructs a `NodeId` from `public_key`.
    ///
    /// To derive a node id(address), take the keccak256 hash of the
    /// uncompressed public key.
    fn new_node_id(public_key: &Self::PublicKey) -> NodeId;

    /// Creates a shared secret through Diffie-Hellman key agreement
    ///
    /// "...Creates a secret through elliptic-curve Diffie-Hellman key
    /// agreement. The public key is multiplied by the private key to create a
    /// secret ephemeral key eph = pubkey * privkey. The 33-byte secret output
    /// is y || eph.x where y is 0x02 when eph.y is even or 0x03 when eph.y is
    /// odd.
    fn ecdh(point: &Self::PublicKey, scalar: &Self::PrivateKey) -> Vec<u8>;

    /// Creates a public key for `private_key`.
    fn new_public_key_from_private_key(private_key: &Self::PrivateKey) -> Self::PublicKey;
}
