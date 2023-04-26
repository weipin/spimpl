// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Message;

pub fn encode_to<T: Message + rlp::Encode>(value: T, output: &mut Vec<u8>) {
    output.push(T::TYPE as u8);
    rlp::encode_to(value, output);
}

pub fn encode<T: Message + rlp::Encode>(value: T) -> Vec<u8> {
    let mut output = vec![];
    encode_to(value, &mut output);
    output
}

// Ideally, `encode_to` can be written as the code snippet below.
// Unfortunately, it seems this does not work:
// https://github.com/rust-lang/rust/issues/37748
//
// No idea how to get this fixed.
//
// pub fn encode_to<T: Message>(value: &T, output: &mut Vec<u8>)
// where
//     for<'a> &'a T: rlp::Encode,
// {
//     output.push(T::TYPE as u8);
//     rlp::encode_to(value, output);
// }
