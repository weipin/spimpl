// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{decode_payload, Decode, Error, ItemPayloadSlice, ItemType};

/// Implements RLP decoding for `Vec<T>`.
impl<'a, T> Decode<'a> for Vec<T>
where
    T: Decode<'a>,
{
    const TYPE: ItemType = ItemType::List;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        let list_iter = payload.list_iter_unchecked();
        let mut v = vec![];
        for result in list_iter {
            match result {
                Ok((item_type, item_payload)) => {
                    let element = decode_payload(item_type, item_payload)?;
                    v.push(element);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::{decode, Error};

    #[test]
    fn test_decode_vec_of_u16() {
        // eth_rlp.py: `encode_vec_of_uint_1_2_3`
        let encoded = &[0xc3, 1, 2, 3];

        assert_eq!(decode::<Vec<u16>>(encoded).unwrap(), vec![1_u16, 2, 3]);
    }

    #[test]
    fn test_decode_vec_of_u16_with_last_element_overflow() {
        // eth_rlp.py: `encode_vec_of_uint_1_2_3_65536`
        let encoded = &[0xc7, 1, 2, 3, 0x83, 1, 0, 0];

        // 65536 = u16::MAX + 1
        assert_eq!(
            decode::<Vec<u16>>(encoded).unwrap_err(),
            Error::ItemPayloadByteLengthTooLarge
        );
    }

    #[test]
    fn test_decode_vec_of_u8_slice() {
        let data: Vec<&[u8]> = vec![&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]];
        // eth_rlp.py: `encode_vec_of_bytes_1_2_3`
        let encoded = hex!("cc830102038301020383010203");

        assert_eq!(decode::<Vec<&[u8]>>(&encoded).unwrap(), data);
    }
}
