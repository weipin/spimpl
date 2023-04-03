// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `u8`.

use std::mem::size_of;

use extensions::strip_left_padding;

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

/// Type wraps `u8`.
///
/// The sole purpose of this type is to implement RLP for `u8`.
/// The traits `Encode` and `Decode` cannot be implemented for `u8` to avoid
/// the conflicting implementations of trait `Decode` and `Encode` for
/// type `Vec<u8>`.
///
/// The conflict origins from the fact that an exception has to be made here:
/// `Vec<u8>` should be encoded and decoded as a RLP string instead of a list
/// of `u8`.
///
/// # Examples
///
/// ```
/// use rlp::{decode, encode, U8};
///
/// // Encodes and decodes a `u8` as a single value.
/// let mut output = vec![];
/// encode(U8(123), &mut output);
/// assert_eq!(output, &[0x7b]);
///
/// let decoded: U8 = decode(&output).unwrap();
/// assert_eq!(decoded.0, 123);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct U8(pub u8);

impl Decode<'_> for U8 {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
        if payload.0.len() > size_of::<u8>() {
            return Err(Error::ItemPayloadByteLengthTooLarge);
        }
        if payload.0.is_empty() {
            return Ok(U8(0));
        }
        if *payload.0.first().unwrap() == 0 {
            return Err(Error::UintDecodingFoundLeftPadding);
        }

        Ok(U8(*payload.0.first().unwrap()))
    }
}

impl Encode for U8 {
    fn encode(self, output: &mut Vec<u8>) {
        ItemPayloadSlice(strip_left_padding(&self.0.to_be_bytes())).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};
    use ::quickcheck_macros::quickcheck;
    use parity_rlp;

    #[quickcheck]
    fn test_u8(n: u8) -> bool {
        let parity_rlp_encoded = parity_rlp::encode(&n);

        let mut output = vec![];
        encode(U8(n), &mut output);
        output == parity_rlp_encoded && decode::<U8>(&output).unwrap() == U8(n)
    }

    #[test]
    fn test_encode_zero() {
        let mut output = vec![];
        encode(U8(0), &mut output);
        // py_sandbox: `encode_uint_0`
        assert_eq!(output, &[0x80]);
    }

    #[test]
    fn test_decoding_left_padded() {
        let parity_rlp_encoded = parity_rlp::encode(&0_u64);
        assert_eq!(decode::<U8>(&parity_rlp_encoded).unwrap().0, 0);

        // py_sandbox: `encode_bytes_0`
        let left_padded_bytes_rlp_encoded = &[0x00];
        assert_eq!(
            // py_sandbox: `encode_left_padded_bytes`
            decode::<U8>(left_padded_bytes_rlp_encoded).unwrap_err(),
            Error::UintDecodingFoundLeftPadding
        );
    }

    #[test]
    fn test_decoding_u8_with_overflow() {
        // py_sandbox: `encode_uint_65536`
        // 65536 = u16::MAX + 1
        let rlp_encoded = &[0x83, 1, 0, 0];
        assert_eq!(
            // py_sandbox: `encode_left_padded_bytes`
            decode::<U8>(rlp_encoded).unwrap_err(),
            Error::ItemPayloadByteLengthTooLarge
        );
    }
}
