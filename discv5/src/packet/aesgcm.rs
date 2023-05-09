// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use aes_gcm::{AeadInPlace, Aes128Gcm, KeyInit};

// aesgcm_encrypt(key, nonce, pt, ad)
//  is AES-GCM encryption/authentication with the given key, nonce and additional
//  authenticated data ad. Size of key is 16 bytes (AES-128), size of nonce 12 bytes.
//
// pt_in_ct_out.capacity() == pt.len() + TAG_BYTE_LENGTH
pub(crate) fn encrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    ad: &[u8],
    pt_in_ct_out: &mut Vec<u8>,
) -> bool {
    debug_assert!(!ad.is_empty());

    let cipher = Aes128Gcm::new_from_slice(key).unwrap();
    let nonce = aes_gcm::Nonce::from_slice(nonce);

    cipher.encrypt_in_place(nonce, ad, pt_in_ct_out).is_ok()
}

pub(crate) fn decrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    ad: &[u8],
    ct_in_pt_out: &mut Vec<u8>,
) -> bool {
    debug_assert!(!ad.is_empty());

    let cipher = Aes128Gcm::new_from_slice(key).unwrap();
    let nonce = aes_gcm::Nonce::from_slice(nonce);

    cipher.decrypt_in_place(nonce, ad, ct_in_pt_out).is_ok()
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#encryptiondecryption
        let encryption_key = hex!("9f2d77db7004bf8a1a85107ac686990b");
        let nonce = hex!("27b5af763c446acd2749fe8e");
        let pt = hex!("01c20101");
        let ad = hex!("93a7400fa0d6a694ebc24d5cf570f65d04215b6ac00757875e3f3a5f42107903");
        let message_ciphertext = hex!("a5d12a2d94b8ccb3ba55558229867dc13bfa3648");

        let mut pt_in_ct_out = pt.to_vec();
        assert!(encrypt(&encryption_key, &nonce, &ad, &mut pt_in_ct_out));
        assert_eq!(pt_in_ct_out, &message_ciphertext);

        decrypt(&encryption_key, &nonce, &ad, &mut pt_in_ct_out);
        assert_eq!(pt_in_ct_out, pt);
    }
}
