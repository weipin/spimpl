// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl Decode<'_> for IpAddr {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
        match payload.0.len() {
            4 => Ok(IpAddr::V4(<Ipv4Addr as Decode>::decode(payload)?)),
            16 => Ok(IpAddr::V6(<Ipv6Addr as Decode>::decode(payload)?)),
            _ => Err(Error::InvalidByteRepresentaion),
        }
    }
}

impl Encode for &IpAddr {
    fn encode_to(self, output: &mut Vec<u8>) {
        match self {
            IpAddr::V4(ip) => <&Ipv4Addr as Encode>::encode_to(ip, output),
            IpAddr::V6(ip) => <&Ipv6Addr as Encode>::encode_to(ip, output),
        }
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::{decode, encode};

    use super::*;

    #[test]
    fn test_ipv4addr() {
        let ipv4addr = Ipv4Addr::new(127, 0, 0, 1);
        let ipaddr = IpAddr::V4(ipv4addr);
        // eth_rlp.py: `encode_bytes_127_0_0_1`
        let encoded = hex!("847f000001");

        let output = encode(&ipaddr);
        assert_eq!(&output, &encoded);

        let decoded: IpAddr = decode(&output).unwrap();
        assert_eq!(decoded, ipaddr);
    }

    #[test]
    fn test_ipv6addr() {
        let ipv6addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
        let ipaddr = IpAddr::V6(ipv6addr);
        // eth_rlp.py: `encode_bytes_ipv6addr_octets`
        let encoded = hex!("9000000000000000000000ffffc00a02ff");

        let output = encode(&ipaddr);
        assert_eq!(&output, &encoded);

        let decoded: IpAddr = decode(&output).unwrap();
        assert_eq!(decoded, ipaddr);
    }

    #[test]
    fn test_ipaddr_decoding_errors() {
        let test_data = [
            // eth_rlp.py: `encode_bytes_1_2_3`
            ("encode_bytes_1_2_3", Error::InvalidByteRepresentaion, &hex!("83010203") as &[u8]),
            ("encode_bytes_1_2_3_4_5_6_7_8_9", Error::InvalidByteRepresentaion, &hex!("89010203040506070809")),
            ("first_byte_eq_0xb7", Error::InvalidByteRepresentaion, &hex!("b7000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            ("encode_uint_0", Error::InvalidByteRepresentaion, &hex!("80")),
        ];

        for (test_name, err, bytes) in test_data {
            assert_eq!(err, decode::<IpAddr>(&bytes).unwrap_err(), "{test_name}");
        }
    }
}
