// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::Decodable;
use crate::rlp::{
    decode_list_payload, decode_payload, encode, encode_item, encode_single_value, DecodingError,
    Encodable, RlpItemType,
};

impl Decodable for Vec<u8> {
    const TYPE: RlpItemType = RlpItemType::SingleValue;

    fn decode(payload: &[u8]) -> Result<Self, DecodingError> {
        Ok(payload.to_vec())
    }
}

// TODO: remove this one
impl Decodable for Vec<u64> {
    const TYPE: RlpItemType = RlpItemType::List;

    fn decode(payload: &[u8]) -> Result<Self, DecodingError> {
        decode_vec(payload)
    }
}

pub(crate) fn decode_vec<T: Decodable>(payload: &[u8]) -> Result<Vec<T>, DecodingError> {
    let items = decode_list_payload(payload)?;
    let mut v = vec![];
    for (item_type, item_payload) in items {
        let element = decode_payload(item_type, item_payload)?;
        v.push(element);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::core::Decodable;
    use crate::rlp::{decode, encode};
    use ::quickcheck_macros::quickcheck;
    use bytes::BytesMut;

    #[quickcheck]
    fn test_u8_vec(v: Vec<u8>) -> bool {
        let rlp_data = rlp::encode(&v);

        decode::<Vec<u8>>(&rlp_data).unwrap() == v
    }

    #[quickcheck]
    fn test_u64_vec(v: Vec<u64>) -> bool {
        let mut rlp_data = BytesMut::new();
        fastrlp::encode_list(&v, &mut rlp_data);

        decode::<Vec<u64>>(&rlp_data).unwrap() == v
    }
}
