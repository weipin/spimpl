// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `Vec<u8>`.

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl<'a> Decode<'a> for Vec<u8> {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        Ok(payload.0.to_vec())
    }
}

impl Encode for Vec<u8> {
    fn encode_to(&self, output: &mut Vec<u8>) {
        ItemPayloadSlice(self.as_slice()).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    #[test]
    fn test_byte_vec() {
        let data = vec![1, 2, 3];
        // eth_rlp.py: `encode_bytes_1_2_3`
        let encoded = &[0x83, 1, 2, 3];

        let output = encode(&data);
        assert_eq!(output, encoded);

        assert_eq!(decode::<Vec<u8>>(encoded).unwrap(), data);
    }
}
