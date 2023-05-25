// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Textual form ("enr:xxx") related functions.

use crate::constants::TEXTUAL_FORM_PREFIX;
use crate::{Error, Record, RecordRlpEncoded, Scheme};

impl Record {
    /// Returns the textual form of the `Record`.
    pub fn to_textual_form<S: Scheme>(&self) -> Result<String, Error> {
        let encoded = self.to_rlp_encoded::<S>()?;
        let base64 = encoded.to_base64();

        Ok([TEXTUAL_FORM_PREFIX, &String::from_utf8(base64).unwrap()].concat())
    }

    /// Creates a `Record` from its textual form.
    pub fn from_textual_form<S: Scheme>(s: &str) -> Result<Self, Error> {
        let encoded = RecordRlpEncoded::from_textual_form(s)?;
        Record::from_rlp_encoded::<S>(&encoded)
    }
}

impl RecordRlpEncoded<'_> {
    /// Creates a `RecordRlpEncoded` from ENR textual form.
    pub fn from_textual_form(s: &str) -> Result<Self, Error> {
        let base64 = s
            .strip_prefix(TEXTUAL_FORM_PREFIX)
            .ok_or(Error::DecodingFailedForInvalidInput)?;
        RecordRlpEncoded::from_base64(base64)
    }
}
