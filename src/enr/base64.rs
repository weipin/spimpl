// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::storage_content_with_signature_rlp::StorageWithSignatureRlp;
use super::storage_content_with_signature_rlp::MAXIMUM_BASE64_ENCODED_BYTE_LENGTH;
use super::storage_rlp_decoding::RlpDecodingError;
use base64::alphabet::URL_SAFE;
use base64::engine::fast_portable::{FastPortable, FastPortableConfig};
use base64::{decode_engine_slice, encode_engine_slice};

impl StorageWithSignatureRlp {
    /// Encodes the `StorageWithSignatureRlp` to its base64 form
    /// and returns the slice of the `output` that contains the base64 characters.
    /// Will panic if `output` isn't large enough.
    pub(crate) fn to_base64<'a>(&self, output: &'a mut [u8]) -> &'a [u8] {
        let size = encode_engine_slice(&self.0, output, &URL_SAFE_CONFIG);
        &output[0..size]
    }

    /// Creates a `StorageWithSignatureRlp` from its base64 form.
    /// Will panics if `intermediate_decoding_output` isn't large enough.
    pub(crate) fn from_base64(
        s: &str,
        intermediate_decoding_output: &mut [u8],
    ) -> Result<Self, RlpDecodingError> {
        if s.len() > MAXIMUM_BASE64_ENCODED_BYTE_LENGTH {
            return Err(RlpDecodingError::MaximumEncodedByteLengthExceeded);
        }

        let size = decode_engine_slice(s, intermediate_decoding_output, &URL_SAFE_CONFIG)
            .map_err(|_| RlpDecodingError::InvalidFormat)?;
        Ok(StorageWithSignatureRlp(
            intermediate_decoding_output[0..size].to_vec(),
        ))
    }
}

pub(crate) static URL_SAFE_CONFIG: FastPortable = FastPortable::from(
    &URL_SAFE,
    FastPortableConfig::new()
        .with_encode_padding(false)
        .with_decode_allow_trailing_bits(false),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::output::maximum_rlp_encoded_output;

    #[test]
    fn test_check_maximum_encoded_byte_length() {
        let mut intermediate_coding_output = maximum_rlp_encoded_output();
        let data1 = concat!(
            "-QEouQElYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWE"
        ); // 299
        let data2 = concat!(
            "-QEpuQEmYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh"
        ); // 300
        let data3 = concat!(
            "-QEquQEnYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFh",
            "YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYQ"
        ); // 301

        assert!(
            StorageWithSignatureRlp::from_base64(data1, &mut intermediate_coding_output).is_ok()
        );
        assert!(
            StorageWithSignatureRlp::from_base64(data2, &mut intermediate_coding_output).is_ok()
        );
        assert_eq!(
            StorageWithSignatureRlp::from_base64(data3, &mut intermediate_coding_output)
                .unwrap_err(),
            RlpDecodingError::MaximumEncodedByteLengthExceeded
        );
    }
}
