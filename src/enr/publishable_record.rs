// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::record::Record;
use super::scheme::Scheme;
use super::storage::Storage;
use super::storage_content_rlp::StorageContentRlp;
use super::storage_rlp_encoding::RlpEncodingError;
use super::types::SequenceNumber;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct PublishableRecord {
    storage: Storage,
    previous_storage_content_rlp: Option<StorageContentRlp>,
}

impl PublishableRecord {
    fn from_record(record: Record) -> PublishableRecord {
        PublishableRecord {
            storage: record.0,
            previous_storage_content_rlp: None,
        }
    }

    pub fn update_ip4(&mut self, ip4: Ipv4Addr) {
        self.storage.ip4 = Some(ip4);
    }

    pub fn update_ip6(&mut self, ip6: Ipv6Addr) {
        self.storage.ip6 = Some(ip6);
    }

    pub fn publish<S: Scheme>(&mut self) -> Result<(SequenceNumber, String), RlpEncodingError> {
        let content_rlp = self.storage.encode_content_to_rlp::<S>();
        match self.previous_storage_content_rlp {
            None => {
                self.previous_storage_content_rlp = Some(content_rlp);
            }
            Some(ref previous_storage_content_rlp) => {
                if content_rlp != *previous_storage_content_rlp {
                    self.storage.seq = self.storage.seq.checked_add(1).unwrap();
                    let new_content_rlp = self.storage.encode_content_to_rlp::<S>();
                    self.previous_storage_content_rlp = Some(new_content_rlp);
                }
            }
        }

        let textual_form = self.storage.textual_form::<S>()?;
        Ok((self.storage.seq, textual_form))
    }
}

impl Record {
    pub fn to_publishable(self) -> PublishableRecord {
        PublishableRecord::from_record(self)
    }
}

impl PublishableRecord {
    pub fn ip4(&self) -> Option<Ipv4Addr> {
        self.storage.ip4
    }

    pub fn tcp4(&self) -> Option<u16> {
        self.storage.tcp4
    }

    pub fn udp4(&self) -> Option<u16> {
        self.storage.udp4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::testing_helper::MOCKED_EXAMPLE_RECORD_ADDRESS;
    use crate::enr::{Builder, Schemev4};
    use hex_literal::hex;
    use secp256k1::{PublicKey, SecretKey, SECP256K1};

    #[test]
    fn test_publishable() {
        let private_key = SecretKey::from_slice(&hex!(
            "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
        ))
        .unwrap();
        let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
        let ip4 = Ipv4Addr::from(hex!("7f000001"));
        let udp4 = u16::from_be_bytes(hex!("765f"));

        let mut publishable_record = Builder::new()
            .with_ip4(ip4)
            .with_udp4(udp4)
            .sign_and_build::<Schemev4>(&private_key, &public_key)
            .unwrap()
            .to_publishable();

        let (seq1, textual_form1) = publishable_record.publish::<Schemev4>().unwrap();
        assert_eq!(seq1, 1);
        assert_eq!(textual_form1, MOCKED_EXAMPLE_RECORD_ADDRESS);

        let (seq2, textual_form2) = publishable_record.publish::<Schemev4>().unwrap();
        assert_eq!(seq1, seq2);
        assert_eq!(textual_form1, textual_form2);

        publishable_record.update_ip4(Ipv4Addr::from(hex!("7f000002")));
        let (seq3, textual_form3) = publishable_record.publish::<Schemev4>().unwrap();
        assert_eq!(seq3, 2);
        assert_ne!(textual_form2, textual_form3);
    }
}
