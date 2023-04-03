// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use rlp::{decode, encode};
use test::Bencher;

#[bench]
fn decode_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _: u64 = decode(&[0x88, 0x10, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).unwrap();
    })
}

#[bench]
fn decode_vec_of_u64(b: &mut test::Bencher) {
    let data: Vec<u64> = (0u64..1000).collect();
    let mut rlp_encoded = vec![];
    encode(data.as_slice(), &mut rlp_encoded);

    let mut stream = parity_rlp::RlpStream::new_list(1000);
    for i in 0..1000u64 {
        stream.append(&i);
    }
    assert_eq!(stream.out(), rlp_encoded);

    b.iter(|| {
        let _: Vec<u64> = decode(&rlp_encoded).unwrap();
    })
}
