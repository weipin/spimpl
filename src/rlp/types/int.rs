// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::{Decodable, Encodable};
use crate::rlp::{encode_single_value, DecodingError, RlpItemType};
use crate::utils::bytes::strip_leading_zeros;
use crate::utils::int_from_bytes::{
    new_u16_from_unaligned_bytes, new_u32_from_unaligned_bytes, new_u64_from_unaligned_bytes,
    new_u8_from_unaligned_bytes,
};
use std::mem::size_of;

macro_rules! impl_decodable_for_int {
    ($t:ty, $fn_int_from_unaligned_bytes: expr) => {
        impl Decodable for $t {
            const TYPE: RlpItemType = RlpItemType::SingleValue;

            fn decode(payload: &[u8]) -> Result<Self, DecodingError> {
                if payload.len() > size_of::<$t>() {
                    return Err(DecodingError::InvalidFormat);
                }

                if payload.is_empty() {
                    return Ok(0);
                }
                if *payload.first().unwrap() == 0 {
                    return Err(DecodingError::InvalidFormat);
                }

                Ok($fn_int_from_unaligned_bytes(payload))
            }
        }
    };
}

impl_decodable_for_int!(u8, new_u8_from_unaligned_bytes);
impl_decodable_for_int!(u16, new_u16_from_unaligned_bytes);
impl_decodable_for_int!(u32, new_u32_from_unaligned_bytes);
impl_decodable_for_int!(u64, new_u64_from_unaligned_bytes);

macro_rules! impl_encodable_for_int {
    ($t:ty) => {
        impl Encodable for $t {
            fn encode(self, output: &mut Vec<u8>) {
                encode_single_value(output, strip_leading_zeros(&self.to_be_bytes()));
            }
        }
    };
}

impl_encodable_for_int!(u8);
impl_encodable_for_int!(u16);
impl_encodable_for_int!(u32);
impl_encodable_for_int!(u64);

#[cfg(test)]
mod tests {
    use crate::rlp::core::Decodable;
    use crate::rlp::{decode, encode, DecodingError};
    use ::quickcheck_macros::quickcheck;

    macro_rules! impl_test_int {
        ($test_name:ident, $t:ty) => {
            #[quickcheck]
            fn $test_name(n: $t) -> bool {
                let rlp_data = rlp::encode(&n);

                let mut output = vec![];
                encode(n, &mut output);
                output == rlp_data && decode::<$t>(&rlp_data).unwrap() == n
            }
        };
    }

    impl_test_int!(test_u8, u8);
    impl_test_int!(test_u16, u16);
    impl_test_int!(test_u32, u32);
    impl_test_int!(test_u64, u64);

    #[test]
    fn test_zero() {
        let rlp_data = rlp::encode(&0_u64);
        assert_eq!(decode::<u64>(&rlp_data).unwrap(), 0);

        let rlp_data = rlp::encode(&[0x0_u8, 0x0, 0x1].as_slice());
        assert_eq!(
            decode::<u64>(&rlp_data).unwrap_err(),
            DecodingError::InvalidFormat
        );
    }
}
