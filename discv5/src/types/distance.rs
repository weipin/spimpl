// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use ethnum::U256;

#[derive(Debug)]
pub struct Distance(pub U256);

#[derive(rlp::Encode, rlp::Decode, Clone, Debug, PartialEq)]
pub struct Log2Distance(pub u64);
