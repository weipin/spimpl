// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{dispatch_function_t, dispatch_queue_t};

extern "C" {
    pub fn dispatch_group_create() -> dispatch_group_t;

    pub fn dispatch_group_async_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *mut ::std::os::raw::c_void,
        work: dispatch_function_t,
    );

    pub fn dispatch_group_notify_f(
        group: dispatch_group_t,
        queue: dispatch_queue_t,
        context: *mut ::std::os::raw::c_void,
        work: dispatch_function_t,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_group_s {
    pub _address: u8,
}

pub type dispatch_group_t = *mut dispatch_group_s;
