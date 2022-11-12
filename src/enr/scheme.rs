// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub trait Scheme {
    type PrivateKey;
    type PublicKey;
    type Signature;
    type SigningError;
    type VerifyingError;

    fn id() -> &'static [u8];
    fn public_key_key() -> &'static [u8];

    fn value_to_public_key(value: &[u8]) -> Option<Self::PublicKey>;
    fn public_key_to_value(public_key: &Self::PublicKey) -> Vec<u8>;
    fn value_to_signature(value: &[u8]) -> Option<Self::Signature>;
    fn signature_to_value(public_key: &Self::Signature) -> Vec<u8>;

    fn sign(
        hash: &[u8],
        private_key: &Self::PrivateKey,
    ) -> Result<Self::Signature, Self::SigningError>;
    fn verify(
        msg: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::VerifyingError>;

    fn construct_node_id(public_key: &Self::PublicKey) -> String;
}
