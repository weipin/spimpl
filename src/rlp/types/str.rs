// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::core::Decodable;
use crate::rlp::{encode_single_value, DecodingError, Encodable, RlpItemType};

impl Encodable for &str {
    fn encode(self, output: &mut Vec<u8>) {
        encode_single_value(output, self.as_bytes());
    }
}

impl Decodable for String {
    const TYPE: RlpItemType = RlpItemType::SingleValue;

    fn decode(payload: &[u8]) -> Result<Self, DecodingError> {
        match String::from_utf8(payload.to_vec()) {
            Ok(str) => Ok(str),
            Err(_) => Err(DecodingError::InvalidFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::core::Decodable;
    use crate::rlp::{decode, encode};
    use ::quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_str_slice(s: String) -> bool {
        let mut output = vec![];
        encode(s.as_str(), &mut output);
        output == rlp::encode(&s.as_str())
    }

    #[quickcheck]
    fn test_string(s: String) -> bool {
        let rlp_data = rlp::encode(&s);
        decode::<String>(&rlp_data).unwrap() == s
    }
}
