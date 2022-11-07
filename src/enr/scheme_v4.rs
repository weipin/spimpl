use super::scheme::Scheme;
use rand::RngCore;
use secp256k1::{ecdsa, Message, SECP256K1};
use sha3::{Digest, Keccak256};

#[cfg(test)]
use self::MockOsRng as OsRng;
#[cfg(not(test))]
use rand::rngs::OsRng;

pub struct Schemev4;

impl Scheme for Schemev4 {
    type PrivateKey = secp256k1::SecretKey;
    type PublicKey = secp256k1::PublicKey;
    type Signature = ecdsa::Signature;
    type SigningError = secp256k1::Error;
    type VerifyingError = secp256k1::Error;

    fn id() -> &'static str {
        "v4"
    }

    fn public_key_key() -> &'static str {
        "secp256k1"
    }

    fn value_to_public_key(value: &[u8]) -> Option<Self::PublicKey> {
        secp256k1::PublicKey::from_slice(value).ok()
    }

    fn public_key_to_value(public_key: &Self::PublicKey) -> Vec<u8> {
        public_key.serialize().to_vec()
    }

    fn value_to_signature(value: &[u8]) -> Option<Self::Signature> {
        ecdsa::Signature::from_compact(value).ok()
    }

    fn signature_to_value(signature: &Self::Signature) -> Vec<u8> {
        signature.serialize_compact().to_vec()
    }

    fn sign(
        hash: &[u8],
        private_key: &Self::PrivateKey,
    ) -> Result<Self::Signature, Self::SigningError> {
        let msg = Message::from_slice(hash)?;
        let signature = {
            let mut noncedata = [0; 32];
            OsRng.fill_bytes(&mut noncedata);
            SECP256K1.sign_ecdsa_with_noncedata(&msg, private_key, &noncedata)
        };

        Ok(signature)
    }

    fn verify(
        hash: &[u8],
        signature: &Self::Signature,
        public_key: &Self::PublicKey,
    ) -> Result<bool, Self::VerifyingError> {
        let msg = Message::from_slice(hash)?;
        Ok(SECP256K1.verify_ecdsa(&msg, signature, public_key).is_ok())
    }

    fn construct_node_id(public_key: &Self::PublicKey) -> String {
        // keccak256(x || y)
        // uncompressed keys are 65 bytes, consisting of constant prefix (0x04)
        let uncompressed = &public_key.serialize_uncompressed()[1..];
        let hash = Keccak256::digest(uncompressed);
        hex::encode(hash)
    }
}

#[cfg(test)]
const MOCK_ECDSA_NONCE_ADDITIONAL_DATA: [u8; 32] = [
    // 0xbaaaaaad...
    0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad,
    0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad, 0xba, 0xaa, 0xaa, 0xad,
];

#[cfg(test)]
struct MockOsRng;

#[cfg(test)]
impl RngCore for MockOsRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!();
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!();
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        debug_assert_eq!(dest.len(), MOCK_ECDSA_NONCE_ADDITIONAL_DATA.len());
        dest.copy_from_slice(&MOCK_ECDSA_NONCE_ADDITIONAL_DATA);
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), rand::Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_construct_node_id_with_spec_example_record() {
        let key_data = hex!("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291");
        let key = secp256k1::SecretKey::from_slice(&key_data).unwrap();
        let public_key = key.public_key(SECP256K1);
        let node_id = Schemev4::construct_node_id(&public_key);

        assert_eq!(
            node_id,
            "a448f24c6d18e575453db13171562b71999873db5b286df957af199ec94617f7"
        );
    }
}
