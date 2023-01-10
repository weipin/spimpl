// Copyright 2022 Developers of the lightcryptotools project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP (Recursive Length Prefix) encoding.
//! https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp

use super::core::{RlpItemType, MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH};
use crate::utils::bytes::strip_leading_zeros;

/// Encodes `payload` as a single value item.
pub fn encode_single_value(payload: &[u8], output: &mut Vec<u8>) {
    encode_item(RlpItemType::SingleValue, payload, output);
}

/// Encodes `payload` as a single value item or a list item.
/// The item type is specified by `item_type`.
pub fn encode_item(item_type: RlpItemType, payload: &[u8], output: &mut Vec<u8>) {
    encode_payload_length(item_type, payload, output);
    output.extend(payload);
}

/// Encodes RLP header of `payload`.
///
/// `payload`: the string/list in its binary form.
///
/// While this function encodes only the length of the `payload`,
/// the `payload` itself is still required for the examination of its first byte.
pub(crate) fn encode_payload_length(item_type: RlpItemType, payload: &[u8], output: &mut Vec<u8>) {
    let payload_length = payload.len();

    if item_type == RlpItemType::SingleValue
        && payload_length == 1
        && *payload.first().unwrap() < 0x80
    {
        // "For a single byte whose value is in the [0x00, 0x7f] range, that byte is its own RLP encoding."
        return;
    }

    if payload_length < 56 {
        match item_type {
            // "...if a string is 0-55 bytes long,
            // the RLP encoding consists of a single byte with value 0x80
            // plus the length of the string..."
            RlpItemType::SingleValue => {
                output.push(0x80 + payload_length as u8);
            }

            // "...if the total payload of a list is 0-55 bytes long,
            // the RLP encoding consists of a single byte with value 0xc0
            // plus the length of the list..."
            RlpItemType::List => {
                output.push(0xc0 + payload_length as u8);
            }
        }
    } else {
        let base_value = match item_type {
            // "...If a string is more than 55 bytes long,
            // the RLP encoding consists of a single byte with value 0xb7..."
            RlpItemType::SingleValue => 0xb7,

            // "...If the total payload of a list is more than 55 bytes long,
            // the RLP encoding consists of a single byte with value 0xf7..."
            RlpItemType::List => 0xf7,
        };

        // Represents `data_length` in bytes, big-endian without leading zero bytes
        let bytes = payload_length.to_be_bytes();
        let payload_length_bytes = strip_leading_zeros(&bytes);
        if payload_length_bytes.len() > MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH {
            // this should never happen, for usize is up to a maximum of 8 bytes on a 64 bit target
            panic!("RLP encoding data too large!");
        }

        // "...plus the length in bytes of the length of the string/payload in binary form..."
        output.push(base_value + payload_length_bytes.len() as u8);
        // "...followed by the length of the string/payload..."
        output.extend(payload_length_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let mut output = vec![];
        // The string “dog” = [ 0x83, ‘d’, ‘o’, ‘g’ ]
        encode_single_value("dog".as_bytes(), &mut output);
        assert_eq!(output, vec![0x83, b'd', b'o', b'g']);

        // The list [ “cat”, “dog” ] = [ 0xc8, 0x83, 'c', 'a', 't', 0x83, 'd', 'o', 'g' ]
        let items = [
            (RlpItemType::SingleValue, &[b'c', b'a', b't'][..]),
            (RlpItemType::SingleValue, &[b'd', b'o', b'g'][..]),
        ];
        let mut payload = vec![];
        for item in items {
            encode_single_value(item.1, &mut payload);
        }
        output.clear();
        encode_list(&mut output, &payload);
        assert_eq!(
            output,
            vec![0xc8, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g']
        );

        // The empty string (‘null’) = [ 0x80 ]
        output.clear();
        encode_single_value(&[], &mut output);
        assert_eq!(output, vec![0x80]);

        // The empty list = [ 0xc0 ]
        output.clear();
        encode_list(&mut output, &[]);
        assert_eq!(output, vec![0xc0]);

        // The integer 0 = [ 0x80 ]
        output.clear();
        encode_single_value(strip_leading_zeros(&0_u8.to_be_bytes()), &mut output);
        assert_eq!(output, vec![0x80]);

        // The encoded integer 0 (’\x00’) = [ 0x00 ]
        output.clear();
        encode_single_value(&[0x00], &mut output);
        assert_eq!(output, vec![0x00]);

        // The encoded integer 15 (’\x0f’) = [ 0x0f ]
        output.clear();
        encode_single_value(&[0x0f], &mut output);
        assert_eq!(output, vec![0x0f]);

        // The encoded integer 1024 (’\x04\x00’) = [ 0x82, 0x04, 0x00 ]
        output.clear();
        encode_single_value(strip_leading_zeros(&1024_usize.to_be_bytes()), &mut output);
        assert_eq!(output, vec![0x82, 0x04, 0x00]);

        // The set theoretical representation of three,
        // [ [], [[]], [ [], [[]] ] ] = [ 0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0 ]
        let mut encoded1 = vec![];
        encode_list(&mut encoded1, &[]); // []
        let mut encoded2 = vec![];
        encode_list(&mut encoded2, &encoded1); // [[]]
        let mut payload = encoded1.clone();
        payload.extend(encoded2.clone()); // [], [[]]
        let mut encoded3 = vec![];
        encode_list(&mut encoded3, &payload); // [ [], [[]] ]
        let mut payload = encoded1.clone();
        payload.extend(&encoded2);
        payload.extend(&encoded3);
        output.clear();
        encode_list(&mut output, &payload);
        assert_eq!(output, vec![0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0]);

        // The string
        // “Lorem ipsum dolor sit amet, consectetur adipisicing elit” =
        // [ 0xb8, 0x38, 'L', 'o', 'r', 'e', 'm', ' ', ... , 'e', 'l', 'i', 't' ]
        let str_bytes = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
        let mut encoded = vec![0xb8, 0x38];
        encoded.extend(str_bytes);
        output.clear();
        encode_single_value(str_bytes, &mut output);
        assert_eq!(output, encoded);
    }

    /// Encodes `payload` as a list item.
    fn encode_list(output: &mut Vec<u8>, payload: &[u8]) {
        encode_item(RlpItemType::List, payload, output)
    }
}
