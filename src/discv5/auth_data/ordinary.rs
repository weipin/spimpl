// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::discv5::auth_data::core::{AuthDataSize, AuthDataSource};
use crate::discv5::device;

// authdata      = src-id
// authdata-size = 32
impl AuthDataSource for device::Context {
    const SIZE: AuthDataSize = 32;

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend(self.node_id.0)
    }
}
