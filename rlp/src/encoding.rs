// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP encoding.

use extensions::strip_left_padding;

use crate::constants::MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH;
use crate::{ItemPayloadSlice, ItemType};

impl<'a> ItemPayloadSlice<'a> {
    /// Encodes `self` as a single value and appends the result to `output`.
    #[inline]
    pub fn encode_as_single_value(self, output: &mut Vec<u8>) {
        self.encode(ItemType::SingleValue, output);
    }

    /// Encodes `self` as a list and appends the result to `output`.
    #[inline]
    pub fn encode_as_list(self, output: &mut Vec<u8>) {
        self.encode(ItemType::List, output);
    }

    /// Encodes `self` as `item_type` and appends the result to `output`.
    #[inline]
    fn encode(self, item_type: ItemType, output: &mut Vec<u8>) {
        encode_payload_length(item_type, self, output);
        output.extend(self.0);
    }
}

/// Encodes the length of `payload` according to `item_type` and appends the
/// result to `output`.
fn encode_payload_length(item_type: ItemType, payload: ItemPayloadSlice, output: &mut Vec<u8>) {
    let payload_length = payload.0.len();

    if item_type == ItemType::SingleValue
        && payload_length == 1
        && *payload.0.first().unwrap() < 0x80
    {
        // "For a single byte whose value is in the [0x00, 0x7f] range, that
        // byte is its own RLP encoding."
        return;
    }

    if payload_length < 56 {
        match item_type {
            // "...if a string is 0-55 bytes long, the RLP encoding consists of
            // a single byte with value 0x80 plus the length of the string..."
            ItemType::SingleValue => {
                output.push(0x80 + payload_length as u8);
            }

            // "...if the total payload of a list is 0-55 bytes long, the RLP
            // encoding consists of a single byte with value 0xc0 plus the
            // length of the list..."
            ItemType::List => {
                output.push(0xc0 + payload_length as u8);
            }
        }
    } else {
        let base_value = match item_type {
            // "...If a string is more than 55 bytes long, the RLP encoding
            // consists of a single byte with value 0xb7..."
            ItemType::SingleValue => 0xb7,

            // "...If the total payload of a list is more than 55 bytes long,
            // the RLP encoding consists of a single byte with value 0xf7..."
            ItemType::List => 0xf7,
        };

        // Represents `data_length` in bytes, big-endian without left padding
        // (leading zeroes)
        let bytes = payload_length.to_be_bytes();
        let payload_length_bytes = strip_left_padding(&bytes);
        if payload_length_bytes.len() > MAX_BYTE_LENGTH_OF_PAYLOAD_BYTE_LENGTH {
            // Unlikely, for `usize` is normally up to a maximum of 8 bytes.
            panic!("RLP encoding data too large!");
        }

        // "...plus the length in bytes of the length of the string/payload in
        // binary form..."
        output.push(base_value + payload_length_bytes.len() as u8);
        // "...followed by the length of the string/payload..."
        output.extend(payload_length_bytes);
    }
}
