// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::RequestId;

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct Ping<'a> {
    pub request_id: RequestId<'a>,
    pub enr_seq: enr::SeqNum,
}

impl<'a> Message<'a> for Ping<'a> {
    const TYPE: Type = Type::Ping;

    const MIN_DATA_BYTE_LENGTH: usize = 3; // see test `min_data_byte_length`
}

#[cfg(test)]
mod tests {
    use crate::messages;

    use super::*;

    #[test]
    fn test_ping() {
        let request_id_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let enr_seq = 7;
        let ping = Ping {
            request_id: RequestId::from_vec(request_id_vec).unwrap(),
            enr_seq,
        };

        let encoded = messages::encode(&ping);
        // discv5_playground: `ping_1`
        assert_eq!(encoded, hex_literal::hex!("01ca88010203040506070807"));

        assert_eq!(rlp::decode::<Ping>(&encoded[1..]).unwrap(), ping);
    }

    #[test]
    fn min_data_byte_length() {
        let message = Ping {
            request_id: RequestId::from_vec(vec![]).unwrap(),
            enr_seq: 0,
        };
        let data = rlp::encode(&message);
        assert_eq!(data.len(), Ping::MIN_DATA_BYTE_LENGTH);
    }
}
