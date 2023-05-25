// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;

/// Represents a sequence number.
pub type SequenceNumber = u64;

/// Represents a 256-bit node ID.
#[derive(rlp::Decode, rlp::Encode, Debug)]
pub struct NodeId<'a>(Cow<'a, NodeIdType>);

/// Type of node ID.
pub type NodeIdType = [u8; 32];

impl<'a> NodeId<'a> {
    /// Creates a `NodeId` from a byte slice.
    pub fn from_slice(slice: &'a [u8; 32]) -> Self {
        NodeId(Cow::Borrowed(slice))
    }

    /// Creates a `NodeId` from a byte array.
    pub const fn from_array(array: [u8; 32]) -> Self {
        NodeId(Cow::Owned(array))
    }

    /// Returns a reference to the byte array.
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
