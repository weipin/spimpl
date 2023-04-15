// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements record content.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::constants::SEQUENCE_NUMBER_INITIAL;
use crate::SequenceNumber;

/// Represents record content.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Content {
    pub(crate) seq: SequenceNumber,
    pub(crate) id: &'static [u8],
    pub(crate) public_key_data: Option<Vec<u8>>,
    pub(crate) ip4: Option<Ipv4Addr>,
    pub(crate) tcp4: Option<u16>,
    pub(crate) udp4: Option<u16>,
    pub(crate) ip6: Option<Ipv6Addr>,
    pub(crate) tcp6: Option<u16>,
    pub(crate) udp6: Option<u16>,
}

impl Content {
    /// Creates a `Content` with `id`.
    pub(crate) fn new(id: &'static [u8]) -> Self {
        Content {
            seq: SEQUENCE_NUMBER_INITIAL,
            id,
            public_key_data: None,
            ip4: None,
            tcp4: None,
            udp4: None,
            ip6: None,
            tcp6: None,
            udp6: None,
        }
    }
}

/// Represents the RLP encoded form of a `Content`.
#[derive(PartialEq)]
pub(crate) struct ContentRlpEncoded(pub(crate) Vec<u8>);
