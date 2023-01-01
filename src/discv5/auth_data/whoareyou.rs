// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::{AuthDataSize, FixedSizeAuthDataSource};
use crate::discv5::message::protocol::whoareyou::{IdNonce, Whoareyou};
use crate::enr;
use std::mem;

impl FixedSizeAuthDataSource for Whoareyou {
    const SIZE: AuthDataSize =
        (mem::size_of::<IdNonce>() + mem::size_of::<enr::SequenceNumber>()) as u16;

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.id_nonce.0);
        buffer.extend(self.enr_seq.to_be_bytes());
    }
}
