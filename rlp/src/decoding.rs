// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP decoding.

use extensions::new_u64_from_be_bytes_with_left_padding;

use crate::error::Error;
use crate::types::{
    HeaderByteLength, ItemDataSlice, ItemPayloadSlice, ItemType, PayloadByteLength,
};

impl<'a> ItemDataSlice<'a> {
    /// Decodes the head part of the data, and returns the type and payload of
    /// the item.
    ///
    /// The returned payload is a subslice of the data.
    pub fn as_payload(&self) -> Result<(ItemType, ItemPayloadSlice<'a>), Error> {
        let (item_type, header_byte_length, payload_byte_length) = decode_header_unchecked(self.0)?;
        if (self.0.len() - header_byte_length as usize) != payload_byte_length as usize {
            return Err(Error::ItemDataWithInvalidByteLength);
        }

        Ok((
            item_type,
            ItemPayloadSlice(&self.0[header_byte_length as usize..]),
        ))
    }
}

/// Decodes the header part of `data`, and returns the type, header length,
/// and payload length of the item.
///
/// Doesn't check that the length of `data` is valid.
pub fn decode_header_unchecked(
    data: &[u8],
) -> Result<(ItemType, HeaderByteLength, PayloadByteLength), Error> {
    if data.is_empty() {
        return Err(Error::EmptyData);
    }

    let first = *data.first().unwrap();
    match first {
        // "For a single byte whose value is in the [0x00, 0x7f] range, that
        // byte is its own RLP encoding"
        0x00..=0x7f => Ok((ItemType::SingleValue, 0, 1)),

        // "...if a string is 0-55 bytes long, the RLP encoding consists of a
        // single byte with value 0x80 plus the length of the string followed
        // by the string. The range of the first byte is thus [0x80, 0xb7]"
        0x80..=0xb7 => {
            let payload_byte_length = (first - 0x80) as PayloadByteLength;
            if payload_byte_length == 1 {
                if data.len() == 1 {
                    return Err(Error::ItemDataWithInvalidByteLength);
                }
                // Single byte (in [0x00, 0x7f]) encoded as two is invalid.
                // https://github.com/paritytech/parity-common/issues/49
                if data[1] < 0x80 {
                    return Err(Error::SingleByteEncodedAsTwo);
                }
            }
            Ok((ItemType::SingleValue, 1, payload_byte_length))
        }

        // "If a string is more than 55 bytes long, the RLP encoding consists
        // of a single byte with value 0xb7 plus the length in bytes of the
        // length of the string in binary form, followed by the length of the
        // string...The range of the first byte is thus [0xb8, 0xbf]"
        0xb8..=0xbf => {
            let byte_length_of_payload_byte_length = first - 0xb7;
            if data.len() < (1 + byte_length_of_payload_byte_length as usize) {
                return Err(Error::ItemDataWithInvalidByteLength);
            }
            let payload_byte_length = new_u64_from_be_bytes_with_left_padding(
                &data[1..=byte_length_of_payload_byte_length as usize],
            );
            // Short string (0-55 bytes) encoded as long is invalid.
            if payload_byte_length < 56 {
                return Err(Error::ShortStringEncodedAsLong);
            }
            Ok((
                ItemType::SingleValue,
                1 + byte_length_of_payload_byte_length,
                payload_byte_length,
            ))
        }

        // "If the total payload of a list (i.e. the combined length of all its
        // items being RLPencoded) is 0-55 bytes long, the RLP encoding consists
        // of a single byte with value 0xc0 plus the length of the list followed
        // by the concatenation of the RLP encodings of the items. The range of
        // the first byte is thus [0xc0, 0xf7]"
        0xc0..=0xf7 => Ok((ItemType::List, 1, (first - 0xc0) as PayloadByteLength)),

        // "If the total payload of a list is more than 55 bytes long, the RLP
        // encoding consists of a single byte with value 0xf7 plus the length
        // in bytes of the length of the payload in binary form, followed by the
        // length of the payload, followed by the concatenation of the RLP
        // encodings of the items. The range of the first byte is thus
        // [0xf8, 0xff]."
        0xf8..=0xff => {
            let byte_length_of_payload_byte_length = first - 0xf7;
            if data.len() < (1 + byte_length_of_payload_byte_length as usize) {
                return Err(Error::ItemDataWithInvalidByteLength);
            }
            let payload_byte_length = new_u64_from_be_bytes_with_left_padding(
                &data[1..=byte_length_of_payload_byte_length as usize],
            );
            // Short list (0-55 bytes) encoded as long is invalid.
            if payload_byte_length < 56 {
                return Err(Error::ShortListEncodedAsLong);
            }

            Ok((
                ItemType::List,
                1 + byte_length_of_payload_byte_length,
                payload_byte_length,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_decode_header_unchecked() {
        let test_data = [
            // eth_rlp.py: `first_byte_eq_0`
            ("first_byte_eq_0", ItemType::SingleValue, 0, 1, &hex!("00") as &[u8]),
            ("first_byte_lt_0x7f", ItemType::SingleValue, 0, 1, &hex!("66")),
            ("first_byte_eq_0x7f", ItemType::SingleValue, 0, 1, &hex!("7f")),
            ("first_byte_eq_0x80", ItemType::SingleValue, 1, 0, &hex!("80")),
            ("first_byte_lt_0xb7_a", ItemType::SingleValue, 1, 1, &hex!("8180")),
            ("first_byte_lt_0xb7_b", ItemType::SingleValue, 1, 5, &hex!("850102030405")),
            ("first_byte_eq_0xb7", ItemType::SingleValue, 1, 55, &hex!("b7000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            ("first_byte_eq_0xb8", ItemType::SingleValue, 2, 56, &hex!("b838000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            ("first_byte_lt_0xbf", ItemType::SingleValue, 2, 60, &hex!("b83c000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
            ("first_byte_eq_0xc0", ItemType::List, 1, 0, &hex!("c0")),
            ("first_byte_lt_0xf7", ItemType::List, 1, 3, &hex!("c3010203")),
            ("first_byte_eq_0xf7", ItemType::List, 1, 55, &hex!("f7800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            ("first_byte_eq_0xf8", ItemType::List, 2, 56, &hex!("f838800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            ("first_byte_lt_ff", ItemType::List, 2, 60, &hex!("f83c800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),

            ("encode_uint_0", ItemType::SingleValue, 1, 0, &hex!("80")),
            ("encode_uint_123", ItemType::SingleValue, 0, 1, &hex!("7b")),
            ("encode_uint_127", ItemType::SingleValue, 0, 1, &hex!("7f")),
            ("encode_uint_128", ItemType::SingleValue, 1, 1, &hex!("8180")),
            ("encode_uint_129", ItemType::SingleValue, 1, 1, &hex!("8181")),
            ("encode_uint_255", ItemType::SingleValue, 1, 1, &hex!("81ff")),
            ("encode_uint_256", ItemType::SingleValue, 1, 2, &hex!("820100")),
            ("encode_uint_65536", ItemType::SingleValue, 1, 3, &hex!("83010000")),

            (
                "max byte length of payload (single value)",
                ItemType::SingleValue,
                9,
                PayloadByteLength::MAX,
                &hex!("bfffffffffffffffff"),
            ),
            (
                "max byte length of payload (list)",
                ItemType::List,
                9,
                PayloadByteLength::MAX,
                &hex!("ffffffffffffffffff"),
            ),
        ];

        for (test_name, item_type, header_len, payload_len, data) in test_data {
            let info = decode_header_unchecked(data).unwrap();
            assert_eq!(info.0, item_type, "{test_name}");
            assert_eq!(info.1, header_len, "{test_name}");
            assert_eq!(info.2, payload_len, "{test_name}");
        }
    }

    #[test]
    fn test_decode_header_unchecked_errors() {
        let test_data = [
            (Error::EmptyData, &hex!("") as &[u8]),
            // not enough bytes for "payload length" construction (single value)
            (Error::ItemDataWithInvalidByteLength, &hex!("b938")),
            // ...continue...(list)
            (Error::ItemDataWithInvalidByteLength, &hex!("f938")),
            // short encoded as long: payload length < 56 (single value)
            (Error::ShortStringEncodedAsLong, &hex!("b801")),
            (Error::ShortStringEncodedAsLong, &hex!("b837")),
            // ...continue...(list)
            (Error::ShortListEncodedAsLong, &hex!("f801")),
            (Error::ShortListEncodedAsLong, &hex!("f837")),
            // single byte encoded as two
            (Error::SingleByteEncodedAsTwo, &hex!("8100")),
            (Error::SingleByteEncodedAsTwo, &hex!("8101")),
            (Error::SingleByteEncodedAsTwo, &hex!("817f")),
        ];

        for (error, data) in test_data {
            assert_eq!(decode_header_unchecked(data).unwrap_err(), error);
        }
    }
}
