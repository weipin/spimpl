// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(missing_docs)]

/// Errors which can occur when performing RLP encoding and decoding.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("byte length of item data doesn't match header info")]
    ItemDataWithInvalidByteLength,
    #[error("single byte encoded as two")]
    SingleByteEncodedAsTwo,
    #[error("short string encoded as long")]
    ShortStringEncodedAsLong,
    #[error("short list encoded as long")]
    ShortListEncodedAsLong,
    #[error("empty decoding data")]
    EmptyData,
    #[error("unexpected decoding item type")]
    ItemTypeDoesNotMatch,

    // prelude
    #[error("byte length of item payload exceeds decoding type capacity")]
    ItemPayloadByteLengthTooLarge,
    #[error("decoding unsigned integer found left padding")]
    UintDecodingFoundLeftPadding,
    #[error("list iter returned None")]
    ListDecodingIterationEnded,
    #[error("item list number doesn't match decoding type")]
    ListDecodingNumberDoesNotMatch,
    #[error("cannot create a new value from its byte representation")]
    InvalidByteRepresentaion,
}
