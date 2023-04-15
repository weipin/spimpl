// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{ItemDataSlice, ItemPayloadSlice};
use crate::{decode_header_unchecked, decode_payload, Decode, Error, ItemType};

/// An iterator over the items of a RLP list.
#[derive(Debug)]
pub struct ListIter<'a> {
    remaining_list_payload: ItemPayloadSlice<'a>,
}

impl<'a> ListIter<'a> {
    /// Decodes `item_data` and returns a `ListIter` wrapping the underlying
    /// payload of the list items.
    pub fn from_item_data(item_data: ItemDataSlice<'a>) -> Result<ListIter<'a>, Error> {
        let (item_type, payload) = item_data.as_payload()?;
        if item_type != ItemType::List {
            return Err(Error::ItemTypeDoesNotMatch);
        }

        Ok(Self::from_list_payload_unchecked(payload))
    }

    /// Returns a `ListIter` wrapping `list_payload`.
    ///
    /// Does not check if `list_payload` represents an item list.
    pub fn from_list_payload_unchecked(list_payload: ItemPayloadSlice<'a>) -> ListIter<'a> {
        ListIter {
            remaining_list_payload: list_payload,
        }
    }

    /// Returns the subslice of the list payload which hasn't been decoded.
    pub fn remaining_list_payload(&self) -> ItemPayloadSlice<'a> {
        self.remaining_list_payload
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = Result<(ItemType, ItemPayloadSlice<'a>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_list_payload.0.is_empty() {
            return None;
        }

        match decode_header_unchecked(self.remaining_list_payload.0) {
            Ok((item_type, header_byte_length, payload_byte_length)) => {
                if (self.remaining_list_payload.0.len() - header_byte_length as usize)
                    < payload_byte_length as usize
                {
                    return Some(Err(Error::ItemDataWithInvalidByteLength));
                }

                // TODO: handle overflow of
                // `header_byte_length as usize + payload_byte_length as usize`
                let (data1, data2) = self
                    .remaining_list_payload
                    .0
                    .split_at(header_byte_length as usize + payload_byte_length as usize);
                self.remaining_list_payload = ItemPayloadSlice(data2);
                Some(Ok((
                    item_type,
                    ItemPayloadSlice(&data1[header_byte_length as usize..]),
                )))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a> ListIter<'a> {
    /// Advances the iterator and returns the next RLP item or error if decoding
    /// fails.
    ///
    /// Returns `Err(Error::ListDecodingIterationEnded)` when iteration is
    /// finished.
    pub fn next_item<T: Decode<'a>>(&mut self) -> Result<T, Error> {
        match self.next() {
            None => Err(Error::ListDecodingIterationEnded),
            Some(Ok((item_type, payload))) => decode_payload(item_type, payload),
            Some(Err(e)) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_iter() {
        let test_data = [
            // eth_rlp.py: `first_byte_eq_0xc0`
            (0, &hex!("c0") as &[u8]),
            // `first_byte_lt_0xf7`
            (3, &hex!("c3010203")),
            // `first_byte_eq_0xf7`
            (55, &hex!("f7800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            // `first_byte_eq_0xf8`
            (56, &hex!("f838800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            // `first_byte_lt_ff`
            (60, &hex!("f83c800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
        ];

        for (item_num, data) in test_data {
            assert_eq!(
                ItemDataSlice(data)
                    .list_iter()
                    .unwrap()
                    .collect::<Vec<_>>()
                    .len(),
                item_num
            );
        }
    }

    #[test]
    fn test_new_iter_from_single_value_item_should_fail() {
        // eth_rlp.py: `encode_uint_65536`
        assert_eq!(
            ItemDataSlice(&hex!("83010000")).list_iter().unwrap_err(),
            Error::ItemTypeDoesNotMatch
        );
    }

    #[test]
    fn test_iter_next_errors() {
        let test_data = [
            // length of the remaining < `payload_byte_length`
            (Error::ItemDataWithInvalidByteLength, &hex!("b7") as &[u8]),
            // see `test_try_decode_header_errors`
            (Error::ItemDataWithInvalidByteLength, &hex!("b938")),
            (Error::ItemDataWithInvalidByteLength, &hex!("f938")),
        ];

        for (err, payload) in test_data {
            assert_eq!(
                ItemPayloadSlice(payload)
                    .list_iter_unchecked()
                    .next()
                    .unwrap()
                    .unwrap_err(),
                err
            )
        }
    }
}
