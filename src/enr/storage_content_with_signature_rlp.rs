// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements the struct representing "signature + content" RLP.

use super::scheme::Scheme;
use super::storage::Storage;
use super::storage_rlp_encoding::RlpEncodingError;

// The maximum encoded size of a node record is 300 bytes.
pub(crate) const MAXIMUM_RLP_ENCODED_BYTE_LENGTH: usize = 300;

// The largest value of `a` that
// `URL_SAFE_CONFIG.decoded_length_estimate(a).decoded_length_estimate() <= MAXIMUM_RLP_ENCODED_BYTE_LENGTH`
pub(crate) const MAXIMUM_BASE64_ENCODED_BYTE_LENGTH: usize = 400;

#[derive(Debug)]
pub(crate) struct StorageWithSignatureRlp(pub(crate) Vec<u8>);

impl Storage {
    pub(crate) fn encode_content_with_signature_to_rlp<S: Scheme>(
        &self,
    ) -> Result<StorageWithSignatureRlp, RlpEncodingError> {
        debug_assert!(self.id.is_some());
        debug_assert!(self.public_key_value.is_some());
        debug_assert!(self.signature_value.is_some());

        let rlp = self.to_rlp::<S>(true);
        if rlp.len() > MAXIMUM_RLP_ENCODED_BYTE_LENGTH {
            return Err(RlpEncodingError::MaximumEncodedByteLengthExceeded);
        }

        Ok(StorageWithSignatureRlp(self.to_rlp::<S>(true)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::base64::URL_SAFE_CONFIG;
    use base64::engine::{DecodeEstimate, Engine};

    #[test]
    fn test_maximum_base64_encoded_byte_length() {
        assert!(
            URL_SAFE_CONFIG
                .decoded_length_estimate(MAXIMUM_BASE64_ENCODED_BYTE_LENGTH)
                .decoded_length_estimate()
                <= MAXIMUM_RLP_ENCODED_BYTE_LENGTH
        );
        assert!(
            URL_SAFE_CONFIG
                .decoded_length_estimate(MAXIMUM_BASE64_ENCODED_BYTE_LENGTH + 1)
                .decoded_length_estimate()
                > MAXIMUM_RLP_ENCODED_BYTE_LENGTH
        );
    }
}
