// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use enr::{Record, Schemev4Secp256k1};

const EXAMPLE_RECORD_ADDRESS: &str = "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";

fn record_from_address(c: &mut Criterion) {
    c.bench_function("record_from_address_secp256k1", |b| {
        b.iter(|| {
            let _ = Record::from_textual_form::<Schemev4Secp256k1>(EXAMPLE_RECORD_ADDRESS).unwrap();
        })
    });

    #[cfg(feature = "k256")]
    c.bench_function("record_from_address_k256", |b| {
        b.iter(|| {
            let _ = Record::from_textual_form::<enr::Schemev4K256>(EXAMPLE_RECORD_ADDRESS).unwrap();
        })
    });
}

criterion_group!(benches, record_from_address);
criterion_main!(benches);
