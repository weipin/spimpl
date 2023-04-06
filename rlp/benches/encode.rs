// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use rlp::encode;
use test::Bencher;

#[bench]
fn encode_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let mut output = vec![];
        encode(0x1023_4567_89ab_cdefu64, &mut output);
    })
}

#[bench]
fn encode_slice_of_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let data: Vec<u64> = (0u64..1000).collect();

        let mut rlp_encoded = vec![];
        encode(data.as_slice(), &mut rlp_encoded);
    })
}

// Consumes the vec
#[bench]
fn encode_vec_of_u64(b: &mut test::Bencher) {
    b.iter(|| {
        let data: Vec<u64> = (0u64..1000).collect();

        let mut rlp_encoded = vec![];
        encode(&data, &mut rlp_encoded);
    })
}
