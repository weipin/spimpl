// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use aes::Aes128;
use cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr128BE;

// aesctr_encrypt(key, iv, pt)
//     is unauthenticated AES/CTR symmetric encryption with the given key and iv.
//     Size of key and iv is 16 bytes (AES-128).
pub(crate) fn encrypt(key: &[u8], iv: &[u8], pt_in_ct_out: &mut [u8]) {
    debug_assert_eq!(key.len(), 16);
    debug_assert_eq!(iv.len(), 16);

    let mut mode = Ctr128BE::<Aes128>::new_from_slices(key, iv).unwrap();
    mode.apply_keystream(pt_in_ct_out);
}
