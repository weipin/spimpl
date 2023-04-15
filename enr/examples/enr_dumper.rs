// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Decodes ENR address and dumps the content.
//!
//! Doesn't do any verification.
//!
//! # Examples
//!
//! ```
//! cargo run --example enr_dumper -- enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8
//! ```

use base64::Engine;
use enr::BASE64_ENGINE;
use serde_json::to_string_pretty;

use rlp_types::json::decode_rlp_to_json_value;

fn main() {
    let enr_address = std::env::args()
        .nth(1)
        .expect("Error: the parameter is missing");
    let base64 = enr_address.strip_prefix("enr:").unwrap_or(&enr_address);

    let rlp_data = match BASE64_ENGINE.decode(base64) {
        Ok(decoded) => decoded,
        Err(e) => {
            println!("Decoding BASE64 failed: {e}");
            return;
        }
    };

    let value = match decode_rlp_to_json_value(&rlp_data) {
        Ok(value) => value,
        Err(e) => {
            println!("Decoding RLP failed: {e}");
            return;
        }
    };
    println!("{}", to_string_pretty(&value).unwrap());
}
