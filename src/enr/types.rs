// Copyright 2022 Developers of the simple_enr project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Defines ENR common types.

// The sequence number, a 64-bit unsigned integer.
// Nodes should increase the number whenever the record changes and republish the record.
pub(crate) type SequenceNumber = u64;

// TODO: uses 1?
// The spec doesn't mention the initial value.
// While 0 is a natural choice, the value seems to have a special meaning for discv5:
// "...If enr-seq is zero, the record must be sent...".
pub(crate) const SEQUENCE_NUMBER_INITIAL: SequenceNumber = 1;
