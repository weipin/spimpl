// Copyright 2022 Developers of the lightcryptotools project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::Encodable;

pub fn encode<T: Encodable>(value: T, output: &mut Vec<u8>) {
    Encodable::encode(value, output);
}
