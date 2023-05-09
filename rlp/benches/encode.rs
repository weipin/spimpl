// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};

use rlp::encode;

fn encode_bench(c: &mut Criterion) {
    c.bench_function("encode_u64", |b| {
        b.iter(|| {
            encode(&0x1023_4567_89ab_cdefu64);
        })
    });

    c.bench_function("encode_slice_of_u64", |b| {
        let data: Vec<u64> = (0u64..1000).collect();

        b.iter(|| {
            encode(&data.as_slice());
        })
    });

    c.bench_function("encode_vec_of_u64", |b| {
        let data: Vec<u64> = (0u64..1000).collect();

        b.iter(|| {
            encode(&data);
        })
    });
}

criterion_group!(benches, encode_bench);
criterion_main!(benches);
