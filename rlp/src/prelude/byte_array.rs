// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `[u8; N]`.

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl<'a, const N: usize> Decode<'a> for [u8; N] {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice) -> Result<Self, Error> {
        payload
            .0
            .try_into()
            .map_err(|_| Error::InvalidByteRepresentaion)
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode_to(&self, output: &mut Vec<u8>) {
        ItemPayloadSlice(self).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode, Error};

    #[test]
    fn test_byte_array() {
        let data: [u8; 3] = [1, 2, 3];
        // eth_rlp.py: `encode_bytes_1_2_3`
        let encoded = &[0x83, 1, 2, 3];

        let output = encode(&data);
        assert_eq!(output, encoded);

        assert_eq!(decode::<[u8; 3]>(&output).unwrap(), data);
    }

    #[test]
    fn test_decoding_byte_array_number_does_not_match() {
        // eth_rlp.py: `encode_bytes_1_2_3`
        let encoded = &[0x83, 1, 2, 3];

        assert_eq!(
            decode::<[u8; 4]>(encoded).unwrap_err(),
            Error::InvalidByteRepresentaion
        );
    }
}
