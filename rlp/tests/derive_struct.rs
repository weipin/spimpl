// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use hex_literal::hex;
use rlp::{decode, encode, Decode, Encode, Error};

#[derive(Encode, Decode, Debug, PartialEq)]
struct Entry<'a> {
    id: u16,
    field1: &'a [u8],
    field2: Vec<u8>,
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
    encode(&v, &mut rlp_encoded);
    // py_sandbox: `encode_vec_of_uint_1_bytes_1_2_3_bytes_4_5_6`
    assert_eq!(&rlp_encoded, &hex!("cac9018301020383040506"));

    let vec_of_entry_decoded: Vec<Entry> = decode(&rlp_encoded).unwrap();
    assert_eq!(vec_of_entry_decoded, v);
}
