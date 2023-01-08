// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::Decodable;
use crate::rlp::{encode_single_value, DecodingError, Encodable, RlpItemType};

impl<const N: usize> Decodable for [u8; N] {
    const TYPE: RlpItemType = RlpItemType::SingleValue;

    fn decode(payload: &[u8]) -> Result<Self, DecodingError> {
        if payload.len() != N {
            return Err(DecodingError::InvalidFormat);
        }

        Ok(payload.try_into().unwrap())
    }
}

impl<const N: usize> Encodable for &[u8; N] {
    fn encode(self, output: &mut Vec<u8>) {
        encode_single_value(output, self);
    }
}

#[cfg(test)]
mod tests {
    use crate::rlp::core::Decodable;
    use crate::rlp::{decode, encode};
    use ::quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_u8_array(a: u8, b: u8, c: u8, d: u8, e: u8) -> bool {
        let array = [a, b, c, d, e];
        let rlp_data = rlp::encode(&array.as_slice());

        let mut output = vec![];
        encode(&array, &mut output);
        output == rlp_data && decode::<[u8; 5]>(&rlp_data).unwrap() == array
    }
}
