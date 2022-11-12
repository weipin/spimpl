// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::record::Record;
use super::scheme::Scheme;
use super::storage::Storage;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct Builder(pub(crate) Storage);

impl Builder {
    pub fn sign_and_build<S: Scheme>(
        &mut self,
        private_key: &S::PrivateKey,
        public_key: &S::PublicKey,
    ) -> Result<Record, S::SigningError> {
        let content_rlp = self
            .0
            .update_public_key_and_encode_content_to_rlp::<S>(public_key);
        let signature = content_rlp.sign::<S>(private_key)?;
        let signature_value = S::signature_to_value(&signature);
        self.with_signature_value(signature_value);
        Ok(Record(self.0.clone()))
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder(Storage::default())
    }

    pub(crate) fn with_signature_value(&mut self, signature: Vec<u8>) -> &mut Self {
        self.0.signature_value = Some(signature);
        self
    }

    pub fn with_seq(&mut self, seq: u64) -> &mut Self {
        self.0.seq = seq;
        self
    }

    pub(crate) fn with_id(&mut self, id: &'static [u8]) -> &mut Self {
        self.0.id = Some(id);
        self
    }

    pub(crate) fn with_public_key_value(&mut self, public_key: Vec<u8>) -> &mut Self {
        self.0.public_key_value = Some(public_key);
        self
    }

    pub fn with_ip4(&mut self, ip4: Ipv4Addr) -> &mut Self {
        self.0.ip4 = Some(ip4);
        self
    }

    pub fn with_tcp4(&mut self, tcp4: u16) -> &mut Self {
        self.0.tcp4 = Some(tcp4);
        self
    }

    pub fn with_udp4(&mut self, udp4: u16) -> &mut Self {
        self.0.udp4 = Some(udp4);
        self
    }

    pub fn with_ip6(&mut self, ip6: Ipv6Addr) -> &mut Self {
        self.0.ip6 = Some(ip6);
        self
    }

    pub fn with_tcp6(&mut self, tcp6: u16) -> &mut Self {
        self.0.tcp6 = Some(tcp6);
        self
    }

    pub fn with_udp6(&mut self, udp6: u16) -> &mut Self {
        self.0.udp6 = Some(udp6);
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
