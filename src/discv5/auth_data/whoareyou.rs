// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::{AuthDataSize, AuthDataSource};
use crate::enr;

// authdata      = id-nonce || enr-seq
// authdata-size = 24
// id-nonce      = uint128   -- random bytes
// enr-seq       = uint64    -- ENR sequence number of the requesting node
pub(crate) struct IdNonce(pub(crate) [u8; 16]);

pub(crate) struct WhoareyouAuthDataSource {
    id_nonce: IdNonce,
    enr_seq: enr::SequenceNumber,
}

impl AuthDataSource for WhoareyouAuthDataSource {
    const SIZE: AuthDataSize = 24;

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.id_nonce.0);
        buffer.extend(self.enr_seq.to_be_bytes());
    }
}
