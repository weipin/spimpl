// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Simple RLP encoding.

use hex_literal::hex;
use rlp::encode;

#[test]
fn test_encode_u32() {
    let output = encode(&65536_u32);

    // py_playground: `encode_uint_65536`
    assert_eq!(output, hex!("83010000"));
}

#[test]
fn test_encode_slice_of_u16() {
    let data: &[u16] = &[1, 2, 3];
    let output = encode(&data);

    // py_playground: `encode_vec_of_uint_1_2_3`
    assert_eq!(output, hex!("c3010203"));
}

#[test]
fn test_encode_slice_of_byte_slice() {
    let data: &[&[u8]] = &[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]];
    let output = encode(&data);

    // py_playground: `encode_vec_of_bytes_1_2_3`
    assert_eq!(output, hex!("cc830102038301020383010203"));
}
