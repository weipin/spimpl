// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements `Record`.

use super::storage::Storage;
use crate::enr::storage_rlp_encoding::RlpEncodingError;
use crate::enr::Scheme;
use std::net::Ipv4Addr;

/// To create a `Record`, use `Builder`.
#[derive(Debug)]
pub struct Record(pub(crate) Storage);

impl Record {
    pub fn rlp_data<S: Scheme>(&self) -> Result<Vec<u8>, RlpEncodingError> {
        let rlp = self.0.encode_content_with_signature_to_rlp::<S>()?;
        Ok(rlp.0)
    }

    pub fn ip4(&self) -> Option<Ipv4Addr> {
        self.0.ip4
    }

    pub fn tcp4(&self) -> Option<u16> {
        self.0.tcp4
    }

    pub fn udp4(&self) -> Option<u16> {
        self.0.udp4
    }
}
