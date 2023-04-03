// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Examples for newtype RLP serialization.

use hex_literal::hex;
use rlp::{decode, encode, Decode, Encode, Error, ItemPayloadSlice, ItemType};

#[derive(Debug, PartialEq)]
struct Signature<'a>(&'a [u8]);

impl Encode for &Signature<'_> {
    fn encode(self, output: &mut Vec<u8>) {
        encode(self.0, output);
    }
}

impl<'a> Decode<'a> for Signature<'a> {
    const TYPE: ItemType = ItemType::SingleValue;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        let s = <&'a [u8] as Decode>::decode(payload)?;
        Ok(Signature(s))
    }
}

#[test]
fn test_signature() {
    let signature = Signature(&[1, 2, 3]);
    let mut rlp_encoded = vec![];
    encode(&signature, &mut rlp_encoded);
    // py_sandbox: `encode_bytes_1_2_3`
    assert_eq!(&rlp_encoded, &hex!("83010203"));

    let signature_decoded: Signature = decode(&rlp_encoded).unwrap();
    assert_eq!(signature_decoded, signature);
}

#[test]
fn test_vec_of_signature() {
    let signature = Signature(&[1, 2, 3]);
    let v = vec![signature];

    let mut rlp_encoded = vec![];
    encode(v.as_slice(), &mut rlp_encoded);
    // py_sandbox: `encode_vec_of_bytes_1_2_3_a`
    assert_eq!(&rlp_encoded, &hex!("c483010203"));

    let vec_of_signature_decoded: Vec<Signature> = decode(&rlp_encoded).unwrap();
    assert_eq!(vec_of_signature_decoded, v);
}
