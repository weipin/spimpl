// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(missing_docs)]

/// Errors which can occur when performing RLP encoding and decoding.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("The byte length of the item data doesn't match the header info")]
    ItemDataWithInvalidByteLength,
    #[error("The data is invalid, the single byte isn't its own RLP encoding")]
    ItemDataWithInvalidSingleByteEncoding,
    #[error("The byte length of the payload is invalid, the value is less than 56")]
    ItemPayloadWithInvalidByteLengthLessThan56,
    #[error("The decoding data is empty")]
    EmptyData,
    #[error("The type of the decoding item is unexpected")]
    ItemTypeDoesNotMatch,

    // prelude
    #[error("The byte length of the item payload exceeds the capacity of the decoding type")]
    ItemPayloadByteLengthTooLarge,
    #[error("The decoding unsigned integer is represented with left padding")]
    UintDecodingFoundLeftPadding,
    #[error("The list iter returned None")]
    ListDecodingIterationEnded,
    #[error("The number of the item list doesn't match the decoding type")]
    ListDecodingNumberDoesNotMatch,
}
