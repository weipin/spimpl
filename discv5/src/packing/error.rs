// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("packet size too large")]
    PacketTooLarge,
    #[error("packet size too small")]
    PacketTooSmall,
}
