// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Some tests from the Parity implemenation.
//! https://github.com/paritytech/parity-common/tree/master/rlp/tests

use hex_literal::hex;
use rlp::{decode, encode, Decode, Encode, Error, ItemPayloadSlice, ItemType};

#[test]
fn test_rlp_data_length_check() {
    let data = &[0x84, b'c', b'a', b't'];
    assert_eq!(
        decode::<&[u8]>(data).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
}

#[test]
fn test_rlp_long_data_length_check() {
    let mut data = hex!("b8ff").to_vec();
    for _ in 0..253 {
        data.push(b'c');
    }

    assert_eq!(
        decode::<&[u8]>(&data).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
}

#[test]
fn test_the_exact_long_string() {
    let mut data = hex!("b8ff").to_vec();
    for _ in 0..255 {
        data.push(b'c');
    }

    assert!(decode::<&[u8]>(&data).is_ok());
}

#[test]
fn test_rlp_2bytes_data_length_check() {
    let mut data = hex!("b902ff").to_vec(); // 512+255
    for _ in 0..700 {
        data.push(b'c');
    }

    assert_eq!(
        decode::<&[u8]>(&data).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
}

#[test]
fn test_rlp_list_length_overflow() {
    let data = hex!("ffffffffffffffffff000000");
    assert_eq!(
        decode::<&[u8]>(&data).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
}

#[test]
fn test_canonical_string_encoding() {
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 3, 0x82, b'a', b'b']).unwrap(),
        vec![b"ab"],
    );

    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 4, 0xb7 + 1, 2, b'a', b'b']).unwrap_err(),
        Error::ItemPayloadWithInvalidByteLengthLessThan56
    );
}

#[test]
fn test_canonical_list_encoding() {
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 3, 0x82, b'a', b'b']).unwrap(),
        vec![b"ab"],
    );

    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xf7 + 1, 3, 0x82, b'a', b'b']).unwrap_err(),
        Error::ItemPayloadWithInvalidByteLengthLessThan56
    );
}

#[test]
fn test_inner_length_capping_for_short_lists() {
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0, 0x82, b'a', b'b']).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 1, 0x82, b'a', b'b']).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 2, 0x82, b'a', b'b']).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 3, 0x82, b'a', b'b']).unwrap(),
        vec![b"ab"],
    );
    assert_eq!(
        decode::<Vec<&[u8]>>(&[0xc0 + 4, 0x82, b'a', b'b']).unwrap_err(),
        Error::ItemDataWithInvalidByteLength
    );
}

#[test]
fn test_nested_list_roundtrip() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Inner(u64, u64);

    impl Encode for &Inner {
        fn encode(self, output: &mut Vec<u8>) {
            let mut payload = vec![];
            encode(self.0, &mut payload);
            encode(self.1, &mut payload);

            ItemPayloadSlice(&payload).encode_as_list(output);
        }
    }

    impl<'a> Decode<'a> for Inner {
        const TYPE: ItemType = ItemType::List;

        fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
            let mut list_iter = payload.list_iter_unchecked();
            let u1: u64 = list_iter.next_item()?;
            let u2: u64 = list_iter.next_item()?;
            if !list_iter.next().is_none() {
                return Err(Error::ListDecodingNumberDoesNotMatch);
            }

            Ok(Inner(u1, u2))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Nest<T>(Vec<T>);

    impl<'a, T> Encode for &'a Nest<T>
    where
        &'a T: Encode,
    {
        fn encode(self, output: &mut Vec<u8>) {
            encode(&self.0, output);
        }
    }

    impl<'a, T: Decode<'a>> Decode<'a> for Nest<T> {
        const TYPE: ItemType = ItemType::List;

        fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
            let v = <Vec<T> as Decode>::decode(payload)?;
            Ok(Nest(v))
        }
    }

    let items = (0..4).map(|i| Inner(i, i + 1)).collect();
    let nest = Nest(items);

    let mut encoded = vec![];
    encode(&nest, &mut encoded);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(nest, decoded);

    let nest2 = Nest(vec![nest.clone(), nest]);

    let mut encoded = vec![];
    encode(&nest2, &mut encoded);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(nest2, decoded);
}
