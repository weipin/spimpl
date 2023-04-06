// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Traits types have to implement to support RLP serialization.

use crate::{Error, ItemPayloadSlice, ItemType};
pub use rlp_derive::*;

/// Trait for RLP encoding.
pub trait Encode {
    /// Encodes `self` and appends the result to `output`.
    fn encode(self, output: &mut Vec<u8>);
}

/// Trait for RLP decoding.
pub trait Decode<'a> {
    /// The type of the payload item representing `Self`.
    ///
    /// For example, normally, it would be `ItemType::SingleValue` for
    /// primitive types, and `ItemType::List` for Vec<T> or custom structs.
    const TYPE: ItemType;

    /// Decodes `payload` to a `Self`.
    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error>
    where
        Self: Sized;
}
