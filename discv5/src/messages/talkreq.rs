// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::RequestId;

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct TalkReq {
    pub request_id: RequestId,
    pub protocol: Vec<u8>,
    pub request: Vec<u8>,
}

impl Message for TalkReq {
    const TYPE: Type = Type::TalkReq;
}

#[cfg(test)]
mod tests {
    use crate::messages;

    use super::*;

    #[test]
    fn test_talkreq() {
        let request_id_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let talkreq = TalkReq {
            request_id: RequestId::from_vec(request_id_vec).unwrap(),
            protocol: vec![1, 2, 3],
            request: vec![4, 5, 6],
        };

        let encoded = messages::encode(&talkreq);
        // discv5_messages: `talkreq_1`
        assert_eq!(
            encoded,
            hex_literal::hex!("05d18801020304050607088301020383040506")
        );

        assert_eq!(rlp::decode::<TalkReq>(&encoded[1..]).unwrap(), talkreq);
    }
}
