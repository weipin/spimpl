// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub(crate) mod error;
mod handshake_message;
mod ordinary_message;
mod packet;
mod static_header;
mod tests;
mod whoareyou;

pub use error::Error;
pub use handshake_message::{
    unpack as unpack_handshake_message, unpack_with_record as unpack_handshake_message_with_record,
};
pub use ordinary_message::unpack as unpack_ordinary_message;
pub use packet::unpack;
pub use whoareyou::unpack as unpack_whoareyou;
