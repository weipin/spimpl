// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern "C" {
    pub fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
    pub fn dispatch_walltime(when: *const timespec, delta: i64) -> dispatch_time_t;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: ::std::os::raw::c_long,
}

pub type __darwin_time_t = ::std::os::raw::c_long;
pub type dispatch_time_t = u64;

pub const DISPATCH_TIME_NOW: u32 = 0;
