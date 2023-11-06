// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements the builder pattern for `Record` creation.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::content::Content;
use crate::{Error, Record, Scheme, SchemeKeyPair, SeqNum};

/// Builder for `Content`.
pub struct Builder(pub(crate) Content);

impl Builder {
    /// Creates a new `Record` with the specified key pair.
    pub fn sign_and_build<S: Scheme>(
        &mut self,
        key_pair: &SchemeKeyPair<S>,
    ) -> Result<Record, Error> {
        self.0.public_key_data = Some(S::public_key_to_bytes(key_pair.public_key()));
        let encoded = self.0.to_rlp_encoded::<S>();
        let signature = encoded
            .sign::<S>(key_pair.private_key())
            .map_err(|e| Error::SignatureConstructingFailed(format!("{e}")))?;
        let signature_data = S::signature_to_bytes(&signature);

        Ok(Record {
            signature_data,
            content: self.0.clone(),
        })
    }
}

impl Builder {
    /// Creates a new `Builder`.
    ///
    /// The content id, e.g. "v4", will be immediately set by `S`.
    pub fn new<S: Scheme>() -> Self {
        Self(Content::new(S::id()))
    }

    /// Sets the sequence number `seq`.
    pub fn with_seq(&mut self, seq: SeqNum) -> &mut Self {
        self.0.seq = seq;
        self
    }

    /// Sets the IPv4 address `ip4`.
    pub fn with_ip4(&mut self, ip4: Ipv4Addr) -> &mut Self {
        self.0.ip4 = Some(ip4);
        self
    }

    /// Sets the IPv6 address `ip6`.
    pub fn with_ip6(&mut self, ip6: Ipv6Addr) -> &mut Self {
        self.0.ip6 = Some(ip6);
        self
    }

    /// Sets the TCP port `tcp4`.
    pub fn with_tcp4(&mut self, tcp4: u16) -> &mut Self {
        self.0.tcp4 = Some(tcp4);
        self
    }

    /// Sets the IPv6-specific TCP port `tcp6`.
    pub fn with_tcp6(&mut self, tcp6: u16) -> &mut Self {
        self.0.tcp6 = Some(tcp6);
        self
    }

    /// Sets the UDP port `udp4`.
    pub fn with_udp4(&mut self, udp4: u16) -> &mut Self {
        self.0.udp4 = Some(udp4);
        self
    }

    /// Sets the IPv6-specific UDP port `udp6`.
    pub fn with_udp6(&mut self, udp6: u16) -> &mut Self {
        self.0.udp6 = Some(udp6);
        self
    }
}
