// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::decoding::decode_data_header;
use crate::rlp::{decode_data, DecodingError, RlpItemType};

pub struct ListDecoder<'a> {
    remaining_payload: &'a [u8],
}

impl<'a> ListDecoder<'a> {
    pub fn from_rlp_data(rlp_data: &'a [u8]) -> Result<ListDecoder, DecodingError> {
        let (item_type, payload) = decode_data(rlp_data)?;
        if item_type != RlpItemType::List {
            return Err(DecodingError::InvalidFormat);
        }

        Ok(Self::from_list_payload(payload))
    }

    pub fn from_list_payload(list_payload: &'a [u8]) -> ListDecoder {
        ListDecoder {
            remaining_payload: list_payload,
        }
    }
}

impl<'a> Iterator for ListDecoder<'a> {
    type Item = (RlpItemType, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_payload.is_empty() {
            return None;
        }

        let (item_type, header_byte_length, payload_byte_length) =
            decode_data_header(self.remaining_payload).ok()?;
        if self.remaining_payload.len() < header_byte_length as usize + payload_byte_length as usize
        {
            return None;
        }

        let (data1, data2) = self
            .remaining_payload
            .split_at(header_byte_length as usize + payload_byte_length as usize);
        self.remaining_payload = data2;
        Some((item_type, &data1[header_byte_length as usize..]))
    }
}
