// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::storage::Storage;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Record(pub(crate) Storage);

impl Record {
    pub fn ip4(&self) -> Option<Ipv4Addr> {
        self.0.ip4
    }

    pub fn tcp4(&self) -> Option<u16> {
        self.0.tcp4
    }

    pub fn udp4(&self) -> Option<u16> {
        self.0.udp4
    }
}
