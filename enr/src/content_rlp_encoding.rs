// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements `Content` encoding to its RLP form.

use rlp::{encode, encode_to, ItemPayloadSlice};

use crate::content::{Content, ContentRlpEncoded};
use crate::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use crate::Scheme;

impl Content {
    /// Encodes `self` to its RLP encoded form.
    pub(crate) fn to_rlp_encoded<S: Scheme>(&self) -> ContentRlpEncoded {
        let mut list_payload = vec![];
        self.encode_to_rlp_list_payload::<S>(&mut list_payload);

        let mut encoded = vec![];
        ItemPayloadSlice(&list_payload).encode_as_list(&mut encoded);
        ContentRlpEncoded(encoded)
    }

    /// Encodes `self` to its RLP encoded form and appends the result to
    /// `list_payload`.
    pub(crate) fn encode_to_rlp_list_payload<S: Scheme>(&self, list_payload: &mut Vec<u8>) {
        encode_to(self.seq, list_payload);

        // The key/value pairs must be sorted by key.
        let mut pairs = vec![];
        pairs.push((ID_KEY, encode(self.id)));
        if let Some(ref ip4) = self.ip4 {
            pairs.push((IP4_KEY, encode(ip4)));
        }
        if let Some(ref ip6) = self.ip6 {
            pairs.push((IP6_KEY, encode(ip6)));
        }
        if let Some(ref public_key_bytes) = self.public_key_data {
            pairs.push((S::key_of_public_key(), encode(public_key_bytes)));
        }
        if let Some(tcp4) = self.tcp4 {
            pairs.push((TCP4_KEY, encode(tcp4)));
        }
        if let Some(tcp6) = self.tcp6 {
            pairs.push((TCP6_KEY, encode(tcp6)));
        }
        if let Some(udp4) = self.udp4 {
            pairs.push((UDP4_KEY, encode(udp4)));
        }
        if let Some(udp6) = self.udp6 {
            pairs.push((UDP6_KEY, encode(udp6)));
        }

        pairs.sort_by_key(|k| k.0);
        for (key, rlp_bytes) in pairs {
            encode_to(key, list_payload);
            list_payload.extend(rlp_bytes);
        }
    }
}
