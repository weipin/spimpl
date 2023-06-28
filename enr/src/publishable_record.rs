// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements `PublishableRecord`, the mutable version of `Record`.

use std::net::Ipv4Addr;

use crate::content::{Content, ContentRlpEncoded};
use crate::{Error, Record, Scheme, SequenceNumber};

/// Mutable record.
pub struct PublishableRecord {
    content: Content,

    // "Cached" RLP encoded content.
    //
    // By comparing it with the current status, we can detect if any change has
    // been made.
    content_encoded: ContentRlpEncoded,
}

impl PublishableRecord {
    /// Creates a new `PublishableRecord` from `Record`.
    pub fn from_record<S: Scheme>(record: Record) -> Self {
        let content_encoded = record.content.to_rlp_encoded::<S>();

        Self {
            content: record.content,
            content_encoded,
        }
    }

    /// Returns a tuple with:
    /// - the sequence number of the record
    /// - the textual form of the record
    ///
    /// The sequence number increases by 1 if the record content has any change
    /// since its creation or the previous `publish` call.
    ///
    /// `private_key` must be the same key that constructed the original
    /// immutable record.
    ///
    /// This method always performs a RLP encoding operation upon the content.
    /// If you are certain that the content doesn't change since the last
    /// publishing, consider caching the previously returned values.
    ///
    /// # Panics
    ///
    /// Will panic in debug mode if the public key of `private_key` doesn't
    /// match the existing one stored in the content. Will return an invalid
    /// record in release mode.
    pub fn publish<S: Scheme>(
        &mut self,
        private_key: &S::PrivateKey,
    ) -> Result<(SequenceNumber, String), Error> {
        debug_assert!(self.content.public_key_data.is_some());
        debug_assert_eq!(
            &S::public_key_to_bytes(&S::new_public_key_from_private_key(private_key)),
            self.content.public_key_data.as_ref().unwrap()
        );

        let encoded = self.content.to_rlp_encoded::<S>();
        if encoded != self.content_encoded {
            self.content.seq = self.content.seq.checked_add(1).ok_or(Error::SeqOverflow)?;
            self.content_encoded = self.content.to_rlp_encoded::<S>();
        }

        let signature = self
            .content_encoded
            .sign::<S>(private_key)
            .map_err(|e| Error::SignatureConstructingFailed(format!("{e}")))?;
        let signature_data = S::signature_to_bytes(&signature);
        let textual_form = Record {
            signature_data,
            content: self.content.clone(),
        }
        .to_textual_form::<S>()?;

        Ok((self.content.seq, textual_form))
    }
}

impl PublishableRecord {
    /// Sets IPv4 address `ip4`.
    pub fn update_ip4(&mut self, ip4: Ipv4Addr) {
        self.content.ip4 = Some(ip4);
    }

    /// Sets UDP port `udp4`.
    pub fn update_udp4(&mut self, udp4: u16) {
        self.content.udp4 = Some(udp4);
    }
}

impl Record {
    /// Converts to a `PublishableRecord`.
    pub fn to_publishable<S: Scheme>(self) -> PublishableRecord {
        PublishableRecord::from_record::<S>(self)
    }
}
