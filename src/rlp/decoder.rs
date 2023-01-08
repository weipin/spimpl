// Copyright 2022 Developers of the lightcryptotools project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::Decodable;
use crate::rlp::{decode_data, DecodingError, RlpItemType};

pub fn decode_payload<T: Decodable>(
    item_type: RlpItemType,
    payload: &[u8],
) -> Result<T, DecodingError> {
    if item_type != T::TYPE {
        return Err(DecodingError::InvalidFormat);
    }
    T::decode(payload)
}

pub fn decode<T: Decodable>(bytes: &[u8]) -> Result<T, DecodingError> {
    let (item_type, payload) = decode_data(bytes)?;
    decode_payload(item_type, payload)
}
