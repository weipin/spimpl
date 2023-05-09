// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Concatenates three slices and copies the result into `$target`.
///
/// For each pair `(usize, &[u8])`, the first element must be the length of the
/// slice (the second element).
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate extensions;
/// # fn main() {
/// let mut buf = [0; 10];
/// vec_copy_from_concatenating_slices3!(&mut buf, (2, b"ab"), (3, b"123"), (1, b"x"));
///
/// assert_eq!(buf, [b'a', b'b', b'1', b'2', b'3', b'x', 0, 0, 0, 0]);
/// # }
/// ```
#[macro_export]
macro_rules! vec_copy_from_concatenating_slices3 {
    ($target:expr, ($a_len:expr, $a:expr), ($b_len:expr, $b:expr), ($c_len:expr, $c:expr)) => {
        $target[..$a_len].copy_from_slice($a);
        $target[$a_len..($a_len + $b_len)].copy_from_slice($b);
        $target[($a_len + $b_len)..($a_len + $b_len + $c_len)].copy_from_slice($c);
    };
}

/// Same as `vec_copy_from_concatenating_slices3` but for four slices.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate extensions;
/// # fn main() {
/// let mut buf = [0; 10];
/// vec_copy_from_concatenating_slices4!(
///     &mut buf,
///     (2, b"ab"),
///     (3, b"123"),
///     (1, b"z"),
///     (4, b"4567")
/// );
///
/// assert_eq!(
///     buf,
///     [b'a', b'b', b'1', b'2', b'3', b'z', b'4', b'5', b'6', b'7']
/// );
/// # }
/// ```
#[macro_export]
macro_rules! vec_copy_from_concatenating_slices4 {
    ($target:expr, ($a_len:expr, $a:expr), ($b_len:expr, $b:expr), ($c_len:expr, $c:expr), ($d_len:expr, $d:expr)) => {
        $target[..$a_len].copy_from_slice($a);
        $target[$a_len..($a_len + $b_len)].copy_from_slice($b);
        $target[($a_len + $b_len)..($a_len + $b_len + $c_len)].copy_from_slice($c);
        $target[($a_len + $b_len + $c_len)..($a_len + $b_len + $c_len + $d_len)]
            .copy_from_slice($d);
    };
}
