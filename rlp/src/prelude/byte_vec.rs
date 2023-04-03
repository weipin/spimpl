// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP decoding for `Vec<u8>`.

use crate::{Decode, Error, ItemPayloadSlice, ItemType};

impl<'a> Decode<'a> for Vec<u8> {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        Ok(payload.0.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn test_byte_vec() {
        let data = vec![1, 2, 3];
        // py_sandbox: `encode_bytes_1_2_3`
        let rlp_encoded = &[0x83, 1, 2, 3];

        assert_eq!(decode::<Vec<u8>>(rlp_encoded).unwrap(), data);
    }
}