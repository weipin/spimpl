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
