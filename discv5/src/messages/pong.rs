// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::IpAddr;

use crate::types::RequestId;

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct Pong<'a> {
    pub request_id: RequestId<'a>,
    pub enr_seq: enr::SeqNum,
    pub recipient_ip: IpAddr,
    pub recipient_port: u16,
}

impl<'a> Message<'a> for Pong<'a> {
    const TYPE: Type = Type::Pong;

    const MIN_DATA_BYTE_LENGTH: usize = 9; // see test `min_data_byte_length`
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use hex_literal::hex;

    use crate::messages;

    use super::*;

    #[test]
    fn test_pong() {
        let request_id = RequestId::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        let enr_seq = 7;
        let ip4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ip6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff));

        // ipv4
        let pong = Pong {
            request_id: request_id.clone(),
            enr_seq,
            recipient_ip: ip4,
            recipient_port: u16::MAX,
        };
        let encoded = messages::encode(&pong);
        // discv5_playground: `pong_ipv4_ipv6`
        assert_eq!(encoded, hex!("02d288010203040506070807847f00000182ffff"));
        assert_eq!(rlp::decode::<Pong>(&encoded[1..]).unwrap(), pong);

        // ipv6
        let pong = Pong {
            request_id,
            enr_seq,
            recipient_ip: ip6,
            recipient_port: u16::MAX,
        };
        let encoded = messages::encode(&pong);
        assert_eq!(
            encoded,
            hex!("02de880102030405060708079000000000000000000000ffffc00a02ff82ffff")
        );
        assert_eq!(rlp::decode::<Pong>(&encoded[1..]).unwrap(), pong);
    }

    #[test]
    fn min_data_byte_length() {
        let message = Pong {
            request_id: RequestId::from_vec(vec![]).unwrap(),
            enr_seq: 0,
            recipient_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            recipient_port: 0,
        };
        let data = rlp::encode(&message);
        assert_eq!(data.len(), Pong::MIN_DATA_BYTE_LENGTH);
    }
}
