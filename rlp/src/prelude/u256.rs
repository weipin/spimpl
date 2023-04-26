// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use ethnum::U256;

use super::uint::{impl_decode_for_uint, impl_encode_for_uint};

impl_decode_for_uint!(U256, U256::ZERO, new_u256_from_be_bytes_with_left_padding);
impl_encode_for_uint!(U256);
impl_encode_for_uint!(&U256);

#[inline]
fn new_u256_from_be_bytes_with_left_padding(bytes: &[u8]) -> U256 {
    assert!(!bytes.is_empty() && bytes.len() <= std::mem::size_of::<U256>());

    let mut n_bytes = [0; size_of::<U256>()];
    n_bytes[(size_of::<U256>() - bytes.len())..].copy_from_slice(bytes);
    U256::from_be_bytes(n_bytes)
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::{decode, encode};

    use super::*;

    #[test]
    fn test_u256() {
        let n = U256::from_words(
            0x00010203_04050607_08090a0b_0c0d0e0f,
            0x10111213_14151617_18191a1b_1c1d1e1f,
        );

        let output = encode(n);
        assert_eq!(decode::<U256>(&output).unwrap(), n);
    }

    #[test]
    fn test_encode_zero() {
        let output = encode(U256::ZERO);
        // eth_rlp.py: `encode_uint_0`
        assert_eq!(output, &[0x80]);
    }

    #[test]
    fn test_decoding_left_padded() {
        // eth_rlp.py: `encode_left_padded_bytes`
        let left_padded_bytes_rlp_encoded = &[0x82, 0, 1];

        assert_eq!(
            decode::<U256>(left_padded_bytes_rlp_encoded).unwrap_err(),
            crate::Error::UintDecodingFoundLeftPadding
        );
    }

    #[test]
    fn test_decoding_u8_with_overflow() {
        // eth_rlp.py: `encode_bytes_33`
        let encoded = hex!("a17f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f");
        assert_eq!(
            decode::<U256>(&encoded).unwrap_err(),
            crate::Error::ItemPayloadByteLengthTooLarge
        );
    }
}
