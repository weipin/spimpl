// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Type {
    Ping = 0x01,
    Pong = 0x02,
    FindNode = 0x03,
    Nodes = 0x04,
    TalkReq = 0x05,
    TalkResp = 0x06,
}

impl Type {
    #[inline]
    pub fn value(self) -> u8 {
        self as u8
    }

    pub(crate) fn from_u8(value: u8) -> Option<Type> {
        match value {
            value if value == Self::Ping as u8 => Some(Self::Ping),
            value if value == Self::Pong as u8 => Some(Self::Pong),
            value if value == Self::FindNode as u8 => Some(Self::FindNode),
            value if value == Self::Nodes as u8 => Some(Self::Nodes),
            value if value == Self::TalkReq as u8 => Some(Self::TalkReq),
            value if value == Self::TalkResp as u8 => Some(Self::TalkResp),
            _ => None,
        }
    }
}
