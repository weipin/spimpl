// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Debug)]
pub struct QuickCheckArray<A: Arbitrary, const N: usize>(pub [A; N]);

impl<const N: usize, A: Arbitrary> Arbitrary for QuickCheckArray<A, N> {
    fn arbitrary(g: &mut Gen) -> QuickCheckArray<A, N> {
        QuickCheckArray([(); N].map(|_| A::arbitrary(g)))
    }
}
