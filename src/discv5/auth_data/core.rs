// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub(crate) trait FixedSizeAuthDataSource {
    const SIZE: AuthDataSize;

    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>);
}

pub(crate) trait VariableSizeAuthDataSource {
    fn size(&self) -> AuthDataSize;
    fn append_data_to_buffer(&self, buffer: &mut Vec<u8>);
}

pub(crate) type AuthDataSize = u16;
