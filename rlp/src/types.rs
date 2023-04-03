// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Types used to implement RLP serialization.

use crate::{Error, ListIter};

/// The type of a RLP item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    /// A single item -- a string (i.e. byte array).
    SingleValue,
    /// A list of items.
    List,
}

/// The length in bytes of a payload.
///
/// The spec limits the length to `u64::MAX`.
/// See `MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH` for details.
pub type PayloadByteLength = u64;

/// Represents "length in bytes of the length of the payload".
pub type ByteLengthOfPayloadByteLength = u8;

/// The length in bytes of the header part of an item.
pub type HeaderByteLength = u8;

/// Represents the data of an item.
///
/// The state of the underlying data is undefined.
#[derive(Copy, Clone, Debug)]
pub struct ItemDataSlice<'a>(pub &'a [u8]);

impl<'a> ItemDataSlice<'a> {
    /// Creates a `ListIter` from `self`.
    pub fn list_iter(self) -> Result<ListIter<'a>, Error> {
        ListIter::from_item_data(self)
    }
}

/// Represents the data of an item payload.
///
/// The state of the underlying data is undefined.
#[derive(Copy, Clone, Debug)]
pub struct ItemPayloadSlice<'a>(pub &'a [u8]);

impl<'a> ItemPayloadSlice<'a> {
    /// Creates a `ListIter` from `self`.
    ///
    /// Does not check if `self` represents an item list.
    pub fn list_iter_unchecked(self) -> ListIter<'a> {
        ListIter::from_list_payload_unchecked(self)
    }
}
