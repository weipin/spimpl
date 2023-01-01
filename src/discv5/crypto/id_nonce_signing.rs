// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::FixedSizeAuthDataSource;
use crate::discv5::message::protocol::whoareyou::Whoareyou;
use crate::discv5::packet::masked_header::MaskingIv;
use crate::discv5::packet::static_header::{StaticHeaderData, STATIC_HEADER_DATA_BYTE_LENGTH};
use crate::enr;
use crate::enr::Scheme;
use crate::utils::vec::vec_copy_from_concatenating_slices3;
use sha2::{Digest, Sha256};
use std::mem;

// challenge-data     = masking-iv || static-header || authdata
pub(crate) const CHALLENGE_DATA_BYTE_LENGTH: usize =
    mem::size_of::<MaskingIv>() + STATIC_HEADER_DATA_BYTE_LENGTH + Whoareyou::SIZE as usize;

pub(crate) const ID_SIGNATURE_TEXT: &[u8] = b"discovery v5 identity proof";
pub(crate) const ID_SIGNATURE_TEXT_BYTE_LENGTH: usize = ID_SIGNATURE_TEXT.len();

// id_sign(hash) creates a signature over hash using the node's static private key.
// The signature is encoded as the 64-byte array r || s, i.e. as the concatenation of the signature values.

// id-signature-text  = "discovery v5 identity proof"
// id-signature-input = id-signature-text || challenge-data || ephemeral-pubkey || node-id-B
// id-signature       = id_sign(sha256(id-signature-input))

pub(crate) const fn id_signature_input_byte_length<S: Scheme>() -> usize {
    ID_SIGNATURE_TEXT_BYTE_LENGTH
        + CHALLENGE_DATA_BYTE_LENGTH
        + S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH
        + mem::size_of::<enr::NodeId>()
}

pub(crate) fn id_sign<S: Scheme>(
    id_signature_input_buffer: &mut [u8],
    challenge_data: &[u8; CHALLENGE_DATA_BYTE_LENGTH],
    ephemeral_pubkey_data: &[u8],
    node_id_b: &enr::NodeId,
    private_key: &S::PrivateKey,
) -> Result<Vec<u8>, S::SigningError> {
    debug_assert_eq!(
        id_signature_input_buffer.len(),
        id_signature_input_byte_length::<S>()
    );
    debug_assert_eq!(
        ephemeral_pubkey_data.len(),
        S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH
    );

    debug_assert!(id_signature_input_buffer.starts_with(ID_SIGNATURE_TEXT));

    vec_copy_from_concatenating_slices3!(
        id_signature_input_buffer,
        ID_SIGNATURE_TEXT_BYTE_LENGTH,
        (CHALLENGE_DATA_BYTE_LENGTH, challenge_data),
        (
            S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH,
            ephemeral_pubkey_data
        ),
        (mem::size_of::<enr::NodeId>(), &node_id_b.0)
    );

    let mut hasher = Sha256::new();
    hasher.update(id_signature_input_buffer);
    let digest = hasher.finalize();

    let signature = S::sign(&digest, private_key)?;
    Ok(S::signature_to_value(&signature))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::Schemev4;
    use hex_literal::hex;
    use secp256k1::SECP256K1;

    #[test]
    fn test_id_sign() {
        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#id-nonce-signing
        let static_key_data =
            hex!("fb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000");
        let ephemeral_pubkey_data =
            hex!("039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231");
        let node_id_b_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let id_signature_data = hex!("94852a1e2318c4e5e9d422c98eaf19d1d90d876b29cd06ca7cb7546d0fff7b484fe86c09a064fe72bdbef73ba8e9c34df0cd2b53e9d65528c2c7f336d5dfc6e6");

        let mut id_signature_input_buffer = [0; id_signature_input_byte_length::<Schemev4>()];
        id_signature_input_buffer[..ID_SIGNATURE_TEXT_BYTE_LENGTH]
            .copy_from_slice(ID_SIGNATURE_TEXT);
        let mut id_signature_output_buffer = [0; Schemev4::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH];

        let static_key = Schemev4::value_to_private_key(&static_key_data).unwrap();
        let result = id_sign::<Schemev4>(
            &mut id_signature_input_buffer,
            &challenge_data,
            &ephemeral_pubkey_data,
            &enr::NodeId(node_id_b_data),
            &static_key,
        )
        .unwrap();

        // != id_signature_data, for ECDSA signing with additional data

        let input_hash = hex!("f8b18af81856bc494b09db6930f1feebc8bbebc5c1738c58ed5dc1281618ddc0");

        // First, we ensure `input_hash` is correct by verifying it with the `id-signature` from discv5-wire-test-vectors.md.
        let public_key = static_key.public_key(SECP256K1);
        let signature = Schemev4::value_to_signature(&id_signature_data).unwrap();
        assert!(Schemev4::verify(&input_hash, &signature, &public_key).unwrap());

        // Now we verify the result with `input_hash`.
        let signature = Schemev4::value_to_signature(&result).unwrap();
        assert!(Schemev4::verify(&input_hash, &signature, &public_key).unwrap());
    }
}
