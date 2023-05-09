// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Flag {
    OrdinaryMessage = 0,
    Whoareyou = 1,
    HandshakeMessage = 2,
}

impl Flag {
    #[inline]
    pub(crate) fn value(self) -> u8 {
        self as u8
    }

    pub(crate) fn from_u8(value: u8) -> Option<Flag> {
        match value {
            value if value == Self::OrdinaryMessage as u8 => Some(Self::OrdinaryMessage),
            value if value == Self::Whoareyou as u8 => Some(Self::Whoareyou),
            value if value == Self::HandshakeMessage as u8 => Some(Self::HandshakeMessage),
            _ => None,
        }
    }
}
