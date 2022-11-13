// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements storage RLP decoding.

use super::builder::Builder;
use super::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use super::scheme::Scheme;
use super::storage::Storage;
use bytes::{Buf, Bytes};
use fastrlp::{Decodable, Header};
use std::net::{Ipv4Addr, Ipv6Addr};

impl Storage {
    pub(crate) fn from_rlp<S: Scheme>(buf: &mut &[u8]) -> Result<Storage, RlpDecodingError> {
        let header = Header::decode(buf).map_err(RlpDecodingError::DecodingRLPError)?;
        if !header.list {
            return Err(RlpDecodingError::InvalidFormat);
        }

        let payload_view = &mut &buf[..header.payload_length];
        if payload_view.is_empty() {
            return Err(RlpDecodingError::EmptyPayload);
        }

        let mut builder = Builder::new();

        // signature
        let signature = Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
        builder.with_signature_value(signature.to_vec());

        // seq
        if payload_view.is_empty() {
            return Err(RlpDecodingError::MissingSeq);
        }
        let seq = u64::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
        builder.with_seq(seq);

        let mut previous_key = Bytes::new();
        // pairs
        while !payload_view.is_empty() {
            let key = Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
            if key <= previous_key {
                // The key/value pairs must be sorted by key and must be unique
                return Err(RlpDecodingError::KeyNotInOrderOrDuplicate);
            }
            previous_key = key.clone();
            match key.as_ref() {
                _ if key == S::public_key_key() => {
                    let public_key_value =
                        Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    builder.with_public_key_value(public_key_value.to_vec());
                }
                ID_KEY => {
                    let id =
                        Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    if id != S::id() {
                        return Err(RlpDecodingError::SchemeIdNotSupported);
                    }
                    builder.with_id(S::id());
                }
                IP4_KEY => {
                    let ip_bytes =
                        Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    let ip_octets: [u8; 4] = ip_bytes
                        .as_ref()
                        .try_into()
                        .map_err(|_| RlpDecodingError::InvalidIp4Octets)?;
                    builder.with_ip4(Ipv4Addr::from(ip_octets));
                }
                TCP4_KEY => {
                    let tcp =
                        u16::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    builder.with_tcp4(tcp);
                }
                UDP4_KEY => {
                    let udp =
                        u16::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    builder.with_udp4(udp);
                }
                IP6_KEY => {
                    let ip6_bytes =
                        Bytes::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    let ip6_octets: [u8; 16] = ip6_bytes
                        .as_ref()
                        .try_into()
                        .map_err(|_| RlpDecodingError::InvalidIp6Octets)?;
                    builder.with_ip6(Ipv6Addr::from(ip6_octets));
                }
                TCP6_KEY => {
                    let tcp6 =
                        u16::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    builder.with_tcp6(tcp6);
                }
                UDP6_KEY => {
                    let udp6 =
                        u16::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    builder.with_udp6(udp6);
                }
                _ => {
                    // unknown pair
                    let header =
                        Header::decode(payload_view).map_err(RlpDecodingError::DecodingRLPError)?;
                    if header.list {
                        return Err(RlpDecodingError::InvalidPair);
                    }
                    payload_view.advance(header.payload_length);
                }
            }
        } // while payload_view.is_empty

        if builder.0.id.is_none() {
            return Err(RlpDecodingError::MissingSchemeId);
        }
        if builder.0.public_key_value.is_none() {
            return Err(RlpDecodingError::MissingPublicKeyValue);
        }

        Ok(builder.0)
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum RlpDecodingError {
    #[error("Decoding RLP failed")]
    DecodingRLPError(#[source] fastrlp::DecodeError),

    #[error("Maximum record size exceeded")]
    MaximumEncodedByteLengthExceeded,

    #[error("Invalid format")]
    InvalidFormat,

    #[error("Empty payload")]
    EmptyPayload,

    #[error("Key not in order or duplicate")]
    KeyNotInOrderOrDuplicate,

    #[error("Missing seq")]
    MissingSeq,

    #[error("Missing scheme id")]
    MissingSchemeId,

    #[error("Missing public key value")]
    MissingPublicKeyValue,

    #[error("Invalid pair")]
    InvalidPair,

    #[error("Scheme id not supported")]
    SchemeIdNotSupported,

    #[error("Invalid ip4 octets")]
    InvalidIp4Octets,

    #[error("Invalid ip6 octets")]
    InvalidIp6Octets,

    #[error("Converting from value to signature failed")]
    InvalidSignatureValue,

    #[error("Converting from value to public key failed")]
    InvalidPublicKeyValue,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Verifying record failed with error")]
    VerifyingError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::base64::URL_SAFE_CONFIG;
    use crate::enr::builder::Builder;
    use crate::enr::textual_form::TEXTUAL_FORM_PREFIX;
    use crate::enr::Schemev4;
    use crate::testing_helper::quickcheck_ip_addr_octets::{Ipv4AddrOctets, Ipv6AddrOctets};
    use ::quickcheck_macros::quickcheck;
    use base64::decode_engine_slice;
    use hex_literal::hex;
    use rand::thread_rng;
    use rand::RngCore;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_decoding_with_spec_sample_node() {
        let bytes = hex!("f884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f").to_vec();
        let storage = Storage::from_rlp::<Schemev4>(&mut bytes.as_slice()).unwrap();

        assert_eq!(
            storage.signature_value.unwrap(),
            hex!("7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c")
        );
        assert_eq!(storage.seq, 1);
        assert_eq!(storage.id.unwrap(), b"v4");
        assert_eq!(storage.ip4.unwrap(), Ipv4Addr::from([127, 0, 0, 1]));
        assert_eq!(
            storage.public_key_value.unwrap(),
            hex!("03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138")
        );
        assert_eq!(storage.udp4.unwrap(), 30303);
    }

    #[quickcheck]
    fn test_decoding_double_conversion(
        seq: u64,
        ip4_octets: Option<Ipv4AddrOctets>,
        tcp4: Option<u16>,
        udp4: Option<u16>,
        ip6_octets: Option<Ipv6AddrOctets>,
        tcp6: Option<u16>,
        udp6: Option<u16>,
    ) -> bool {
        let mut key_data = [0u8; 32];
        let mut rng = thread_rng();
        rng.fill_bytes(&mut key_data);

        let key = secp256k1::SecretKey::from_slice(&key_data).unwrap();
        let public_key = secp256k1::PublicKey::from_secret_key(secp256k1::SECP256K1, &key);
        let ip4 = ip4_octets.map(|octets| Ipv4Addr::from(octets.0));
        let ip6 = ip6_octets.map(|octets| Ipv6Addr::from(octets.0));

        let mut builder = Builder::new();
        builder.with_seq(seq);
        if let Some(ip4) = ip4 {
            builder.with_ip4(ip4);
        }
        if let Some(tcp4) = tcp4 {
            builder.with_tcp4(tcp4);
        }
        if let Some(udp4) = udp4 {
            builder.with_udp4(udp4);
        }
        if let Some(ip6) = ip6 {
            builder.with_ip6(ip6);
        }
        if let Some(tcp6) = tcp6 {
            builder.with_tcp6(tcp6);
        }
        if let Some(udp6) = udp6 {
            builder.with_udp6(udp6);
        }
        let record = builder
            .sign_and_build::<Schemev4>(&key, &public_key)
            .unwrap();
        let textual_form = record.textual_form::<Schemev4>().unwrap();
        let base64 = textual_form.strip_prefix(TEXTUAL_FORM_PREFIX).unwrap();
        let mut rlp_data = vec![0; 1024];
        let size = decode_engine_slice(base64, &mut rlp_data, &URL_SAFE_CONFIG).unwrap();

        let storage = Storage::from_rlp::<Schemev4>(&mut &rlp_data[0..size]).unwrap();
        assert_eq!(storage.seq, seq);
        assert_eq!(storage.id.unwrap(), b"v4");
        assert_eq!(
            storage.public_key_value.unwrap(),
            Schemev4::public_key_to_value(&public_key)
        );
        assert_eq!(storage.ip4, builder.0.ip4);
        assert_eq!(storage.tcp4, builder.0.tcp4);
        assert_eq!(storage.udp4, builder.0.udp4);
        assert_eq!(storage.ip6, builder.0.ip6);
        assert_eq!(storage.tcp6, builder.0.tcp6);
        assert_eq!(storage.udp6, builder.0.udp6);

        true
    }

    #[test]
    fn test_decoding_errors() {
        let data = [
            ("01", RlpDecodingError::InvalidFormat),
            ("c3", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::InputTooShort)),
            ("d291636f6e74656e745f7369676e6174757265", RlpDecodingError::MissingSeq),
            ("dc91636f6e74656e745f7369676e617475726589020000000000000000", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::Overflow)), // invalid seq
            ("d991636f6e74656e745f7369676e617475726501826964827636", RlpDecodingError::SchemeIdNotSupported),
            ("dc91636f6e74656e745f7369676e617475726501826970851234567800", RlpDecodingError::InvalidIp4Octets),
            ("db91636f6e74656e745f7369676e6174757265018374637083100000", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::Overflow)), // invalid tcp
            ("db91636f6e74656e745f7369676e6174757265018375647083100000", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::Overflow)), // invalid udp
            ("e991636f6e74656e745f7369676e61747572650183697036911234567812345678123456781234567800", RlpDecodingError::InvalidIp6Octets),
            ("dc91636f6e74656e745f7369676e617475726501847463703683100000", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::Overflow)), // invalid tcp6
            ("dc91636f6e74656e745f7369676e617475726501847564703683100000", RlpDecodingError::DecodingRLPError(fastrlp::DecodeError::Overflow)), // invalid udp6
            ("d391636f6e74656e745f7369676e617475726501", RlpDecodingError::MissingSchemeId),
            ("d991636f6e74656e745f7369676e617475726501826964827634", RlpDecodingError::MissingPublicKeyValue),
            ("e991636f6e74656e745f7369676e6174757265018269648276348b756e6b6e6f776e5f6b6579c3010203", RlpDecodingError::InvalidPair),
        ];
        for (rlp_data_hex, err) in data {
            let rlp_data = hex::decode(rlp_data_hex).unwrap();
            let result = Storage::from_rlp::<Schemev4>(&mut rlp_data.as_slice());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), err);
        }
    }

    #[test]
    fn test_decoding_with_unknown_pair() {
        let bytes =
            hex!("f895b840d8b6cfc15b0174b5df681f6c5b9e1904b9c8b315aad73a1f02e19ed33dec3a760bb245536f04b70c005a1a5d7ff6dfc591fe820cd866f23653294f23eb312c3301826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138837463700184746370360283756470038475647036047a02")
                .to_vec();
        assert!(Storage::from_rlp::<Schemev4>(&mut bytes.as_slice()).is_ok())
    }

    #[test]
    fn test_decoding_reject_unsorted_or_duplicate() {
        let data = [
            "f87fb840f4471c3d71d26487aa50e33d668771a5a67c3d883a8c49dd07fc7f59e77758591f0c6dda9828857996ba9ae971b613de053cea5b9c329477e3d132552c876b2f018269648276347a02826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138", // unsorted
            "f885b840bd74ad9559fd2822b0876324a4ace68898a9c265318cec284bac0435e091ec4914388e9bbb752d2a5fdf386c071ceca1a598d895d26a3995c112af9977e33e9501826964827634826970847f000001826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138" // duplicate
        ];
        for rlp_data_s in data {
            let rlp_data = hex::decode(rlp_data_s).unwrap();
            let result = Storage::from_rlp::<Schemev4>(&mut rlp_data.as_slice());
            assert_eq!(
                result.unwrap_err(),
                RlpDecodingError::KeyNotInOrderOrDuplicate
            );
        }
    }
}
