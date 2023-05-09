// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Type;

pub trait Message: rlp::Encode + for<'a> rlp::Decode<'a> {
    const TYPE: Type;
}
