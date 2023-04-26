// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Base64 encoding and decoding

use base64::Engine;

use crate::constants::{MAX_BASE64_ENCODED_BYTE_LENGTH, MAX_RLP_ENCODED_BYTE_LENGTH};
use crate::{Error, RecordRlpEncoded};

impl RecordRlpEncoded {
    /// Encodes a `RecordRlpEncoded` to its base64 from.
    pub(crate) fn to_base64(&self) -> Vec<u8> {
        let mut output = vec![0; MAX_BASE64_ENCODED_BYTE_LENGTH];
        let size = base64_engine()
            .encode_slice(self.rlp_encoded(), &mut output)
            .unwrap();
        output.truncate(size);

        output
    }

    /// Creates a `RecordRlpEncoded` from its base64 form `s`.
    pub(crate) fn from_base64(s: &str) -> Result<Self, Error> {
        if s.len() > MAX_BASE64_ENCODED_BYTE_LENGTH {
            return Err(Error::MaximumRecordRlpEncodedByteLengthExceeded);
        }

        let mut output = vec![0; MAX_RLP_ENCODED_BYTE_LENGTH];
        let size = base64_engine()
            .decode_slice(s, &mut output)
            .map_err(|_| Error::DecodingFailedForInvalidInput)?;
        if size > MAX_RLP_ENCODED_BYTE_LENGTH {
            return Err(Error::MaximumRecordRlpEncodedByteLengthExceeded);
        }
        output.truncate(size);

        Ok(Self::from_vec(output))
    }
}

/// Returns the base64 engine with options as per spec.
pub const fn base64_engine() -> &'static base64::engine::GeneralPurpose {
    &base64::engine::general_purpose::URL_SAFE_NO_PAD
}
