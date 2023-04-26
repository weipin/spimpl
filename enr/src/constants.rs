// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::SequenceNumber;

/// The default value of sequence number.
pub(crate) const SEQUENCE_NUMBER_INITIAL: SequenceNumber = 1;

/// The maximum encoded size in bytes of a node record.
pub const MAX_RLP_ENCODED_BYTE_LENGTH: usize = 300;

// The maximum base64 size in bytes of a node record.
pub(crate) const MAX_BASE64_ENCODED_BYTE_LENGTH: usize = 400;

// The prefix of record textual form.
pub(crate) const TEXTUAL_FORM_PREFIX: &str = "enr:";

#[cfg(test)]
mod tests {
    use base64::engine::DecodeEstimate;
    use base64::Engine;

    use crate::base64::base64_engine;

    use super::*;

    #[test]
    fn max_base64_encoded_byte_length() {
        let decoded_estimate = base64_engine()
            .internal_decoded_len_estimate(MAX_BASE64_ENCODED_BYTE_LENGTH)
            .decoded_len_estimate();
        assert_eq!(decoded_estimate, MAX_RLP_ENCODED_BYTE_LENGTH);
    }
}
