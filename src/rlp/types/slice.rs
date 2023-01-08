// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp::{encode_single_value, DecodingError, Encodable, RlpItemType};

impl Encodable for &[u8] {
    fn encode(self, output: &mut Vec<u8>) {
        encode_single_value(output, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlp::core::Decodable;
    use crate::rlp::encode;
    use ::quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_u8_slice(v: Vec<u8>) -> bool {
        let mut output = vec![];
        encode(v.as_slice(), &mut output);
        output == rlp::encode(&v)
    }
}
