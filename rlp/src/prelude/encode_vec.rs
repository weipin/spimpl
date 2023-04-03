// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements RLP encoding for `Vec<T>` and its slice.

use crate::{encode, Encode, ItemPayloadSlice};

impl<'a, T> Encode for &'a [T]
where
    &'a T: Encode,
{
    fn encode(self, output: &mut Vec<u8>) {
        let mut payload = vec![];
        self.iter().for_each(|element| {
            encode(element, &mut payload);
        });
        ItemPayloadSlice(&payload).encode_as_list(output);
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode(self, output: &mut Vec<u8>) {
        let mut payload = vec![];
        self.into_iter().for_each(|element| {
            encode(element, &mut payload);
        });
        ItemPayloadSlice(&payload).encode_as_list(output);
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::encode;

    #[test]
    fn test_encode_slice_of_u16() {
        let data: &[u16] = &[1, 2, 3];
        let mut output = vec![];
        encode(data, &mut output);

        // py_playground: `encode_vec_of_uint_1_2_3`
        assert_eq!(output, hex!("c3010203"));
    }

    #[test]
    fn test_encode_vec_of_u16() {
        let data: Vec<u16> = vec![1, 2, 3];
        let mut output = vec![];
        encode(data, &mut output);

        // py_playground: `encode_vec_of_uint_1_2_3`
        assert_eq!(output, hex!("c3010203"));
    }

    #[test]
    fn test_encode_slice_of_byte_slice() {
        let data: &[&[u8]] = &[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]];
        let mut output = vec![];
        encode(data, &mut output);

        // py_playground: `encode_vec_of_bytes_1_2_3`
        assert_eq!(output, hex!("cc830102038301020383010203"));
    }

    #[test]
    fn test_encode_vec_of_byte_slice() {
        let data: Vec<&[u8]> = vec![&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]];
        let mut output = vec![];
        encode(data, &mut output);

        // py_playground: `encode_vec_of_bytes_1_2_3`
        assert_eq!(output, hex!("cc830102038301020383010203"));
    }
}
