// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Decodes RLP and prints the decoded structure.
//!
//! For convenience, we leverage `serde_json` (for its data storage and pretty
//! printing) to represent the structure after the RLP decoding.
//!
//! - The RLP data should be specified in hex format and has an optional prefix '0x'.
//! - RLP strings will be represented in hex format as JSON strings.
//!
//! # Examples
//!
//! ```
//! # The hex input is from https://eips.ethereum.org/EIPS/eip-155
//! cargo run --example rlp_decoder -- 0xec098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a764000080018080
//!
//! # Decodes RLP data `[ [], [[]], [ [], [[]] ] ]`, prefix "0x" omitted.
//! # This is an example from https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
//! cargo run --example rlp_decoder -- c7c0c1c0c3c0c1c0
//! ```

use rlp_types::json::decode_rlp_to_json_value;
use serde_json::to_string_pretty;

fn main() {
    let rlp_hex = std::env::args()
        .nth(1)
        .expect("Error: the parameter is missing");
    let rlp_hex = rlp_hex.strip_prefix("0x").unwrap_or(&rlp_hex);

    let rlp_data = match hex::decode(rlp_hex) {
        Ok(data) => data,
        Err(e) => {
            println!("invalid hex input: {e}");
            return;
        }
    };
    let value = match decode_rlp_to_json_value(&rlp_data) {
        Ok(value) => value,
        Err(e) => {
            println!("Decoding failed: {e}");
            return;
        }
    };
    println!("{}", to_string_pretty(&value).unwrap());
}
