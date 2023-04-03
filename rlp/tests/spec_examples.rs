// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Tests against the examples from the spec.
use rlp::{decode, encode, Decode, Encode, Error, ItemPayloadSlice, ItemType, U8};

#[test]
fn the_string_dog() {
    let data = b"dog";
    let encoded = &[0x83, b'd', b'o', b'g'];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[test]
fn the_list_cat_dog() {
    let data: &[&[u8]] = &[b"cat", b"dog"];
    let encoded = &[0xc8, 0x83, b'c', b'a', b't', 0x83, b'd', b'o', b'g'];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let v: Vec<&[u8]> = decode(&output).unwrap();
    assert_eq!(v, data);
}

#[test]
fn the_empty_string() {
    let data = b"";
    let encoded = &[0x80];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[test]
fn the_empty_list() {
    let data: &[&[u8]] = &[];
    let encoded = &[0xc0];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let v: Vec<&[u8]> = decode(&output).unwrap();
    assert_eq!(v, data);
}

#[test]
fn the_integer_0() {
    let data = U8(0);
    let encoded = &[0x80];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let n: U8 = decode(&output).unwrap();
    assert_eq!(n, data);
}

#[test]
fn the_encoded_integer_0() {
    let data: &[u8] = &[0x00];
    let encoded = &[0x00];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[test]
fn the_encoded_integer_15() {
    let data: &[u8] = &[0x0f];
    let encoded = &[0x0f];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[test]
fn the_encoded_integer_1024() {
    let data: &[u8] = &[0x04, 0x00];
    let encoded = &[0x82, 0x04, 0x00];

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[test]
fn the_set_theoretical_representation_of_three() {
    // [ [], [[]], [ [], [[]] ] ]
    let element1 = MyVec(vec![]);
    let element2 = MyVec(vec![element1.clone()]);
    let element3 = MyVec(vec![element1.clone(), element2.clone()]);
    let data = vec![element1, element2, element3];
    let encoded = [0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0];

    let mut output = vec![];
    encode(data.as_slice(), &mut output);
    assert_eq!(output, encoded);

    let v: Vec<MyVec> = decode(&output).unwrap();
    assert_eq!(v, data);
}
#[test]
fn the_string_lorem() {
    let data = b"Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    let mut encoded = vec![0xb8, 0x38];
    encoded.extend(data);

    let mut output = vec![];
    encode(data, &mut output);
    assert_eq!(output, encoded);

    let s: &[u8] = decode(&output).unwrap();
    assert_eq!(s, data);
}

#[derive(Clone, Debug, PartialEq)]
struct MyVec(Vec<MyVec>);

impl Encode for &MyVec {
    fn encode(self, output: &mut Vec<u8>) {
        <&[MyVec] as Encode>::encode(self.0.as_slice(), output);
    }
}

impl<'a> Decode<'a> for MyVec {
    const TYPE: ItemType = ItemType::List;

    fn decode(payload: ItemPayloadSlice<'a>) -> Result<Self, Error> {
        let s = <Vec<MyVec> as Decode>::decode(payload)?;
        Ok(MyVec(s))
    }
}
