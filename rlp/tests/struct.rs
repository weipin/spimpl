// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Examples for struct RLP serialization.

use hex_literal::hex;
use rlp::{decode, encode, Decode, Encode, Error, ItemPayloadSlice, ItemType};

#[derive(Debug, PartialEq)]
struct Entry<'a> {
    id: u16,
    field1: &'a [u8],
    field2: Vec<u8>,
}

impl Encode for &Entry<'_> {
    fn encode(self, output: &mut Vec<u8>) {
        let mut payload = vec![];
        encode(self.id, &mut payload);
        encode(self.field1, &mut payload);
        encode(self.field2.as_slice(), &mut payload);

        ItemPayloadSlice(&payload).encode_as_list(output);
    }
}

impl<'a> Decode<'a> for Entry<'a> {
    const TYPE: ItemType = ItemType::List;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        let mut list_iter = payload.list_iter_unchecked();
        let id: u16 = list_iter.next_item()?;
        let field1: &'a [u8] = list_iter.next_item()?;
        let field2: Vec<u8> = list_iter.next_item()?;

        if !list_iter.next().is_none() {
            return Err(Error::ListDecodingNumberDoesNotMatch);
        }

        Ok(Entry { id, field1, field2 })
    }
}

#[test]
fn test_entry() {
    let entry = Entry {
        id: 1,
        field1: &[1, 2, 3],
        field2: vec![4, 5, 6],
    };

    let mut rlp_encoded = vec![];
    encode(&entry, &mut rlp_encoded);
    // py_sandbox: `encode_uint_1_bytes_1_2_3_bytes_4_5_6`
    assert_eq!(&rlp_encoded, &hex!("c9018301020383040506"));

    let entry_decoded: Entry = decode(&rlp_encoded).unwrap();
    assert_eq!(entry_decoded, entry);
}

#[test]
fn test_decoding_entry_errors() {
    let data = [
        // py_sandbox: `encode_uint_65536_bytes_1_2_3_bytes_4_5_6`
        (
            &hex!("cc830100008301020383040506") as &[u8],
            Error::ItemPayloadByteLengthTooLarge,
        ),
        // `encode_uint_1_bytes_1_2_3_bytes_4_5_6_uint_0`
        (
            &hex!("ca01830102038304050680"),
            Error::ListDecodingNumberDoesNotMatch,
        ),
        // `encode_uint_1_bytes_1_2_3`
        (&hex!("c50183010203"), Error::ListDecodingIterationEnded),
    ];
    for (rlp_encoded, err) in data {
        assert_eq!(decode::<Entry>(rlp_encoded).unwrap_err(), err);
    }
}

#[test]
fn test_vec_of_entry() {
    let entry = Entry {
        id: 1,
        field1: &[1, 2, 3],
        field2: vec![4, 5, 6],
    };
    let v = vec![entry];

    let mut rlp_encoded = vec![];
    encode(v.as_slice(), &mut rlp_encoded);
    // py_sandbox: `encode_vec_of_uint_1_bytes_1_2_3_bytes_4_5_6`
    assert_eq!(&rlp_encoded, &hex!("cac9018301020383040506"));

    let vec_of_entry_decoded: Vec<Entry> = decode(&rlp_encoded).unwrap();
    assert_eq!(vec_of_entry_decoded, v);
}
