// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("rlp decoding failed")]
    RLPDecodingError(#[source] rlp::Error),
    #[error("identity scheme name not recognized")]
    SchemeNameNotRecognized,
    #[error("pairs aren't sorted by key or keys aren't unique")]
    KeysNotSortedOrNotUnique,
    #[error("found a key not followed by any value")]
    PairValueNotFound,
    #[error("sequence number absent")]
    SeqNotFound,
    #[error("byte length of public key data doesn't match scheme")]
    PublicKeyDataWithInvalidByteLength,
    #[error("public key represented by invalid bytes")]
    InvalidPublicKeyData(String),
    #[error("byte length of signature data doesn't match scheme")]
    SignatureDataWithInvalidByteLength,
    #[error("signature represented by invalid bytes")]
    InvalidSignatureData(String),
    #[error("signature verifying against content failed")]
    SignatureVerifyingFailed(String),
    #[error("signature verifying failed: content doesn't contain public key data")]
    SignatureVerifyingFailedForMissingPublicKey,
    #[error("constructing signature for content failed")]
    SignatureConstructingFailed(String),
    #[error("invalid content signature")]
    InvalidSignature,
    #[error("encoded size of node record exceeded maximum")]
    MaximumRecordRlpEncodedByteLengthExceeded,
    #[error("decoding failed for invalid input")]
    DecodingFailedForInvalidInput,
    #[error("incrementing sequence number overflowed")]
    SeqOverflow,
}
