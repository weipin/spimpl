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

    let encoded = encode(&entry);
    // eth_rlp.py: `encode_uint_1_bytes_1_2_3_bytes_4_5_6`
    assert_eq!(&encoded, &hex!("c9018301020383040506"));

    let entry_decoded: Entry = decode(&encoded).unwrap();
    assert_eq!(entry_decoded, entry);
}

#[test]
fn test_decoding_entry_errors() {
    let data = [
        // eth_rlp.py: `encode_uint_65536_bytes_1_2_3_bytes_4_5_6`
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
    for (encoded, err) in data {
        assert_eq!(decode::<Entry>(encoded).unwrap_err(), err);
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

    let encoded = encode(&v);
    // eth_rlp.py: `encode_vec_of_uint_1_bytes_1_2_3_bytes_4_5_6`
    assert_eq!(&encoded, &hex!("cac9018301020383040506"));

    let vec_of_entry_decoded: Vec<Entry> = decode(&encoded).unwrap();
    assert_eq!(vec_of_entry_decoded, v);
}
