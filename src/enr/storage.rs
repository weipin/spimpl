// Copyright 2022 Developers of the simple_enr project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Record storage implementation.

use super::types::{SequenceNumber, SEQUENCE_NUMBER_INITIAL};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
pub(crate) struct Storage {
    pub(crate) signature_value: Option<Vec<u8>>,
    pub(crate) seq: SequenceNumber,
    pub(crate) id: Option<&'static str>,
    pub(crate) public_key_value: Option<Vec<u8>>,
    pub(crate) ip4: Option<Ipv4Addr>,
    pub(crate) tcp4: Option<u16>,
    pub(crate) udp4: Option<u16>,
    pub(crate) ip6: Option<Ipv6Addr>,
    pub(crate) tcp6: Option<u16>,
    pub(crate) udp6: Option<u16>,
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            signature_value: None,
            seq: SEQUENCE_NUMBER_INITIAL,
            id: None,
            public_key_value: None,
            ip4: None,
            tcp4: None,
            udp4: None,
            ip6: None,
            tcp6: None,
            udp6: None,
        }
    }
}
