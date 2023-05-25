// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem::size_of;

use enr::{NodeId, Scheme};
use extensions::vec_copy_from_concatenating_slices4;
use sha2::{Digest, Sha256};

use crate::packet::constants::{
    CHALLENGE_DATA_BYTE_LENGTH, ID_SIGNATURE_TEXT, ID_SIGNATURE_TEXT_BYTE_LENGTH,
};

pub(crate) fn build_id_signature<S: Scheme>(
    id_signature_input_buf: &mut [u8],
    challenge_data: &[u8; CHALLENGE_DATA_BYTE_LENGTH],
    ephemeral_pubkey_data: &[u8],
    node_id_b: &enr::NodeId,
    private_key: &S::PrivateKey,
) -> Result<S::Signature, S::Error> {
    debug_assert_eq!(
        id_signature_input_buf.len(),
        id_signature_input_byte_length::<S>()
    );
    debug_assert_eq!(
        ephemeral_pubkey_data.len(),
        S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH
    );

    vec_copy_from_concatenating_slices4!(
        id_signature_input_buf,
        (ID_SIGNATURE_TEXT_BYTE_LENGTH, ID_SIGNATURE_TEXT),
        (CHALLENGE_DATA_BYTE_LENGTH, challenge_data),
        (
            S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH,
            ephemeral_pubkey_data
        ),
        (size_of::<enr::NodeIdType>(), &node_id_b.0)
    );

    let mut hasher = Sha256::new();
    hasher.update(id_signature_input_buf);
    let digest = hasher.finalize();

    Ok(S::sign(&digest, private_key)?)
}

// id_sign(hash) creates a signature over hash using the node's static private key.
// The signature is encoded as the 64-byte array r || s, i.e. as the concatenation of the signature values.

// id-signature-text  = "discovery v5 identity proof"
// id-signature-input = id-signature-text || challenge-data || ephemeral-pubkey || node-id-B
// id-signature       = id_sign(sha256(id-signature-input))
const fn id_signature_input_byte_length<S: Scheme>() -> usize {
    ID_SIGNATURE_TEXT_BYTE_LENGTH
        + CHALLENGE_DATA_BYTE_LENGTH
        + S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH
        + size_of::<NodeIdType>()
}

pub(crate) const ID_SIGNATURE_TEXT: &[u8] = b"discovery v5 identity proof";
pub(crate) const ID_SIGNATURE_TEXT_BYTE_LENGTH: usize = ID_SIGNATURE_TEXT.len();

// challenge-data     = masking-iv || static-header || authdata
pub(crate) const CHALLENGE_DATA_BYTE_LENGTH: usize =
    size_of::<MaskingIvType>() + STATIC_HEADER_BYTE_LENGTH + WHOAREYOU_AUTHDATA_SIZE as usize;

#[cfg(test)]
mod tests {
    use enr::Schemev4;
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_construct_id_signature() {
        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#id-nonce-signing
        let static_key_data =
            hex!("fb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000");
        let ephemeral_pubkey_data =
            hex!("039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231");
        let node_id_b_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");

        let static_key = Schemev4::new_private_key_from_bytes(&static_key_data).unwrap();

        let mut id_signature_input_buf = [0; id_signature_input_byte_length::<Schemev4>()];
        let signature = build_id_signature::<Schemev4>(
            &mut id_signature_input_buf,
            &challenge_data,
            &ephemeral_pubkey_data,
            &enr::NodeId::from_slice(&node_id_b_data),
            &static_key,
        )
        .unwrap();

        // discv5_id_signature: `id_nonce_signing_example_hash`
        let input_hash = hex!("f8b18af81856bc494b09db6930f1feebc8bbebc5c1738c58ed5dc1281618ddc0");
        let pubkey = Schemev4::new_public_key_from_private_key(&static_key);

        // Checks the input hash is correct.
        // This can be done by verifying the example signature against the hash.
        let example_id_signature_data = hex!("94852a1e2318c4e5e9d422c98eaf19d1d90d876b29cd06ca7cb7546d0fff7b484fe86c09a064fe72bdbef73ba8e9c34df0cd2b53e9d65528c2c7f336d5dfc6e6");
        let example_id_signature =
            Schemev4::new_signature_from_bytes(&example_id_signature_data).unwrap();
        assert!(Schemev4::verify(&input_hash, &example_id_signature, &pubkey).unwrap());

        // Checks the result from `build_id_signature` is correct by verifying
        // the signature against the input hash. Cannot directly use
        // `id-signature` from the example, for the ECDSA signing involves extra
        // entropy.
        assert!(Schemev4::verify(&input_hash, &signature, &pubkey).unwrap());
    }
}
