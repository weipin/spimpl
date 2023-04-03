// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP for JSON.
//!
//! NOTE: RLP strings will be represented in hex format as JSON strings.
//! For example, the RLP string b"abc" will be represented as the JSON string
//! "0x616263". It applies to both JSON => RLP and RLP => JSON.

use rlp::{encode, Error, ItemDataSlice, ItemPayloadSlice, ItemType};
use serde_json::Value;

/// Decodes RLP `data` to a JSON `Value`.
///
/// # Examples
///
/// ```
/// use hex_literal::hex;
/// use rlp_types::json::decode_rlp_to_json_value;
/// use serde_json::to_string;
///
/// let data = hex!("c7c0c1c0c3c0c1c0");
/// let value = decode_rlp_to_json_value(&data).unwrap();
/// assert_eq!("[[],[[]],[[],[[]]]]", to_string(&value).unwrap());
/// ```
pub fn decode_rlp_to_json_value(data: &[u8]) -> Result<Value, Error> {
    let (item_type, payload) = ItemDataSlice(data).as_payload()?;
    decode_payload_to_json_value(item_type, payload)
}

/// Decodes RLP item `payload` to a JSON `Value`.
fn decode_payload_to_json_value(
    item_type: ItemType,
    payload: ItemPayloadSlice,
) -> Result<Value, Error> {
    match item_type {
        ItemType::SingleValue => {
            let bytes_hex = "0x".to_string() + &hex::encode(payload.0);
            Ok(Value::String(bytes_hex))
        }
        ItemType::List => {
            let mut v = vec![];
            for result in payload.list_iter_unchecked() {
                match result {
                    Ok((item, payload)) => v.push(decode_payload_to_json_value(item, payload)?),
                    Err(e) => return Err(e),
                }
            }
            Ok(Value::Array(v))
        }
    }
}

/// Encodes JSON `value` to RLP and returns the output.
///
/// # Examples
///
/// ```
/// use rlp_types::json::encode_json_value_to_rlp;
/// use serde_json;
///
/// let data = "[[],[[]],[[],[[]]]]";
/// let v = serde_json::from_str(data).unwrap();
/// let encoded = encode_json_value_to_rlp(&v);
/// assert_eq!(hex::encode(&encoded), "c7c0c1c0c3c0c1c0");
/// ```
pub fn encode_json_value_to_rlp(value: &Value) -> Vec<u8> {
    let mut output = vec![];
    match value {
        Value::Null => unimplemented!(),
        Value::Bool(_) => unimplemented!(),
        Value::Number(number) => {
            let n = number.as_u64().unwrap();
            encode(n, &mut output);
        }
        Value::String(string) => {
            let bytes = string.as_bytes();
            // Converts hex format string to bytes
            let s = if bytes.starts_with(b"0x") {
                hex::decode(bytes.strip_prefix(b"0x").unwrap()).unwrap()
            } else {
                bytes.to_owned()
            };
            encode(s.as_slice(), &mut output);
        }
        Value::Array(array) => {
            let mut payload = vec![];
            array.iter().for_each(|element| {
                let element_rlp = encode_json_value_to_rlp(element);
                payload.extend(element_rlp);
            });
            ItemPayloadSlice(&payload).encode_as_list(&mut output);
        }
        Value::Object(_) => unimplemented!(),
    }

    output
}
