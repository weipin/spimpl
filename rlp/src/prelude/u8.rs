// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `u8`.

use extensions::new_u8_from_be_bytes_with_left_padding;

use super::uint::{impl_decode_for_uint, impl_encode_for_uint};

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
/// let encoded = encode(&U8(123));
/// assert_eq!(encoded, &[0x7b]);
///
/// let decoded: U8 = decode(&encoded).unwrap();
/// assert_eq!(decoded.0, 123);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct U8(pub u8);

impl U8 {
    /// Return the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    ///
    /// Implemented to fulfil the macro `impl_encode_for_uint`.
    #[inline]
    pub const fn to_be_bytes(self) -> [u8; 1] {
        self.0.to_be_bytes()
    }
}

impl_decode_for_uint!(U8, U8(0), new_u8_newtype_from_be_bytes_with_left_padding);
impl_encode_for_uint!(U8);

#[inline]
pub fn new_u8_newtype_from_be_bytes_with_left_padding(bytes: &[u8]) -> U8 {
    U8(new_u8_from_be_bytes_with_left_padding(bytes))
}

#[cfg(test)]
mod tests {
    use ::quickcheck_macros::quickcheck;
    use parity_rlp;

    use crate::{decode, encode};

    use super::*;

    #[quickcheck]
    fn test_u8(n: u8) -> bool {
        let parity_rlp_encoded = parity_rlp::encode(&n);

        let output = encode(&U8(n));
        output == parity_rlp_encoded && decode::<U8>(&output).unwrap() == U8(n)
    }

    #[test]
    fn test_encode_zero() {
        let output = encode(&U8(0));
        // eth_rlp.py: `encode_uint_0`
        assert_eq!(output, &[0x80]);
    }

    #[test]
    fn test_decoding_left_padded() {
        // eth_rlp.py: `encode_bytes_0`
        let left_padded_bytes_rlp_encoded = &[0x00];
        assert_eq!(
            decode::<U8>(left_padded_bytes_rlp_encoded).unwrap_err(),
            crate::Error::UintDecodingFoundLeftPadding
        );
    }

    #[test]
    fn test_decoding_u8_with_overflow() {
        // eth_rlp.py: `encode_uint_65536`
        // 65536 = u16::MAX + 1
        let encoded = &[0x83, 1, 0, 0];
        assert_eq!(
            // `encode_left_padded_bytes`
            decode::<U8>(encoded).unwrap_err(),
            crate::Error::ItemPayloadByteLengthTooLarge
        );
    }
}
