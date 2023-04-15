// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use hex_literal::hex;
use rlp::{decode, encode, Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq)]
struct Signature<'a>(&'a [u8]);

#[test]
fn test_signature() {
    let signature = Signature(&[1, 2, 3]);
    let encoded = encode(&signature);
    // eth_rlp.py: `encode_bytes_1_2_3`
    assert_eq!(&encoded, &hex!("83010203"));

    let signature_decoded: Signature = decode(&encoded).unwrap();
    assert_eq!(signature_decoded, signature);
}

#[test]
fn test_vec_of_signature() {
    let signature = Signature(&[1, 2, 3]);
    let v = vec![signature];

    let encoded = encode(&v);
    // eth_rlp.py: `encode_vec_of_bytes_1_2_3_a`
    assert_eq!(&encoded, &hex!("c483010203"));

    let vec_of_signature_decoded: Vec<Signature> = decode(&encoded).unwrap();
    assert_eq!(vec_of_signature_decoded, v);
}
