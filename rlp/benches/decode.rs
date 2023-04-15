// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use rlp::{decode, encode};

fn decode_bench(c: &mut Criterion) {
    c.bench_function("decode_u64", |b| {
        b.iter(|| {
            let _: u64 = decode(&[0x88, 0x10, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]).unwrap();
        })
    });

    let data: Vec<u64> = (0u64..1000).collect();
    let encoded = encode(&data);
    let mut stream = parity_rlp::RlpStream::new_list(1000);
    for i in 0..1000u64 {
        stream.append(&i);
    }
    assert_eq!(stream.out(), encoded);

    c.bench_function("decode_vec_of_u64", |b| {
        b.iter(|| {
            let _: Vec<u64> = decode(&encoded).unwrap();
        })
    });
}

criterion_group!(benches, decode_bench);
criterion_main!(benches);
