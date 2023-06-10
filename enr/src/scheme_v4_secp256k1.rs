// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [rust-secp256k1][1] implementation of scheme v4.
//!
//! [1]: https://github.com/rust-bitcoin/rust-secp256k1

#[cfg(test)]
use crate::scheme_v4::MockOsRng as OsRng;
#[cfg(not(test))]
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng, RngCore};
use secp256k1::{ecdsa, Message, SECP256K1};
use sha3::{Digest, Keccak256};

use crate::predefined_keys::SCHEME_V4_KEY_OF_PUBLIC_KEY;
use crate::scheme_v4::{
    DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH, ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH,
    ENR_REQUIRED_SIGNATURE_BYTE_LENGTH, SCHEME_V4_ID,
};
use crate::{NodeId, Scheme};

/// rust-secp256k1 implementation of scheme v4.
pub struct Schemev4Secp256k1;

impl Scheme for Schemev4Secp256k1 {
    type PrivateKey = secp256k1::SecretKey;
    type PublicKey = secp256k1::PublicKey;
    type Signature = ecdsa::Signature;
    type Error = secp256k1::Error;

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
        secp256k1::PublicKey::from_slice(bytes)
    }

    fn public_key_to_bytes(public_key: &Self::PublicKey) -> Vec<u8> {
        let bytes = public_key.serialize().to_vec();
        debug_assert_eq!(bytes.len(), Self::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH);
        bytes
    }

    fn new_private_key<R: Rng + CryptoRng + ?Sized>(
        csprng: &mut R,
    ) -> Result<Self::PrivateKey, Self::Error> {
        Ok(secp256k1::SecretKey::new(csprng))
    }

    fn new_private_key_from_bytes(bytes: &[u8]) -> Result<Self::PrivateKey, Self::Error> {
        secp256k1::SecretKey::from_slice(bytes)
    }

    fn new_signature_from_bytes(bytes: &[u8]) -> Result<Self::Signature, Self::Error> {
        assert_eq!(bytes.len(), Self::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        ecdsa::Signature::from_compact(bytes)
    }

    fn signature_to_bytes(signature: &Self::Signature) -> Vec<u8> {
        let bytes = signature.serialize_compact().to_vec();
        debug_assert_eq!(bytes.len(), Self::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH);
        bytes
    }

    fn sign(hash: &[u8], private_key: &Self::PrivateKey) -> Result<Self::Signature, Self::Error> {
        let msg = Message::from_slice(hash)?;
        let mut noncedata = [0; 32];
        OsRng.fill_bytes(&mut noncedata);
        Ok(SECP256K1.sign_ecdsa_with_noncedata(&msg, private_key, &noncedata))
    }

    fn verify(
        hash: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::Error> {
        let msg = Message::from_slice(hash)?;
        // Ok(SECP256K1.verify_ecdsa(&msg, signature, public_key).is_ok())
        match SECP256K1.verify_ecdsa(&msg, signature, public_key) {
            Ok(_) => Ok(true),
            Err(err) => {
                if err == secp256k1::Error::IncorrectSignature {
                    Ok(false)
                } else {
                    Err(err)
                }
            }
        }
    }

    fn new_node_id(public_key: &Self::PublicKey) -> NodeId {
        let uncompressed = &public_key.serialize_uncompressed()[1..];
        NodeId::from_array(Keccak256::digest(uncompressed).into())
    }

    fn ecdh(point: &Self::PublicKey, scalar: &Self::PrivateKey) -> Vec<u8> {
        let shared_secret_point = secp256k1::ecdh::shared_secret_point(point, scalar);

        let mut compressed_shared_secret =
            Vec::with_capacity(Self::DISCV5_REQUIRED_SHARED_SECRET_BYTE_LENGTH);
        if shared_secret_point.last().unwrap() & 1 == 0 {
            compressed_shared_secret.push(2);
        } else {
            compressed_shared_secret.push(3);
        }
        compressed_shared_secret.extend(&shared_secret_point[..32]);
        compressed_shared_secret
    }

    fn new_public_key_from_private_key(private_key: &Self::PrivateKey) -> Self::PublicKey {
        private_key.public_key(SECP256K1)
    }
}
