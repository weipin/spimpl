// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Tests from Ethereum Consensus Tests:
//! https://github.com/ethereum/tests/tree/develop/RLPTests

#![feature(let_chains)]

use std::fs::File;
use std::path::PathBuf;

use num_bigint::BigUint;
use rlp_types::json::{decode_rlp_to_json_value, encode_json_value_to_rlp};
use rlp_types::RlpBigUint;
use serde_json::Value;

use rlp::{decode, encode, Error};

#[test]
fn rlptest() {
    let root = load_json_root("rlptest.json");
    let cases = root.as_object().unwrap();
    for (name, d) in cases {
        let out = d["out"].as_str().unwrap();
        let rlp_hex = out.strip_prefix("0x").unwrap();
        let rlp_data = hex::decode(rlp_hex).unwrap();

        // Cases involve big int
        if let Some(in_str) = d["in"].as_str() && in_str.starts_with("#") {
            let in_decimal = in_str.strip_prefix("#").unwrap();
            let n = RlpBigUint(BigUint::parse_bytes(in_decimal.as_bytes(), 10).unwrap());
            let mut encoded = vec![];
            encode(&n, &mut encoded);
            assert_eq!(&encoded, &rlp_data);

            let decoded: RlpBigUint = decode(&rlp_data).unwrap();
            assert_eq!(decoded, n, "name: {name}");

            continue;
        };

        let encoded = encode_json_value_to_rlp(&d["in"]);
        assert_eq!(&encoded, &rlp_data);

        // 1) "out" => JSON, `String` and `Number` will be represented in hex
        // format
        let decoded = decode_rlp_to_json_value(&rlp_data).unwrap();
        // 2) JSON => RLP, hex string will be converted to byte and then encoded
        // as RLP string
        let encoded = encode_json_value_to_rlp(&decoded);
        // 3) If step 1 is correct, `encoded` should be the same as `rlp_data`.
        assert_eq!(&encoded, &rlp_data);
    }
}

#[test]
fn invalid_rlptest() {
    let root = load_json_root("invalidRLPTest.json");
    let cases = root.as_object().unwrap();
    for (name, d) in cases {
        let out = d["out"].as_str().unwrap();
        let rlp_hex = out.strip_prefix("0x").unwrap_or(&out);
        let rlp_data = hex::decode(rlp_hex).unwrap();

        if name == "leadingZerosInLongLengthArray1" {
            assert_eq!(
                decode::<RlpBigUint>(&rlp_data).unwrap_err(),
                Error::UintDecodingFoundLeftPadding
            );
            continue;
        }

        if name == "leadingZerosInLongLengthList1" {
            assert_eq!(
                decode::<Vec<u16>>(&rlp_data).unwrap_err(),
                Error::UintDecodingFoundLeftPadding
            );
            continue;
        }

        assert!(decode_rlp_to_json_value(&rlp_data).is_err(), "case: {name}");
    }
}

fn load_json_root(relative_path: &str) -> Value {
    let path = PathBuf::from("./tests")
        .join("RLPTests")
        .join(relative_path);
    let file = File::open(path).unwrap();
    serde_json::from_reader(file).unwrap()
}
