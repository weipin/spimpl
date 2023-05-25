// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for `Cow<[u8]>`.

use std::borrow::Cow;

use crate::{Decode, Encode, Error, ItemPayloadSlice, ItemType};

impl<'a> Decode<'a> for Cow<'a, [u8]> {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        Ok(payload.0.into())
    }
}

impl<'a> Encode for Cow<'a, [u8]> {
    fn encode_to(&self, output: &mut Vec<u8>) {
        ItemPayloadSlice(self).encode_as_single_value(output);
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn test_cow_bytes() {
        let data: &[u8] = &[1, 2, 3];
        let cow: Cow<[u8]> = data.into();
        // eth_rlp.py: `encode_bytes_1_2_3`
        let encoded = &[0x83, 1, 2, 3];

        let output = encode(&cow);
        assert_eq!(output, encoded);

        assert_eq!(decode::<Cow<[u8]>>(&output).unwrap(), cow);
    }
}
