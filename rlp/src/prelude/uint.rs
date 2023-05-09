// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for unsigned integer type.
//!
//! `u8` is excluded, see `U8` for details.

use extensions::{
    new_u16_from_be_bytes_with_left_padding, new_u32_from_be_bytes_with_left_padding,
    new_u64_from_be_bytes_with_left_padding,
};

macro_rules! impl_decode_for_uint {
    ($t:ty, $new_zero: expr, $fn_uint_from_be_bytes: expr) => {
        impl crate::Decode<'_> for $t {
            const TYPE: crate::ItemType = crate::ItemType::SingleValue;

            fn decode(payload: crate::ItemPayloadSlice) -> Result<Self, crate::Error> {
                if payload.0.len() > std::mem::size_of::<$t>() {
                    return Err(crate::Error::ItemPayloadByteLengthTooLarge);
                }
                if payload.0.is_empty() {
                    return Ok($new_zero);
                }
                if *payload.0.first().unwrap() == 0 {
                    return Err(crate::Error::UintDecodingFoundLeftPadding);
                }

                Ok($fn_uint_from_be_bytes(payload.0))
            }
        }
    };
}

pub(crate) use impl_decode_for_uint;

// impl_decode_for_uint!(u8, 0, new_u8_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u16, 0, new_u16_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u32, 0, new_u32_from_be_bytes_with_left_padding);
impl_decode_for_uint!(u64, 0, new_u64_from_be_bytes_with_left_padding);

macro_rules! impl_encode_for_uint {
    ($t:ty) => {
        impl crate::Encode for $t {
            fn encode_to(&self, output: &mut Vec<u8>) {
                crate::ItemPayloadSlice(extensions::strip_left_padding(&self.to_be_bytes()))
                    .encode_as_single_value(output);
            }
        }
    };
}

pub(crate) use impl_encode_for_uint;

// impl_encode_for_uint!(u8);
impl_encode_for_uint!(u16);
impl_encode_for_uint!(u32);
impl_encode_for_uint!(u64);

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

                let output = encode(&n);
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
        // eth_rlp.py: `encode_left_padded_bytes`
        let left_padded_bytes_rlp_encoded = &[0x82, 0, 1];
        assert_eq!(
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
