// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Provides convenience functions for RLP decoding.

use crate::{Decode, Error, ItemDataSlice, ItemPayloadSlice, ItemType};

/// Decodes `data` to a `T`.
///
/// # Examples
///
/// ```
/// use rlp::decode;
///
/// let value: u32 = decode(&[0x83, 0x01, 0x00, 0x00]).unwrap();
/// assert_eq!(value, 65536_u32);
/// ```
#[inline]
pub fn decode<'a, T: Decode<'a>>(data: &'a [u8]) -> Result<T, Error> {
    let (item_type, payload) = ItemDataSlice(data).as_payload()?;
    decode_payload(item_type, payload)
}

/// Decodes `payload` to a `T`.
///
/// `item_type` and `payload` can be obtained by decoding the header of the
/// payload.
///
/// # Examples
///
/// ```
/// use rlp::{decode_payload, ItemPayloadSlice, ItemType};
///
/// let value: u32 =
///     decode_payload(ItemType::SingleValue, ItemPayloadSlice(&[0x01, 0x00, 0x00])).unwrap();
/// assert_eq!(value, 65536_u32);
/// ```
#[inline]
pub fn decode_payload<'a, T: Decode<'a>>(
    item_type: ItemType,
    payload: ItemPayloadSlice<'a>,
) -> Result<T, Error> {
    if item_type != T::TYPE {
        return Err(Error::ItemTypeDoesNotMatch);
    }
    <T as Decode>::decode(payload)
}

#[cfg(test)]
mod tests {
    use crate::encode;

    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_decode_string() {
        let test_data = [
            // py_sandbox: `first_byte_eq_0`
            (hex!("00").to_vec(), &hex!("00") as &[u8]),
            // `first_byte_lt_0x7f`
            (hex!("66").to_vec(), &hex!("66")),
            // `first_byte_eq_0x7f`
            (hex!("7f").to_vec(), &hex!("7f")),
            // `first_byte_eq_0x80`
            (hex!("").to_vec(), &hex!("80")),
            // `first_byte_lt_0xb7_a`
            (hex!("80").to_vec(), &hex!("8180")),
            // `first_byte_lt_0xb7_b`
            (hex!("0102030405").to_vec(), &hex!("850102030405")),
            // `first_byte_eq_0xb7`
            ((0..55).collect::<Vec<u8>>(),  &hex!("b7000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            // `first_byte_eq_0xb8`
            ((0..56).collect::<Vec<u8>>(), &hex!("b838000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            // `first_byte_lt_0xbf`
            ((0..60).collect::<Vec<u8>>(), &hex!("b83c000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
        ];

        for (value, data) in test_data {
            assert_eq!(decode::<Vec<u8>>(data).unwrap(), value.to_vec());
        }
    }

    #[test]
    fn test_decode_list() {
        let test_data = [
            // py_sandbox: `first_byte_eq_0xc0`
            (vec![], &hex!("c0") as &[u8]),
            // `encode_vec_of_uint_0_1`
            (vec![0_u16, 1], &hex!("c28001")),
            // `first_byte_lt_0xf7`
            (vec![1_u16, 2, 3], &hex!("c3010203")),
            // `first_byte_eq_0xf7`
            ((0..55).collect::<Vec<u16>>(), &hex!("f7800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            // `first_byte_eq_0xf81
            ((0..56).collect::<Vec<u16>>(), &hex!("f838800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            // `first_byte_lt_ff`
            ((0..60).collect::<Vec<u16>>(), &hex!("f83c800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
        ];

        for (value, data) in test_data {
            assert_eq!(decode::<Vec<u16>>(data).unwrap(), value.to_vec());
        }
    }
    #[test]
    fn decode_vec_of_u64() {
        // let data: Vec<u64> = (0u64..1000).collect();
        let data: Vec<u64> = (0u64..1000).collect();
        let mut rlp_encoded = vec![];
        encode(data.as_slice(), &mut rlp_encoded);

        let decoded: Vec<u64> = decode(&rlp_encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_decode_string_error() {
        let test_data = [
            (&hex!("") as &[u8], Error::EmptyData),
            // py_sandbox: `first_byte_lt_0xb7_b`
            (&hex!("8501020304"), Error::ItemDataWithInvalidByteLength),
        ];

        for (data, error) in test_data {
            assert_eq!(decode::<Vec<u8>>(data).unwrap_err(), error);
        }
    }

    #[test]
    fn test_decode_list_error() {
        let test_data = [
            // py_sandbox: `first_byte_lt_0xf7`
            //       **
            // c3010203
            // c30102
            (
                &hex!("c30102") as &[u8],
                Error::ItemDataWithInvalidByteLength,
            ),
            // `encode_uint_65536_bytes_1_2_3_bytes_4_5_6`
            // 65536 = u16::MAX + 1
            (
                &hex!("cc830100008301020383040506"),
                Error::ItemPayloadByteLengthTooLarge,
            ),
            // `encode_vec_of_bytes_1_2_3`
            //   **
            // cc830102038301020383010203
            // ccb70102038301020383010203
            (
                &hex!("ccb70102038301020383010203"),
                Error::ItemDataWithInvalidByteLength,
            ),
        ];

        for (data, error) in test_data {
            assert_eq!(decode::<Vec<u16>>(data).unwrap_err(), error);
        }
    }
}
