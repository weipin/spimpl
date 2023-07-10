// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{dispatch_group_s, dispatch_queue_s, dispatch_source_s};

extern "C" {
    pub fn dispatch_release(object: dispatch_object_t);
    pub fn dispatch_activate(object: dispatch_object_t);
    pub fn dispatch_suspend(object: dispatch_object_t);
    pub fn dispatch_resume(object: dispatch_object_t);
    pub fn dispatch_set_context(object: dispatch_object_t, context: *mut ::std::os::raw::c_void);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_object_s {
    pub _address: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dispatch_object_t {
    pub _do: *mut dispatch_object_s,
    pub _dq: *mut dispatch_queue_s,
    pub _dg: *mut dispatch_group_s,
    pub _ds: *mut dispatch_source_s,
}
