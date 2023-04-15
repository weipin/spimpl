// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("RLP decoding failed")]
    RLPDecodingError(#[source] rlp::Error),
    #[error("The name of the identity scheme isn't recognized")]
    SchemeNameNotRecognized,
    #[error("Pairs aren't sorted by key or keys aren't unique")]
    KeysNotSortedOrNotUnique,
    #[error("Found a key not followed by any value")]
    PairValueNotFound,
    #[error("The sequence number is absent")]
    SeqNotFound,
    #[error("Byte length of the public key data doesn't match the scheme")]
    PublicKeyDataWithInvalidByteLength,
    #[error("Bytes represent the public key is invalid")]
    InvalidPublicKeyData(String),
    #[error("Byte length of the signature data doesn't match the scheme")]
    SignatureDataWithInvalidByteLength,
    #[error("Bytes represent the signature is invalid")]
    InvalidSignatureData(String),
    #[error("Signature verifying against a content failed")]
    SignatureVerifyingFailed(String),
    #[error("Signature verifying failed for the content doesn't contain the public key data")]
    SignatureVerifyingFailedForMissingPublicKey,
    #[error("Signature constructing for a content failed")]
    SignatureConstructingFailed(String),
    #[error("The content signature is invalid")]
    InvalidSignature,
    #[error("The encoded size of a node record exceeded the maximum")]
    MaximumRecordRlpEncodedByteLengthExceeded,
    #[error("Decoding failed for invalid input")]
    DecodingFailedForInvalidInput,
    #[error("Incrementing a sequence number overflowed")]
    SeqOverflow,
}
