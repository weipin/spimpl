// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Helper functions extend the standard library.

#![warn(missing_docs)]

mod bytes;
mod int;
mod vec;

pub use bytes::strip_left_padding;
pub use int::uint_from_bytes::*;
