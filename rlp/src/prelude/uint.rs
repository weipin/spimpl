// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for unsigned integer type.
//!
//! `u8` is excluded, see `U8` for details.

use std::mem::size_of;

use extensions::{
    new_u16_from_be_bytes_with_left_padding, new_u32_from_be_bytes_with_left_padding,
    new_u64_from_be_bytes_with_left_padding, strip_left_padding,
};

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

macro_rules! impl_decode_for_uint {
    ($t:ty, $fn_uint_from_be_bytes: expr) => {
        impl Decode<'_> for $t {
            const TYPE: ItemType = ItemType::SingleValue;

            fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
                if payload.0.len() > size_of::<$t>() {
                    return Err(Error::ItemPayloadByteLengthTooLarge);
                }
                if payload.0.is_empty() {
                    return Ok(0);
                }
                if *payload.0.first().unwrap() == 0 {
                    return Err(Error::UintDecodingFoundLeftPadding);
                }

                Ok($fn_uint_from_be_bytes(payload.0))
            }
        }
    };
}

// impl_decode_for_uint!(u8, new_u8_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u16, new_u16_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u32, new_u32_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u64, new_u64_from_be_bytes_with_left_padding);

macro_rules! impl_encode_for_uint {
    ($t:ty) => {
        impl Encode for $t {
            fn encode_to(self, output: &mut Vec<u8>) {
                ItemPayloadSlice(strip_left_padding(&self.to_be_bytes()))
                    .encode_as_single_value(output);
            }
        }
    };
}

// impl_encode_for_uint!(u8);
impl_encode_for_uint!(u16);
impl_encode_for_uint!(u32);
impl_encode_for_uint!(u64);

impl_encode_for_uint!(&u16);
impl_encode_for_uint!(&u32);
impl_encode_for_uint!(&u64);

#[cfg(test)]
mod tests {
    use ::quickcheck_macros::quickcheck;
    use parity_rlp;

    use crate::{decode, encode, Error};

    macro_rules! impl_test_int {
        ($test_name:ident, $t:ty) => {
            #[quickcheck]
            fn $test_name(n: $t) -> bool {
                let parity_rlp_encoded = parity_rlp::encode(&n);

                let output = encode(n);
                output == parity_rlp_encoded && decode::<$t>(&output).unwrap() == n
            }
        };
    }

    // impl_test_int!(test_u8, u8);
    impl_test_int!(test_u16, u16);
    impl_test_int!(test_u32, u32);
    impl_test_int!(test_u64, u64);

    #[test]
    fn test_decoding_left_padded() {
        let parity_rlp_encoded = parity_rlp::encode(&0_u64);
        assert_eq!(decode::<u64>(&parity_rlp_encoded).unwrap(), 0);

        let left_padded_bytes_rlp_encoded = &[0x82, 0, 1];
        assert_eq!(
            // eth_rlp.py: `encode_left_padded_bytes`
            decode::<u64>(left_padded_bytes_rlp_encoded).unwrap_err(),
            Error::UintDecodingFoundLeftPadding
        );
    }

    #[test]
    fn test_decoding_0() {
        // eth_rlp.py: `encode_uint_0`
        let n: u16 = decode(&[0x80]).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn test_decoding_1() {
        // eth_rlp.py: `encode_uint_1`
        let n: u16 = decode(&[0x01]).unwrap();
        assert_eq!(n, 1);
    }

    #[test]
    fn test_decoding_u16_with_overflow() {
        // eth_rlp.py: `encode_uint_65536`
        // 65536 = u16::MAX + 1
        let encoded = &[0x83, 1, 0, 0];
        assert_eq!(
            // `encode_left_padded_bytes`
            decode::<u16>(encoded).unwrap_err(),
            Error::ItemPayloadByteLengthTooLarge
        );
    }
}
