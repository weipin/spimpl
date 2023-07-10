// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{dispatch_function_t, dispatch_queue_t, dispatch_time_t};

extern "C" {
    pub fn dispatch_source_create(
        type_: dispatch_source_type_t,
        handle: usize,
        mask: usize,
        queue: dispatch_queue_t,
    ) -> dispatch_source_t;

    pub fn dispatch_source_set_event_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    pub fn dispatch_source_set_cancel_handler_f(
        source: dispatch_source_t,
        handler: dispatch_function_t,
    );
    pub fn dispatch_source_set_timer(
        source: dispatch_source_t,
        start: dispatch_time_t,
        interval: u64,
        leeway: u64,
    );
    pub fn dispatch_source_get_handle(source: dispatch_source_t) -> usize;
    pub fn dispatch_source_cancel(source: dispatch_source_t);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_source_s {
    pub _address: u8,
}
pub type dispatch_source_t = *mut dispatch_source_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_source_type_s {
    _unused: [u8; 0],
}
pub type dispatch_source_type_t = *const dispatch_source_type_s;

extern "C" {
    pub static _dispatch_source_type_timer: dispatch_source_type_s;
    pub static _dispatch_source_type_read: dispatch_source_type_s;
    pub static _dispatch_source_type_write: dispatch_source_type_s;
}

pub const DISPATCH_TIMER_STRICT: u32 = 1;
