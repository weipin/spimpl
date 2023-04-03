// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use extensions::{
    new_u16_from_be_bytes_with_left_padding, new_u32_from_be_bytes_with_left_padding,
    new_u64_from_be_bytes_with_left_padding, new_u8_from_be_bytes_with_left_padding,
};
use test::Bencher;

#[bench]
fn new_u8(bench: &mut Bencher) {
    let bytes = [0xef_u8];
    bench.iter(|| {
        new_u8_from_be_bytes_with_left_padding(&bytes);
    })
}

#[bench]
fn new_u8_copy(bench: &mut Bencher) {
    let bytes = [0xef_u8];
    bench.iter(|| {
        new_u8_from_be_bytes_with_left_padding_copy(&bytes);
    })
}

#[bench]
fn new_u16(bench: &mut Bencher) {
    let bytes = [0xef_u8, 0x73];
    bench.iter(|| {
        new_u16_from_be_bytes_with_left_padding(&bytes);
    })
}

#[bench]
fn new_u16_copy(bench: &mut Bencher) {
    let bytes = [0xef_u8, 0x73];
    bench.iter(|| {
        new_u16_from_be_bytes_with_left_padding_copy(&bytes);
    })
}

#[bench]
fn new_u32(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73];
    bench.iter(|| {
        new_u32_from_be_bytes_with_left_padding(&bytes);
    })
}

#[bench]
fn new_u32_copy(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73];
    bench.iter(|| {
        new_u32_from_be_bytes_with_left_padding_copy(&bytes);
    })
}

#[bench]
fn new_u64(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73, 0x23, 0x87, 0x99, 0x01];
    bench.iter(|| {
        new_u64_from_be_bytes_with_left_padding(&bytes);
    })
}

#[bench]
fn new_u64_copy(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73, 0x23, 0x87, 0x99, 0x01];
    bench.iter(|| {
        new_u64_from_be_bytes_with_left_padding_copy(&bytes);
    })
}

#[inline]
fn new_u8_from_be_bytes_with_left_padding_copy(bytes: &[u8]) -> u8 {
    assert!(bytes.len() == 1);

    let mut n_bytes = [0; size_of::<u8>()];
    n_bytes[(size_of::<u8>() - bytes.len())..].copy_from_slice(bytes);
    u8::from_be_bytes(n_bytes)
}

#[inline]
fn new_u16_from_be_bytes_with_left_padding_copy(bytes: &[u8]) -> u16 {
    assert!(bytes.len() > 0 && bytes.len() <= size_of::<u16>());

    let mut n_bytes = [0; size_of::<u16>()];
    n_bytes[(size_of::<u16>() - bytes.len())..].copy_from_slice(bytes);
    u16::from_be_bytes(n_bytes)
}

#[inline]
fn new_u32_from_be_bytes_with_left_padding_copy(bytes: &[u8]) -> u32 {
    assert!(bytes.len() > 0 && bytes.len() <= size_of::<u32>());

    let mut n_bytes = [0; size_of::<u32>()];
    n_bytes[(size_of::<u32>() - bytes.len())..].copy_from_slice(bytes);
    u32::from_be_bytes(n_bytes)
}

#[inline]
fn new_u64_from_be_bytes_with_left_padding_copy(bytes: &[u8]) -> u64 {
    assert!(bytes.len() > 0 && bytes.len() <= size_of::<u64>());

    let mut n_bytes = [0; size_of::<u64>()];
    n_bytes[(size_of::<u64>() - bytes.len())..].copy_from_slice(bytes);
    u64::from_be_bytes(n_bytes)
}
