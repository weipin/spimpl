// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP serialization for types from external crates, mainly for
//! testing.

mod biguint;
pub mod json;

pub use biguint::RlpBigUint;
