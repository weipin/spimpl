// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[repr(u8)]
pub enum Type {
    Ping = 0x01,
    Pong = 0x02,
    FindNode = 0x03,
    Nodes = 0x04,
    TalkReq = 0x05,
    TalkResp = 0x06,
}
