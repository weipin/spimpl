// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `&[u8]`.

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl<'a> Decode<'a> for &'a [u8] {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        Ok(payload.0)
    }
}

impl Encode for &[u8] {
    fn encode(self, output: &mut Vec<u8>) {
        ItemPayloadSlice(self).encode_as_single_value(output);
    }
}

impl Encode for &&[u8] {
    fn encode(self, output: &mut Vec<u8>) {
        ItemPayloadSlice(self).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    #[test]
    fn test_byte_slice() {
        let data: &[u8] = &[1, 2, 3];
        // py_sandbox: `encode_bytes_1_2_3`
        let rlp_encoded = &[0x83, 1, 2, 3];

        let mut output = vec![];
        encode(data, &mut output);
        assert_eq!(output, rlp_encoded);

        assert_eq!(decode::<&[u8]>(&output).unwrap(), data);
    }
}
