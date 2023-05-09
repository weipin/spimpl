// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes128;
use ctr::Ctr128BE;

type AesCtr = Ctr128BE<Aes128>;

// aesctr_encrypt(key, iv, pt)
//     is unauthenticated AES/CTR symmetric encryption with the given key and iv.
//     Size of key and iv is 16 bytes (AES-128).
pub(crate) fn encrypt(key: &[u8; 16], iv: &[u8; 16], pt_in_ct_out: &mut [u8]) {
    let mut cipher = new_cipher(key, iv);
    apply_keystream(&mut cipher, pt_in_ct_out);
}

pub(crate) fn new_cipher(key: &[u8; 16], iv: &[u8; 16]) -> AesCtr {
    AesCtr::new_from_slices(key, iv).unwrap()
}

pub(crate) fn apply_keystream(cipher: &mut AesCtr, in_out: &mut [u8]) {
    cipher.apply_keystream(in_out);
}
