// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;

use crate::types::RequestId;

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct TalkResp<'a> {
    pub request_id: RequestId<'a>,
    pub response: Cow<'a, [u8]>,
}

impl<'a> Message<'a> for TalkResp<'a> {
    const TYPE: Type = Type::TalkResp;
}

#[cfg(test)]
mod tests {
    use crate::messages;

    use super::*;

    #[test]
    fn test_talkresp() {
        let request_id_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let talkresp = TalkResp {
            request_id: RequestId::from_vec(request_id_vec).unwrap(),
            response: vec![7, 8, 9].into(),
        };

        let encoded = messages::encode(&talkresp);
        // discv5_playground: `talkresp_1`
        assert_eq!(encoded, hex_literal::hex!("06cd88010203040506070883070809"));

        assert_eq!(rlp::decode::<TalkResp>(&encoded[1..]).unwrap(), talkresp);
    }
}
