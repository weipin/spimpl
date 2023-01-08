// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

// u8
pub fn new_u8_from_unaligned_bytes(bytes: &[u8]) -> u8 {
    debug_assert!(bytes.len() == 1);

    *bytes.first().unwrap()
}

pub fn tmp_new_u8_from_unaligned_bytes(bytes: &[u8]) -> u8 {
    debug_assert!(bytes.len() == 1);

    let mut n_bytes = [0; size_of::<u8>()];
    n_bytes[(std::mem::size_of::<u8>() - bytes.len())..].copy_from_slice(bytes);
    u8::from_be_bytes(n_bytes)
}

// u16
pub fn new_u16_from_unaligned_bytes(bytes: &[u8]) -> u16 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u16>());

    match *bytes {
        [b] => u16::from_be_bytes([0, b]),
        [a, b] => u16::from_be_bytes([a, b]),
        _ => {
            panic!()
        }
    }
}

pub fn tmp_new_u16_from_unaligned_bytes(bytes: &[u8]) -> u16 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u16>());

    let mut n_bytes = [0; size_of::<u16>()];
    n_bytes[(std::mem::size_of::<u16>() - bytes.len())..].copy_from_slice(bytes);
    u16::from_be_bytes(n_bytes)
}

// u32
pub fn new_u32_from_unaligned_bytes(bytes: &[u8]) -> u32 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u32>());

    match *bytes {
        [d] => u32::from_be_bytes([0, 0, 0, d]),
        [c, d] => u32::from_be_bytes([0, 0, c, d]),
        [b, c, d] => u32::from_be_bytes([0, b, c, d]),
        [a, b, c, d] => u32::from_be_bytes([a, b, c, d]),
        _ => {
            panic!()
        }
    }
}

pub fn tmp_new_u32_from_unaligned_bytes(bytes: &[u8]) -> u32 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u32>());

    let mut n_bytes = [0; size_of::<u32>()];
    n_bytes[(std::mem::size_of::<u32>() - bytes.len())..].copy_from_slice(bytes);
    u32::from_be_bytes(n_bytes)
}

// u64
pub fn new_u64_from_unaligned_bytes(bytes: &[u8]) -> u64 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u64>());

    match *bytes {
        [h] => u64::from_be_bytes([0, 0, 0, 0, 0, 0, 0, h]),
        [g, h] => u64::from_be_bytes([0, 0, 0, 0, 0, 0, g, h]),
        [f, g, h] => u64::from_be_bytes([0, 0, 0, 0, 0, f, g, h]),
        [e, f, g, h] => u64::from_be_bytes([0, 0, 0, 0, e, f, g, h]),
        [d, e, f, g, h] => u64::from_be_bytes([0, 0, 0, d, e, f, g, h]),
        [c, d, e, f, g, h] => u64::from_be_bytes([0, 0, c, d, e, f, g, h]),
        [b, c, d, e, f, g, h] => u64::from_be_bytes([0, b, c, d, e, f, g, h]),
        [a, b, c, d, e, f, g, h] => u64::from_be_bytes([a, b, c, d, e, f, g, h]),
        _ => {
            panic!()
        }
    }
}

pub fn tmp_new_u64_from_unaligned_bytes(bytes: &[u8]) -> u64 {
    debug_assert!(bytes.len() > 0 && bytes.len() <= size_of::<u64>());

    let mut n_bytes = [0; size_of::<u64>()];
    n_bytes[(std::mem::size_of::<u64>() - bytes.len())..].copy_from_slice(bytes);
    u64::from_be_bytes(n_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::quickcheck_macros::quickcheck;

    #[test]
    fn test_new_u8_from_unaligned_bytes() {
        for n in 0..=u8::MAX {
            let bytes = n.to_be_bytes();
            assert_eq!(new_u8_from_unaligned_bytes(&bytes), n);
            assert_eq!(tmp_new_u8_from_unaligned_bytes(&bytes), n);
        }
    }

    #[test]
    fn test_new_u16_from_unaligned_bytes() {
        for n in 0..=u16::MAX {
            let bytes = n.to_be_bytes();
            assert_eq!(new_u16_from_unaligned_bytes(&bytes), n);
            assert_eq!(tmp_new_u16_from_unaligned_bytes(&bytes), n);
        }
    }

    #[quickcheck]
    fn test_new_u32_from_unaligned_bytes(n: u32) {
        let bytes = n.to_be_bytes();
        assert_eq!(new_u32_from_unaligned_bytes(&bytes), n);
        assert_eq!(tmp_new_u32_from_unaligned_bytes(&bytes), n);
    }

    #[quickcheck]
    fn test_new_u64_from_unaligned_bytes(n: u64) {
        let bytes = n.to_be_bytes();
        assert_eq!(new_u64_from_unaligned_bytes(&bytes), n);
        assert_eq!(tmp_new_u64_from_unaligned_bytes(&bytes), n);
    }
}
