// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::storage_content_with_signature_rlp::{
    StorageWithSignatureRlp, MAXIMUM_ENCODED_BYTE_LENGTH,
};
use super::storage_rlp_decoding::RlpDecodingError;
use base64::alphabet::URL_SAFE;
use base64::engine::fast_portable::{FastPortable, FastPortableConfig};
use base64::engine::{DecodeEstimate, Engine};
use base64::{decode_engine_slice, encode_engine_slice};

impl StorageWithSignatureRlp {
    pub(crate) fn to_base64(&self) -> Vec<u8> {
        let mut output = vec![0; 1024];
        let size = encode_engine_slice(&self.0, &mut output, &URL_SAFE_CONFIG);
        output[0..size].to_vec()
    }

    pub(crate) fn from_base64(s: &str) -> Result<Self, RlpDecodingError> {
        let estimate = URL_SAFE_CONFIG.decoded_length_estimate(s.len());
        if estimate.decoded_length_estimate() > MAXIMUM_ENCODED_BYTE_LENGTH {
            return Err(RlpDecodingError::MaximumEncodedByteLengthExceeded);
        }

        let mut output = vec![0; 1024];
        let size = decode_engine_slice(s, &mut output, &URL_SAFE_CONFIG)
            .map_err(|_| RlpDecodingError::InvalidFormat)?;
        Ok(StorageWithSignatureRlp(output[0..size].to_vec()))
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

    #[test]
    fn test_check_maximum_encoded_byte_length() {
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

        assert!(StorageWithSignatureRlp::from_base64(data1).is_ok());
        assert!(StorageWithSignatureRlp::from_base64(data2).is_ok());
        assert_eq!(
            StorageWithSignatureRlp::from_base64(data3).unwrap_err(),
            RlpDecodingError::MaximumEncodedByteLengthExceeded
        );
    }
}
