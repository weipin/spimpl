// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// The maximum length in bytes of the "length of a payload in bytes".
///
/// Calculated as `0xbf - 0xb7 = 8` (string) or `0xff - 0xf7 = 8` (list) per
/// spec. In other words, "Byte arrays containing 2^64 or more bytes cannot be
/// encoded."
pub(crate) const MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH: usize = 8;
