// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements `Record`.

use std::net::{Ipv4Addr, Ipv6Addr};

use rlp::{ItemDataSlice, ItemPayloadSlice};

use crate::constants::MAX_RLP_ENCODED_BYTE_LENGTH;
use crate::content::{Content, ContentRlpEncoded};
use crate::{Error, Scheme, SequenceNumber};

/// Represents a node record.
///
/// To create a `Record`, use `Builder`.
#[derive(Debug, PartialEq)]
pub struct Record {
    pub(crate) signature_data: Vec<u8>,
    pub(crate) content: Content,
}

/// Represents the RLP encoded form of a `Record`.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordRlpEncoded(Vec<u8>);

// Rlp-encoded as it is.
impl rlp::Encode for &RecordRlpEncoded {
    fn encode_to(self, output: &mut Vec<u8>) {
        output.extend(&self.0);
    }
}

impl RecordRlpEncoded {
    /// Creates a `RecordRlpEncoded` from a byte vector.
    ///
    /// Will panic if the byte length of the vector is greater than
    /// `MAX_RLP_ENCODED_BYTE_LENGTH`.
    pub fn from_vec(vec: Vec<u8>) -> Self {
        assert!(vec.len() <= MAX_RLP_ENCODED_BYTE_LENGTH);

        RecordRlpEncoded(vec)
    }

    /// Returns a reference to the RLP-encoded record.
    pub fn rlp_encoded(&self) -> &Vec<u8> {
        &self.0
    }
}

impl Record {
    /// Encodes a `Record` to its RLP encoded form.
    pub fn to_rlp_encoded<S: Scheme>(&self) -> Result<RecordRlpEncoded, Error> {
        let mut list_payload = vec![];
        rlp::encode_to(&self.signature_data, &mut list_payload);
        self.content
            .encode_to_rlp_list_payload::<S>(&mut list_payload);

        let mut encoded = vec![];
        ItemPayloadSlice(&list_payload).encode_as_list(&mut encoded);
        if encoded.len() > MAX_RLP_ENCODED_BYTE_LENGTH {
            return Err(Error::MaximumRecordRlpEncodedByteLengthExceeded);
        }

        Ok(RecordRlpEncoded(encoded))
    }

    /// Creates a `Record` from its RLP encoded form.
    pub fn from_rlp_encoded<S: Scheme>(record_encoded: &RecordRlpEncoded) -> Result<Self, Error> {
        let mut list_iter = ItemDataSlice(record_encoded.0.as_slice())
            .list_iter()
            .map_err(Error::RLPDecodingError)?;
        let signature_data: Vec<u8> = list_iter.next_item().map_err(Error::RLPDecodingError)?;
        if signature_data.len() != S::ENR_REQUIRED_SIGNATURE_BYTE_LENGTH {
            return Err(Error::SignatureDataWithInvalidByteLength);
        }
        let signature = S::new_signature_from_bytes(&signature_data)
            .map_err(|e| Error::InvalidSignatureData(format!("{e}")))?;

        let content_rlp_items = list_iter.remaining_list_payload();

        let content = Content::from_rlp_list_iter::<S>(&mut list_iter)?;
        let public_key_data = content
            .public_key_data
            .as_ref()
            .ok_or(Error::SignatureVerifyingFailedForMissingPublicKey)?;
        let public_key = S::new_public_key_from_bytes(public_key_data)
            .map_err(|e| Error::InvalidPublicKeyData(format!("{e}")))?;

        let mut output = vec![];
        content_rlp_items.encode_as_list(&mut output);
        let content_encoded = ContentRlpEncoded(output);
        let result = content_encoded
            .verify::<S>(&signature, &public_key)
            .map_err(|e| Error::SignatureVerifyingFailed(format!("{e}")))?;
        if !result {
            return Err(Error::InvalidSignature);
        }

        Ok(Self {
            signature_data,
            content,
        })
    }
}

impl Record {
    /// Returns the name of identity scheme.
    pub fn id(&self) -> &'static [u8] {
        self.content.id
    }

    /// Returns the sequence number.
    pub fn seq(&self) -> SequenceNumber {
        self.content.seq
    }

    /// Returns the optional IPv4 address.
    pub fn ip4(&self) -> Option<Ipv4Addr> {
        self.content.ip4
    }

    /// Returns the optional TCP port.
    pub fn tcp4(&self) -> Option<u16> {
        self.content.tcp4
    }

    /// Returns the optional UDP port.
    pub fn udp4(&self) -> Option<u16> {
        self.content.udp4
    }

    /// Returns the optional IPv6 address.
    pub fn ip6(&self) -> Option<Ipv6Addr> {
        self.content.ip6
    }

    /// Returns the optional IPv6-specific TCP port.
    pub fn tcp6(&self) -> Option<u16> {
        self.content.tcp6
    }

    /// Returns the optional IPv6-specific UDP port.
    pub fn udp6(&self) -> Option<u16> {
        self.content.udp6
    }
}
