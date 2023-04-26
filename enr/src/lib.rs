// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements EIP-778: Ethereum Node Records (ENR).
//! https://eips.ethereum.org/EIPS/eip-778

#![warn(missing_docs)]

pub(crate) mod base64;
mod builder;
pub(crate) mod constants;
pub(crate) mod content;
mod content_rlp_decoding;
mod content_rlp_encoding;
mod content_signing_verifying;
mod error;
pub mod predefined_keys;
mod publishable_record;
mod record;
mod scheme;
mod scheme_keypair;
mod scheme_v4;
#[cfg(feature = "k256")]
mod scheme_v4_k256;
mod scheme_v4_secp256k1;
mod tests;
mod textual_form;
mod types;

pub use crate::base64::base64_engine;
pub use builder::Builder;
pub use constants::MAX_RLP_ENCODED_BYTE_LENGTH;
pub use error::Error;
pub use publishable_record::PublishableRecord;
pub use record::{Record, RecordRlpEncoded};
pub use scheme::Scheme;
pub use scheme_keypair::SchemeKeyPair;
pub use scheme_v4::Schemev4;
#[cfg(feature = "k256")]
pub use scheme_v4_k256::Schemev4K256;
pub use scheme_v4_secp256k1::Schemev4Secp256k1;
pub use types::{NodeId, SequenceNumber};
