// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Create a native endian integer value from its representation as a byte
/// array in big endian.
///
/// The length of `bytes` must be 1.
///
/// # Panics
///
/// Will panic if the length of `bytes` isn't in the range.
/// ```
#[inline]
pub fn new_u8_from_be_bytes_with_left_padding(bytes: &[u8]) -> u8 {
    match *bytes {
        [a] => a,
        _ => {
            panic!("invalid byte length, expecting 1");
        }
    }
}

/// Create a native endian integer value from its representation as a byte
/// array in big endian.
///
/// The length of `bytes` must be 1 or 2. If the length is 1, `bytes` will
/// first be left padded with a zero.
///
/// # Panics
///
/// Will panic if the length of `bytes` isn't in the range.
/// ```
#[inline]
pub fn new_u16_from_be_bytes_with_left_padding(bytes: &[u8]) -> u16 {
    match *bytes {
        [b] => u16::from_be_bytes([0, b]),
        [a, b] => u16::from_be_bytes([a, b]),
        _ => {
            panic!("invalid byte length, expecting 1 or 2");
        }
    }
}

/// Create a native endian integer value from its representation as a byte
/// array in big endian.
///
/// The length of `bytes` must be in [1, 4]. If the length is less than 4,
/// `bytes` will first be left padded with zeroes.
///
/// # Panics
///
/// Will panic if the length of `bytes` isn't in the range.
/// ```
#[inline]
pub fn new_u32_from_be_bytes_with_left_padding(bytes: &[u8]) -> u32 {
    match *bytes {
        [d] => u32::from_be_bytes([0, 0, 0, d]),
        [c, d] => u32::from_be_bytes([0, 0, c, d]),
        [b, c, d] => u32::from_be_bytes([0, b, c, d]),
        [a, b, c, d] => u32::from_be_bytes([a, b, c, d]),
        _ => {
            panic!("invalid byte length, expecting 1, 2, 3 or 4");
        }
    }
}

/// Create a native endian integer value from its representation as a byte
/// array in big endian.
///
/// The length of `bytes` must be in [1, 8]. If the length is less than 8,
/// `bytes` will first be left padded with zeroes.
///
/// # Panics
///
/// Will panic if the length of `bytes` isn't in the range.
/// ```
#[inline]
pub fn new_u64_from_be_bytes_with_left_padding(bytes: &[u8]) -> u64 {
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
            panic!("invalid byte length, expecting [1, 8]");
        }
    }
}

#[cfg(test)]
mod new_u8_tests {
    use super::*;

    #[test]
    fn from_be_bytes() {
        assert_eq!(new_u8_from_be_bytes_with_left_padding(&[7]), 7);
    }

    #[test]
    #[should_panic]
    fn from_0_bytes() {
        new_u8_from_be_bytes_with_left_padding(&[]);
    }
}

#[cfg(test)]
mod new_16_tests {
    use super::*;

    #[test]
    fn from_be_bytes() {
        let data = [(&[6_u8] as &[u8], &[0, 6]), (&[5, 6], &[5, 6])];
        for (bytes, byte_array) in data {
            assert_eq!(
                new_u16_from_be_bytes_with_left_padding(bytes),
                u16::from_be_bytes(*byte_array)
            );
        }
    }

    #[test]
    #[should_panic]
    fn from_0_bytes() {
        new_u16_from_be_bytes_with_left_padding(&[]);
    }

    #[test]
    #[should_panic]
    fn from_3_bytes() {
        new_u16_from_be_bytes_with_left_padding(&[1, 2, 3]);
    }
}

#[cfg(test)]
mod new_u32_tests {
    use super::*;

    #[test]
    fn from_be_bytes() {
        let data = [
            (&[6_u8] as &[u8], &[0, 0, 0, 6]),
            (&[5, 6], &[0, 0, 5, 6]),
            (&[4, 5, 6], &[0, 4, 5, 6]),
            (&[3, 4, 5, 6], &[3, 4, 5, 6]),
        ];
        for (bytes, byte_array) in data {
            assert_eq!(
                new_u32_from_be_bytes_with_left_padding(bytes),
                u32::from_be_bytes(*byte_array)
            );
        }
    }

    #[test]
    #[should_panic]
    fn from_0_bytes() {
        new_u32_from_be_bytes_with_left_padding(&[]);
    }

    #[test]
    #[should_panic]
    fn from_5_bytes() {
        new_u32_from_be_bytes_with_left_padding(&[1, 2, 3, 4, 5]);
    }
}

#[cfg(test)]
mod new_u64_tests {
    use super::*;

    #[test]
    fn from_be_bytes() {
        let data = [
            (&[9_u8] as &[u8], &[0, 0, 0, 0, 0, 0, 0, 9]),
            (&[8, 9], &[0, 0, 0, 0, 0, 0, 8, 9]),
            (&[7, 8, 9], &[0, 0, 0, 0, 0, 7, 8, 9]),
            (&[6, 7, 8, 9], &[0, 0, 0, 0, 6, 7, 8, 9]),
            (&[5, 6, 7, 8, 9], &[0, 0, 0, 5, 6, 7, 8, 9]),
            (&[4, 5, 6, 7, 8, 9], &[0, 0, 4, 5, 6, 7, 8, 9]),
            (&[3, 4, 5, 6, 7, 8, 9], &[0, 3, 4, 5, 6, 7, 8, 9]),
            (&[2, 3, 4, 5, 6, 7, 8, 9], &[2, 3, 4, 5, 6, 7, 8, 9]),
        ];
        for (bytes, byte_array) in data {
            assert_eq!(
                new_u64_from_be_bytes_with_left_padding(bytes),
                u64::from_be_bytes(*byte_array)
            );
        }
    }

    #[test]
    #[should_panic]
    fn from_0_bytes() {
        new_u64_from_be_bytes_with_left_padding(&[]);
    }

    #[test]
    #[should_panic]
    fn from_trailing_aligned_be_bytes_9_bytes() {
        new_u64_from_be_bytes_with_left_padding(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
