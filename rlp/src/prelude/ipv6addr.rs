// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::Ipv6Addr;

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl Decode<'_> for Ipv6Addr {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
        let ip_octets: [u8; 16] = payload
            .0
            .try_into()
            .map_err(|_| Error::InvalidByteRepresentaion)?;
        Ok(Ipv6Addr::from(ip_octets))
    }
}

impl Encode for &Ipv6Addr {
    fn encode_to(self, output: &mut Vec<u8>) {
        ItemPayloadSlice(&self.octets()).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use hex_literal::hex;

    use super::*;
    use crate::{decode, encode};

    #[test]
    fn test_ipv6addr() {
        let ipv6addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
        // eth_rlp.py: `encode_bytes_ipv6addr_octets`
        let encoded = hex!("9000000000000000000000ffffc00a02ff");

        let output = encode(&ipv6addr);
        assert_eq!(&output, &encoded);

        let decoded: Ipv6Addr = decode(&output).unwrap();
        assert_eq!(decoded, ipv6addr);
    }

    #[test]
    fn test_ipv6addr_decoding_errors() {
        let test_data = [
            // eth_rlp.py: `encode_bytes_1_2_3`
            ("encode_bytes_1_2_3", Error::InvalidByteRepresentaion, &hex!("83010203") as &[u8]),
            ("first_byte_eq_0xb7", Error::InvalidByteRepresentaion, &hex!("b7000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            ("encode_uint_0", Error::InvalidByteRepresentaion, &hex!("80")),
        ];

        for (test_name, err, bytes) in test_data {
            assert_eq!(err, decode::<Ipv6Addr>(&bytes).unwrap_err(), "{test_name}");
        }
    }
}
