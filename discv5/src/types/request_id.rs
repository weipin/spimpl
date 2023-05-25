// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;

use rlp::{Decode, Error, ItemPayloadSlice, ItemType};

// an RLP byte array of length <= 8 bytes
#[derive(rlp::Encode, Clone, Debug, PartialEq)]
pub struct RequestId<'a>(Cow<'a, [u8]>);

impl<'a> Decode<'a> for RequestId<'a> {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        if payload.0.len() > MAX_REQUEST_ID_BYTE_LENGTH {
            return Err(Error::ItemPayloadByteLengthTooLarge);
        }
        Ok(RequestId(payload.0.into()))
    }
}

impl<'a> RequestId<'a> {
    // Creates a `RequestId` from a byte slice.
    //
    // Returns None if the byte length of the slice is greater than 8.
    pub fn from_slice(slice: &'a [u8]) -> Option<Self> {
        if slice.len() > MAX_REQUEST_ID_BYTE_LENGTH {
            return None;
        }
        Some(RequestId(slice.into()))
    }

    // Creates a `RequestId` from a byte vector.
    //
    // Returns None if the byte length of the vector is greater than 8.
    pub fn from_vec(vec: Vec<u8>) -> Option<Self> {
        if vec.len() > MAX_REQUEST_ID_BYTE_LENGTH {
            return None;
        }
        Some(RequestId(vec.into()))
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

const MAX_REQUEST_ID_BYTE_LENGTH: usize = 8;

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use rlp::{decode, encode};

    use super::*;

    #[test]
    fn test_request_id_rlp() {
        let request_id = RequestId::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        // eth_rlp.py: `encode_bytes_1_2_3_4_5_6_7_8`
        let encoded = hex!("880102030405060708");

        let output = encode(&request_id);
        assert_eq!(output, encoded);

        assert_eq!(decode::<RequestId>(&encoded).unwrap(), request_id);
    }

    #[test]
    fn test_request_id_large_rlp() {
        // eth_rlp.py: `encode_bytes_1_2_3_4_5_6_7_8_9`
        let encoded = hex!("89010203040506070809");
        assert_eq!(
            decode::<RequestId>(&encoded).unwrap_err(),
            rlp::Error::ItemPayloadByteLengthTooLarge
        );
    }
}
