// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::record::Record;
use super::scheme::Scheme;
use super::storage::Storage;
use super::storage_content_with_signature_rlp::StorageWithSignatureRlp;
use super::storage_rlp_decoding::RlpDecodingError;
use super::storage_rlp_encoding::RlpEncodingError;

pub(crate) const TEXTUAL_FORM_PREFIX: &str = "enr:";

impl Record {
    pub fn textual_form<S: Scheme>(&self) -> Result<String, RlpEncodingError> {
        self.0.textual_form::<S>()
    }

    pub fn from_textual_form<S: Scheme>(textual_form: &str) -> Result<Record, RlpDecodingError> {
        let storage_with_signature_rlp = StorageWithSignatureRlp::from_textual_form(textual_form)?;
        let storage = Storage::from_rlp::<S>(&mut storage_with_signature_rlp.0.as_slice())?;
        if storage.encode_content_to_rlp::<S>().verify::<S>(
            storage.signature_value.as_ref().unwrap(),
            storage.public_key_value.as_ref().unwrap(),
        )? {
            Ok(Record(storage))
        } else {
            Err(RlpDecodingError::InvalidSignature)
        }
    }
}

impl Storage {
    pub(crate) fn textual_form<S: Scheme>(&self) -> Result<String, RlpEncodingError> {
        let rlp = self.encode_content_with_signature_to_rlp::<S>()?;
        Ok(rlp.to_textual_form())
    }
}

impl StorageWithSignatureRlp {
    pub(crate) fn to_textual_form(&self) -> String {
        // The textual form of a node record is the base64 encoding of its RLP representation,
        // prefixed by enr:
        let base64 = self.to_base64();
        TEXTUAL_FORM_PREFIX.to_string() + &String::from_utf8(base64).unwrap()
    }

    pub(crate) fn from_textual_form(s: &str) -> Result<Self, RlpDecodingError> {
        let base64 = s
            .strip_prefix(TEXTUAL_FORM_PREFIX)
            .ok_or(RlpDecodingError::InvalidFormat)?;
        Self::from_base64(base64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::builder::Builder;
    use crate::enr::testing_helper::EXAMPLE_RECORD_ADDRESS;
    use crate::enr::Schemev4;
    use crate::testing_helper::quickcheck_ip_addr_octets::{Ipv4AddrOctets, Ipv6AddrOctets};
    use fastrlp::DecodeError::InputTooShort;
    use hex_literal::hex;
    use quickcheck_macros::quickcheck;
    use rand::{thread_rng, RngCore};
    use secp256k1::{PublicKey, SecretKey, SECP256K1};
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_textual_double_conversion_with_spec_sample() {
        let result = Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS);
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(
            record.textual_form::<Schemev4>().unwrap(),
            EXAMPLE_RECORD_ADDRESS
        );
    }

    #[test]
    fn test_from_textual_error_cases() {
        let data = [
            ("", RlpDecodingError::InvalidFormat),
            ("enr", RlpDecodingError::InvalidFormat),
            ("enr:", RlpDecodingError::DecodingRLPError(InputTooShort)),
            ("enr:xx", RlpDecodingError::InvalidFormat),
            ("ENR:xx", RlpDecodingError::InvalidFormat),
            (
                concat!(
                    // replaces "...Ay5..." to "...ay5...", making the signature invalid
                    //                      ___
                    "enr:-IS4QHCYrYZbAKWCBRlay5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjz",
                    "CBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
                    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
                ),
                RlpDecodingError::InvalidSignature,
            ),
        ];

        for (s, err) in data {
            let result = Record::from_textual_form::<Schemev4>(s);
            assert_eq!(result.unwrap_err(), err);
        }
    }

    #[test]
    fn test_to_textual_length_exceeding() {
        let private_key = SecretKey::from_slice(&hex!(
            "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
        ))
        .unwrap();
        let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
        let mut record = Builder::new()
            .with_ip4(Ipv4Addr::from(hex!("7f000001")))
            .with_udp4(u16::from_be_bytes(hex!("765f")))
            .sign_and_build::<Schemev4>(&private_key, &public_key)
            .unwrap();
        // big enough dummy signature data
        record.0.signature_value = Some(vec![7; 300]);
        assert_eq!(
            record.textual_form::<Schemev4>().unwrap_err(),
            RlpEncodingError::MaximumEncodedByteLengthExceeded
        );
    }

    #[quickcheck]
    fn test_to_textual_with_sigp_enr(
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

        // sigp enr
        let decoded_record: enr::Enr<enr::secp256k1::SecretKey> = textual_form.parse().unwrap();

        assert_eq!(decoded_record.seq(), seq);
        assert_eq!(decoded_record.ip4(), ip4);
        assert_eq!(decoded_record.tcp4(), tcp4);
        assert_eq!(decoded_record.udp4(), udp4);
        assert_eq!(decoded_record.ip6(), ip6);
        assert_eq!(decoded_record.tcp6(), tcp6);
        assert_eq!(decoded_record.udp6(), udp6);
        true
    }
}
