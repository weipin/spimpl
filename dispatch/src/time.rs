// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::null;

use dispatch_sys::{dispatch_time, dispatch_time_t, dispatch_walltime, DISPATCH_TIME_NOW};

pub struct DispatchTime(pub(crate) dispatch_time_t);

impl DispatchTime {
    pub fn now() -> Self {
        DispatchTime(unsafe { dispatch_time(DISPATCH_TIME_NOW as u64, 0) })
    }
}

impl Default for DispatchTime {
    fn default() -> Self {
        Self::now()
    }
}

pub struct DispatchWallTime(pub(crate) dispatch_time_t);

impl DispatchWallTime {
    pub fn now() -> Self {
        DispatchWallTime(unsafe { dispatch_walltime(null(), 0) })
    }
}

impl Default for DispatchWallTime {
    fn default() -> Self {
        Self::now()
    }
}
