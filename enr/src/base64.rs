// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Base64 encoding and decoding

use base64::engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig};
use base64::{alphabet, Engine};

use crate::constants::{MAXIMUM_BASE64_ENCODED_BYTE_LENGTH, MAXIMUM_RLP_ENCODED_BYTE_LENGTH};
use crate::{Error, RecordRlpEncoded};

impl RecordRlpEncoded {
    /// Encodes a `RecordRlpEncoded` to its base64 from.
    pub(crate) fn to_base64(&self) -> Vec<u8> {
        let mut output = vec![0; MAXIMUM_BASE64_ENCODED_BYTE_LENGTH];
        let size = BASE64_ENGINE.encode_slice(&self.0, &mut output).unwrap();
        output.truncate(size);

        output
    }

    /// Creates a `RecordRlpEncoded` from its base64 form `s`.
    pub(crate) fn from_base64(s: &str) -> Result<Self, Error> {
        if s.len() > MAXIMUM_BASE64_ENCODED_BYTE_LENGTH {
            return Err(Error::MaximumRecordRlpEncodedByteLengthExceeded);
        }

        let mut output = vec![0; MAXIMUM_RLP_ENCODED_BYTE_LENGTH];
        let size = BASE64_ENGINE
            .decode_slice(s, &mut output)
            .map_err(|_| Error::DecodingFailedForInvalidInput)?;
        if size > MAXIMUM_RLP_ENCODED_BYTE_LENGTH {
            return Err(Error::MaximumRecordRlpEncodedByteLengthExceeded);
        }
        output.truncate(size);

        Ok(Self(output))
    }
}

/// The base64 engine with options as per spec.
pub static BASE64_ENGINE: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_allow_trailing_bits(false)
        .with_decode_padding_mode(DecodePaddingMode::RequireNone),
);
