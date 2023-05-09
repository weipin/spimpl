// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::unpacking;

use super::{FindNode, Nodes, Ping, Pong, TalkReq, TalkResp, Type};

pub fn decode_type(data: &[u8]) -> Result<(Type, &[u8]), unpacking::Error> {
    debug_assert!(!data.is_empty());

    let message_type =
        Type::from_u8(*data.first().unwrap()).ok_or(unpacking::Error::InvalidMessageType)?;
    Ok((message_type, &data[1..]))
}

macro_rules! impl_decode_message {
    ($fname:ident, $t:ty) => {
        #[inline]
        pub fn $fname(message_rlp_encoded: &[u8]) -> Result<$t, unpacking::Error> {
            rlp::decode::<$t>(message_rlp_encoded).map_err(unpacking::Error::RlpDecodingFailed)
        }
    };
}

impl_decode_message!(decode_ping, Ping);
impl_decode_message!(decode_pong, Pong);
impl_decode_message!(decode_findnode, FindNode);
impl_decode_message!(decode_nodes, Nodes);
impl_decode_message!(decode_talkreq, TalkReq);
impl_decode_message!(decode_talkresp, TalkResp);
