// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Returns a subslice with the left padding removed.
///
/// # Examples
///
/// ```
/// use extensions::strip_left_padding;
///
/// assert_eq!(strip_left_padding(&[0, 0, 1]), &[1]);
/// ```
pub fn strip_left_padding(bytes: &[u8]) -> &[u8] {
    if let Some(index) = bytes.iter().position(|&x| x != 0) {
        &bytes[index..]
    } else {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_left_padding() {
        let data: &[(&[u8], &[u8])] = &[
            (&[], &[]),
            (&[0], &[]),
            (&[0, 0], &[]),
            (&[0, 0, 1], &[1]),
            (&[1], &[1]),
            (&[0, 1], &[1]),
            (&[0, 1, 1], &[1, 1]),
            (&[0, 0, 1, 1], &[1, 1]),
            (&[0, 0, 1, 1, 0], &[1, 1, 0]),
        ];
        for &(bytes, stripped) in data {
            assert_eq!(strip_left_padding(bytes), stripped);
        }
    }
}
