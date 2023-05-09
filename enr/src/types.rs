// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Represents a sequence number.
pub type SequenceNumber = u64;

/// Represents a 256-bit node ID.
#[derive(rlp::Decode, rlp::Encode)]
pub struct NodeId(pub [u8; 32]);
