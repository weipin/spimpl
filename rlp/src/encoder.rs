// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Provides convenience functions for RLP encoding.

use crate::Encode;

/// Encodes 'value' and appends the result to `output`.
#[inline]
pub fn encode<T: Encode>(value: T, output: &mut Vec<u8>) {
    <T as Encode>::encode(value, output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use parity_rlp;

    #[test]
    fn test_encode_single_value() {
        let test_data = [
            // py_sandbox: `first_byte_eq_0`
            (hex!("00").to_vec(), &hex!("00") as &[u8]),
            // `first_byte_lt_0x7f`
            (hex!("66").to_vec(), &hex!("66")),
            // `first_byte_eq_0x7f`
            (hex!("7f").to_vec(), &hex!("7f")),
            // `first_byte_eq_0x80`
            (hex!("").to_vec(), &hex!("80")),
            // `first_byte_lt_0xb7_a`
            (hex!("80").to_vec(), &hex!("8180")),
            // `first_byte_lt_0xb7_b`
            (hex!("0102030405").to_vec(), &hex!("850102030405")),
            // `first_byte_eq_0xb7`
            ((0..55).collect::<Vec<u8>>(), &hex!("b7000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            // `first_byte_eq_0xb8`
            ((0..56).collect::<Vec<u8>>(), &hex!("b838000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            // `first_byte_lt_0xbf`
            ((0..60).collect::<Vec<u8>>(), &hex!("b83c000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
        ];

        for (value, encoded) in test_data {
            let mut output = vec![];
            encode(value.as_slice(), &mut output);
            assert_eq!(output, encoded);
        }
    }

    #[test]
    fn test_encode_list() {
        let test_data = [
            // py_sandbox: `first_byte_eq_0xc0`
            (vec![] as Vec<u16>, &hex!("c0") as &[u8]),
            // `first_byte_lt_0xf7`
            (vec![1, 2, 3], &hex!("c3010203")),
            // `first_byte_eq_0xf7`
            ((0..55).collect::<Vec<u16>>(), &hex!("f7800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f30313233343536")),
            // `first_byte_eq_0xf8`
            ((0..56).collect::<Vec<u16>>(), &hex!("f838800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637")),
            // `first_byte_lt_ff`
            ((0..60).collect::<Vec<u16>>(), &hex!("f83c800102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b")),
        ];

        for (value, encoded) in test_data {
            let mut output = vec![];
            encode(value.as_slice(), &mut output);
            assert_eq!(output, encoded);
        }
    }

    #[test]
    fn test_encode_slice_length_less_than_2_bytes() {
        let data: Vec<u64> = (0u64..=u16::MAX as u64).collect();

        let mut rlp_encoded = vec![];
        encode(data.as_slice(), &mut rlp_encoded);

        let mut stream = parity_rlp::RlpStream::new_list(data.len());
        for i in data {
            stream.append(&i);
        }
        assert_eq!(rlp_encoded, stream.out());
    }
}
