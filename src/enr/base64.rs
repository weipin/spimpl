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
use base64::engine::DecodePaddingMode;
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
    pub(crate) fn from_base64(s: &str) -> Result<Self, RlpDecodingError> {
        if s.len() > MAXIMUM_BASE64_ENCODED_BYTE_LENGTH {
            return Err(RlpDecodingError::MaximumEncodedByteLengthExceeded);
        }

        let mut output = vec![0; MAXIMUM_BASE64_ENCODED_BYTE_LENGTH];
        let size = decode_engine_slice(s, &mut output, &URL_SAFE_CONFIG)
            .map_err(|_| RlpDecodingError::InvalidFormat)?;
        output.resize(size, 0);

        Ok(StorageWithSignatureRlp(output))
    }
}

pub(crate) static URL_SAFE_CONFIG: FastPortable = FastPortable::from(
    &URL_SAFE,
    FastPortableConfig::new()
        .with_encode_padding(false)
        .with_decode_allow_trailing_bits(false)
        .with_decode_padding_mode(DecodePaddingMode::RequireNone),
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enr::storage_content_with_signature_rlp::MAXIMUM_RLP_ENCODED_BYTE_LENGTH;

    #[test]
    fn test_check_maximum_encoded_byte_length() {
        let data1 = concat!(
            "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gSW50ZWdlciBuZWMgb2Rpby4gUHJhZXNlbnQgbGliZXJvLiBTZWQgY3Vyc3VzIGFudGUgZGFwaWJ1cyBkaWFtLiBTZWQgbmlzaS4gTnVsbGEgcXVpcyBzZW0gYXQgbmliaCBlbGVtZW50dW0gaW1wZXJkaWV0LiBEdWlzIHNhZ2l0dGlzIGlwc3VtLiBQcmFlc2VudCBtYXVyaXMuIEZ1c2NlIG5lYyB0ZWxsdXMgc2VkIGF1Z3VlIHNlbXBlciBwb3J0YS4gTWF1cmlzIG1hc3NhLiBWZXN0aWJ1bHVtIGxhY2luaWEgYXJjdSBlZ2V0dS4",
        ); // 299
        let data2 = concat!(
            "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gSW50ZWdlciBuZWMgb2Rpby4gUHJhZXNlbnQgbGliZXJvLiBTZWQgY3Vyc3VzIGFudGUgZGFwaWJ1cyBkaWFtLiBTZWQgbmlzaS4gTnVsbGEgcXVpcyBzZW0gYXQgbmliaCBlbGVtZW50dW0gaW1wZXJkaWV0LiBEdWlzIHNhZ2l0dGlzIGlwc3VtLiBQcmFlc2VudCBtYXVyaXMuIEZ1c2NlIG5lYyB0ZWxsdXMgc2VkIGF1Z3VlIHNlbXBlciBwb3J0YS4gTWF1cmlzIG1hc3NhLiBWZXN0aWJ1bHVtIGxhY2luaWEgYXJjdSBlZ2V0dWEu",
        ); // 300
        let data3 = concat!(
            "TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4gSW50ZWdlciBuZWMgb2Rpby4gUHJhZXNlbnQgbGliZXJvLiBTZWQgY3Vyc3VzIGFudGUgZGFwaWJ1cyBkaWFtLiBTZWQgbmlzaS4gTnVsbGEgcXVpcyBzZW0gYXQgbmliaCBlbGVtZW50dW0gaW1wZXJkaWV0LiBEdWlzIHNhZ2l0dGlzIGlwc3VtLiBQcmFlc2VudCBtYXVyaXMuIEZ1c2NlIG5lYyB0ZWxsdXMgc2VkIGF1Z3VlIHNlbXBlciBwb3J0YS4gTWF1cmlzIG1hc3NhLiBWZXN0aWJ1bHVtIGxhY2luaWEgYXJjdSBlZ2V0dWFuLg",
        ); // 301

        assert!(StorageWithSignatureRlp::from_base64(data1).is_ok());
        assert!(StorageWithSignatureRlp::from_base64(data2).is_ok());
        assert_eq!(
            StorageWithSignatureRlp::from_base64(data3).unwrap_err(),
            RlpDecodingError::MaximumEncodedByteLengthExceeded
        );
    }
}
