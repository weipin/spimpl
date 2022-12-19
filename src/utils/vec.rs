// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

macro_rules! vec_copy_from_concatenating_slices2 {
    ($target:ident, $starting_index:ident, ($a_len:expr, $a:expr), ($b_len:expr, $b:expr)) => {
        debug_assert_eq!($starting_index + $a_len + $b_len, $target.len());

        $target[$starting_index..$starting_index + $a_len].copy_from_slice($a);
        $target[($starting_index + $a_len)..].copy_from_slice($b);
    };
}

pub(crate) use vec_copy_from_concatenating_slices2;

macro_rules! vec_copy_from_concatenating_slices3 {
    ($target:ident, $starting_index:ident, ($a_len:expr, $a:expr), ($b_len:expr, $b:expr), ($c_len:expr, $c:expr)) => {
        debug_assert_eq!($starting_index + $a_len + $b_len + $c_len, $target.len());

        $target[$starting_index..$starting_index + $a_len].copy_from_slice($a);
        $target[($starting_index + $a_len)..($starting_index + $a_len + $b_len)]
            .copy_from_slice($b);
        $target[($starting_index + $a_len + $b_len)..].copy_from_slice($c);
    };
}

pub(crate) use vec_copy_from_concatenating_slices3;
