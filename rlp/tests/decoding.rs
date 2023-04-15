// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Simple RLP decoding.

use hex_literal::hex;
use rlp::decode;

#[test]
fn test_decode_u32() {
    let value: u32 = decode(&hex!("83010000")).unwrap();

    // py_playground: `encode_uint_65536`
    assert_eq!(value, 65536_u32);
}

#[test]
fn test_decode_vec_of_u16() {
    let value: Vec<u16> = decode(&hex!("c3010203")).unwrap();

    // py_playground: `encode_vec_of_uint_1_2_3`
    assert_eq!(value, &[1, 2, 3]);
}

#[test]
fn test_decode_vec_of_byte_slice() {
    let value: Vec<&[u8]> = decode(&hex!("cc830102038301020383010203")).unwrap();

    // py_playground: `encode_vec_of_bytes_1_2_3`
    assert_eq!(value, &[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]]);
}
