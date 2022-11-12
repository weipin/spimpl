// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use enr::{k256, Enr};
use spimpl::enr::{Record, Schemev4};
use test::Bencher;

const EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjz",
    "CBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
);

type DefaultEnr = Enr<k256::ecdsa::SigningKey>;
type Secp256Enr = Enr<secp256k1::SecretKey>;

#[bench]
fn my_enr_decoding(bench: &mut Bencher) {
    bench.iter(|| {
        Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS).unwrap();
    })
}

#[bench]
fn my_enr_decoding_with_intermediate_decoding_output(bench: &mut Bencher) {
    let mut intermediate_decoding_output = vec![0; 300];
    bench.iter(|| {
        Record::from_textual_form_with_intermediate_decoding_output::<Schemev4>(
            EXAMPLE_RECORD_ADDRESS,
            &mut intermediate_decoding_output,
        )
        .unwrap();
    })
}

#[bench]
fn sigp_enr_k256_decoding(bench: &mut Bencher) {
    bench.iter(|| {
        EXAMPLE_RECORD_ADDRESS.parse::<DefaultEnr>().unwrap();
    })
}

#[bench]
fn sigp_enr_secp256_decoding(bench: &mut Bencher) {
    bench.iter(|| {
        EXAMPLE_RECORD_ADDRESS.parse::<Secp256Enr>().unwrap();
    })
}
