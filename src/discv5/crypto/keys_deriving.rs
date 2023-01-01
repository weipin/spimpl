// dest-pubkey        = public key corresponding to node B's static private key
// secret             = ecdh(dest-pubkey, ephemeral-key)
// kdf-info           = "discovery v5 key agreement" || node-id-A || node-id-B
// prk                = HKDF-Extract(secret, challenge-data)
// key-data           = HKDF-Expand(prk, kdf-info)
// initiator-key      = key-data[:16]
// recipient-key      = key-data[16:]

use crate::enr::{NodeId, Scheme};
use crate::utils::vec::vec_copy_from_concatenating_slices2;
use hkdf::Hkdf;
use k256::sha2::Sha256;

pub(crate) const KDF_INFO_PREFIX: &[u8] = b"discovery v5 key agreement";
pub(crate) const KDF_INFO_PREFIX_BYTE_LENGTH: usize = KDF_INFO_PREFIX.len();
pub(crate) const KDF_INFO_BYTE_LENGTH: usize =
    KDF_INFO_PREFIX_BYTE_LENGTH + std::mem::size_of::<NodeId>() * 2;

pub(crate) fn derive_keys<S: Scheme>(
    kdf_info_buffer: &mut [u8; KDF_INFO_BYTE_LENGTH],
    output_keying_material_buffer: &mut [u8; 32],
    dest_pubkey: &S::PublicKey,
    ephemeral_key: &S::PrivateKey,
    node_id_a: &NodeId,
    node_id_b: &NodeId,
    challenge_data: &[u8],
) {
    let shared_secret_data = S::ecdh(&dest_pubkey, &ephemeral_key);
    let hkdf = Hkdf::<Sha256>::new(Some(challenge_data), &shared_secret_data);

    debug_assert!(kdf_info_buffer.starts_with(KDF_INFO_PREFIX));
    vec_copy_from_concatenating_slices2!(
        kdf_info_buffer,
        KDF_INFO_PREFIX_BYTE_LENGTH,
        (32, &node_id_a.0),
        (32, &node_id_b.0)
    );

    hkdf.expand(kdf_info_buffer, output_keying_material_buffer)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::Schemev4;
    use hex_literal::hex;

    #[test]
    fn test_derive_keys() {
        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#key-derivation
        let ephemeral_key_data =
            hex!("fb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736");
        let dest_pubkey_data =
            hex!("0317931e6e0840220642f230037d285d122bc59063221ef3226b1f403ddc69ca91");
        let node_id_a_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let node_id_b_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000");

        let initiator_key = hex!("dccc82d81bd610f4f76d3ebe97a40571");
        let recipient_key = hex!("ac74bb8773749920b0d3a8881c173ec5");

        let ephemeral_key = Schemev4::value_to_private_key(&ephemeral_key_data).unwrap();
        let dest_pubkey = Schemev4::value_to_public_key(&dest_pubkey_data).unwrap();
        let node_id_a = NodeId(node_id_a_data);
        let node_id_b = NodeId(node_id_b_data);
        let mut kdf_info_buffer = [0; KDF_INFO_BYTE_LENGTH];
        kdf_info_buffer[..KDF_INFO_PREFIX_BYTE_LENGTH].copy_from_slice(KDF_INFO_PREFIX);
        let mut output_keying_material_buffer = [0; 32];

        derive_keys::<Schemev4>(
            &mut kdf_info_buffer,
            &mut output_keying_material_buffer,
            &dest_pubkey,
            &ephemeral_key,
            &node_id_a,
            &node_id_b,
            &challenge_data,
        );

        assert_eq!(output_keying_material_buffer[..16], initiator_key);
        assert_eq!(output_keying_material_buffer[16..], recipient_key);
    }

    #[test]
    fn test_ping_handshake_derive_keys() {
        let node_a_key_data =
            hex!("eef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f");
        let node_b_key_data =
            hex!("66fb62bfbd66b9177a138c1e5cddbe4f7c30c343e94e68df8769459cb1cde628");

        let node_a_key = Schemev4::value_to_private_key(&node_a_key_data).unwrap();
        let node_b_key = Schemev4::value_to_private_key(&node_b_key_data).unwrap();

        let node_a_pub_key =
            secp256k1::PublicKey::from_secret_key(secp256k1::SECP256K1, &node_a_key);
        let node_b_pub_key =
            secp256k1::PublicKey::from_secret_key(secp256k1::SECP256K1, &node_b_key);

        let node_a_id = Schemev4::construct_node_id(&node_a_pub_key);
        let node_b_id = Schemev4::construct_node_id(&node_b_pub_key);
        println!(">>> node_a_id: {}", hex::encode(node_a_id.0)); // aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb
        println!(">>> node_b_id: {}", hex::encode(node_b_id.0)); // bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9

        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#packet-encodings
        let ephemeral_key_data =
            hex!("0288ef00023598499cb6c940146d050d2b1fb914198c327f76aad590bead68b6");

        let node_id_a_data =
            hex!("aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb");
        let node_id_b_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let challenge_data = hex!("000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000001");

        let ephemeral_key = Schemev4::value_to_private_key(&ephemeral_key_data).unwrap();
        let dest_pubkey = node_b_pub_key;
        let node_id_a = NodeId(node_id_a_data);
        let node_id_b = NodeId(node_id_b_data);
        let mut kdf_info_buffer = [0; KDF_INFO_BYTE_LENGTH];
        kdf_info_buffer[..KDF_INFO_PREFIX_BYTE_LENGTH].copy_from_slice(KDF_INFO_PREFIX);
        let mut output_keying_material_buffer = [0; 32];

        derive_keys::<Schemev4>(
            &mut kdf_info_buffer,
            &mut output_keying_material_buffer,
            &dest_pubkey,
            &ephemeral_key,
            &node_id_a,
            &node_id_b,
            &challenge_data,
        );

        let initiator_key = &output_keying_material_buffer[..16];
        let recipient_key = &output_keying_material_buffer[16..];
        println!("{}", hex::encode(initiator_key));
        println!("{}", hex::encode(recipient_key));
    }
}
