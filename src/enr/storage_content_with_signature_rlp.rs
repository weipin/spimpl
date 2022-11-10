// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::scheme::Scheme;
use super::storage::Storage;
use super::storage_rlp_encoding::RlpEncodingError;

pub(crate) const MAXIMUM_ENCODED_BYTE_LENGTH: usize = 300;

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
        if rlp.len() > MAXIMUM_ENCODED_BYTE_LENGTH {
            return Err(RlpEncodingError::MaximumEncodedByteLengthExceeded);
        }

        Ok(StorageWithSignatureRlp(self.to_rlp::<S>(true)))
    }
}
