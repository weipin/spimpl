// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for BigUint.

use extensions::strip_left_padding;
use num_bigint::BigUint;
use num_traits::Zero;
use rlp::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

#[derive(Debug, PartialEq)]
pub struct RlpBigUint(pub BigUint);

impl Encode for RlpBigUint {
    fn encode_to(&self, output: &mut Vec<u8>) {
        ItemPayloadSlice(strip_left_padding(&self.0.to_bytes_be())).encode_as_single_value(output);
    }
}

impl<'a> Decode<'a> for RlpBigUint {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        if payload.0.is_empty() {
            return Ok(RlpBigUint(BigUint::zero()));
        }
        if *payload.0.first().unwrap() == 0 {
            return Err(Error::UintDecodingFoundLeftPadding);
        }

        Ok(RlpBigUint(BigUint::from_bytes_be(payload.0)))
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use rlp::{decode, encode};

    use super::*;

    #[test]
    fn test_biguint() {
        let n = RlpBigUint(
            BigUint::parse_bytes(
                b"105315505618206987246253880190783558935785933862974822347068935681",
                10,
            )
            .unwrap(),
        );

        let output = encode(&n);
        assert_eq!(
            output,
            hex!("9c0100020003000400050006000700080009000a000b000c000d000e01")
        );

        let decoded: RlpBigUint = decode(&output).unwrap();
        assert_eq!(decoded, n);
    }
}
