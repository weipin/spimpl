// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::{encode, encode_item, encode_single_value, DecodingError, Encodable, RlpItemType};

impl Encodable for &[u8] {
    fn encode(self, output: &mut Vec<u8>) {
        encode_single_value(self, output);
    }
}

impl Encodable for &[u64] {
    fn encode(self, output: &mut Vec<u8>) {
        encode_slice_by_value_element(self, output);
    }
}

pub(crate) fn encode_slice_by_value_element<T: Encodable + Copy>(
    slice: &[T],
    output: &mut Vec<u8>,
) {
    let mut payload = vec![];
    for &element in slice {
        Encodable::encode(element, &mut payload);
    }
    encode_item(RlpItemType::List, &payload, output);
}

pub(crate) fn encode_slice_by_ref_element<'a, T>(slice: &'a [T], output: &mut Vec<u8>)
where
    &'a T: Encodable,
{
    let mut payload = vec![];
    for element in slice {
        Encodable::encode(element, &mut payload);
    }
    encode_item(RlpItemType::List, &payload, output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::core::Decodable;
    use crate::rlp::encode;
    use ::quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_u8_slice(v: Vec<u8>) -> bool {
        let mut output = vec![];
        encode(v.as_slice(), &mut output);
        output == rlp::encode(&v)
    }
}
