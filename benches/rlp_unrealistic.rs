// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use bytes::BytesMut;
use fastrlp::{Decodable, Encodable};
use hex_literal::hex;
use spimpl::rlp;
use spimpl::rlp::{DecodingError, RlpItemType};
use test::Bencher;

#[bench]
fn my_encode_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let mut output = vec![];
        rlp::encode(0x1023_4567_89ab_cdefu64, &mut output);
    })
}

#[bench]
fn encode_u64_a(bench: &mut Bencher) {
    bench.iter(|| {
        let mut output = vec![];
        0x1023_4567_89ab_cdefu64.encode(&mut output);
    })
}

#[bench]
fn my_encode_1000_u64(bench: &mut Bencher) {
    let v: Vec<u64> = (0u64..1000).collect();

    bench.iter(|| {
        let mut output = vec![];
        rlp::encode(v.as_slice(), &mut output);
    })
}

#[bench]
fn encode_1000_u64_a(bench: &mut Bencher) {
    let v = (0..1000u64).into_iter().collect::<Vec<_>>();
    bench.iter(|| {
        let mut out = BytesMut::new();
        fastrlp::encode_list(&v, &mut out);
    })
}

// decoding
#[bench]
fn my_decode_u64(bench: &mut Bencher) {
    let rlp_data = hex!("881023456789abcdef");

    bench.iter(|| {
        let foo: u64 = rlp::decode(&rlp_data).unwrap();
    })
}

#[bench]
fn decode_u64_a(bench: &mut Bencher) {
    let rlp_data = hex!("881023456789abcdef");

    bench.iter(|| {
        let foo = u64::decode(&mut rlp_data.as_slice()).unwrap();
    })
}

#[bench]
fn my_decode_1000_u64(bench: &mut Bencher) {
    let v: Vec<u64> = (0u64..1000).collect();
    let mut output = vec![];
    rlp::encode(v.as_slice(), &mut output);

    bench.iter(|| {
        let v: Vec<u64> = rlp::decode(&output).unwrap();
    })
}

#[bench]
fn decode_1000_u64_a(bench: &mut Bencher) {
    let input = (0..1000u64).into_iter().collect::<Vec<_>>();
    let mut data = BytesMut::new();
    fastrlp::encode_list(input.as_slice(), &mut data);

    bench.iter(|| {
        let _: Vec::<u64> = fastrlp::Decodable::decode(&mut &data[..]).unwrap();
    })
}
