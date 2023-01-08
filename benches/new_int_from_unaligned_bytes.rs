// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate test;

use spimpl::utils::int_from_bytes::{
    new_u16_from_unaligned_bytes, new_u32_from_unaligned_bytes, new_u64_from_unaligned_bytes,
    new_u8_from_unaligned_bytes, tmp_new_u16_from_unaligned_bytes,
    tmp_new_u32_from_unaligned_bytes, tmp_new_u64_from_unaligned_bytes,
    tmp_new_u8_from_unaligned_bytes,
};
use test::Bencher;

#[bench]
fn new_u8(bench: &mut Bencher) {
    let bytes = [0xef_u8];
    bench.iter(|| {
        let n = new_u8_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn tmp_new_u8(bench: &mut Bencher) {
    let bytes = [0xef_u8];
    bench.iter(|| {
        let n = tmp_new_u8_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn new_u16(bench: &mut Bencher) {
    let bytes = [0xef_u8, 0x73];
    bench.iter(|| {
        let n = new_u16_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn tmp_new_u16(bench: &mut Bencher) {
    let bytes = [0xef_u8, 0x73];
    bench.iter(|| {
        let n = tmp_new_u16_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn new_u32(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73];
    bench.iter(|| {
        let n = new_u32_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn tmp_new_u32(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73];
    bench.iter(|| {
        let n = tmp_new_u32_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn new_u64(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73, 0x23, 0x87, 0x99, 0x01];
    bench.iter(|| {
        let n = new_u64_from_unaligned_bytes(&bytes);
    })
}

#[bench]
fn tmp_new_u64(bench: &mut Bencher) {
    let bytes = [0x3f_u8, 0x11, 0xef, 0x73, 0x23, 0x87, 0x99, 0x01];
    bench.iter(|| {
        let n = tmp_new_u64_from_unaligned_bytes(&bytes);
    })
}
