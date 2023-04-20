// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::Ipv4Addr;

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl Decode<'_> for Ipv4Addr {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
        let ip_octets: [u8; 4] = payload
            .0
            .try_into()
            .map_err(|_| Error::InvalidByteRepresentaion)?;
        Ok(Ipv4Addr::from(ip_octets))
    }
}

impl Encode for &Ipv4Addr {
    fn encode_to(self, output: &mut Vec<u8>) {
        ItemPayloadSlice(&self.octets()).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use hex_literal::hex;

    use super::*;
    use crate::{decode, encode};

    #[test]
    fn test_ipv4addr() {
        let ipv4addr = Ipv4Addr::new(127, 0, 0, 1);
        // eth_rlp.py: `encode_bytes_127_0_0_1`
        let encoded = hex!("847f000001");

        let output = encode(&ipv4addr);
        assert_eq!(&output, &encoded);

        let decoded: Ipv4Addr = decode(&output).unwrap();
        assert_eq!(decoded, ipv4addr);
    }

    #[test]
    fn test_ipv4addr_decoding_errors() {
        let test_data = [
            // eth_rlp.py: `encode_bytes_1_2_3`
            (
                "encode_bytes_1_2_3",
                Error::InvalidByteRepresentaion,
                &hex!("83010203") as &[u8],
            ),
            (
                "encode_bytes_127_0_0_1_1",
                Error::InvalidByteRepresentaion,
                &hex!("857f00000101"),
            ),
            (
                "encode_uint_0",
                Error::InvalidByteRepresentaion,
                &hex!("80"),
            ),
        ];

        for (test_name, err, bytes) in test_data {
            assert_eq!(err, decode::<Ipv4Addr>(&bytes).unwrap_err(), "{test_name}");
        }
    }
}
