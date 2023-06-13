// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern "C" {
    pub fn dispatch_queue_create(
        label: *const ::std::os::raw::c_char,
        attr: dispatch_queue_attr_t,
    ) -> dispatch_queue_t;

    pub fn dispatch_sync_f(
        queue: dispatch_queue_t,
        context: *mut ::std::os::raw::c_void,
        work: dispatch_function_t,
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_queue_s {
    pub _address: u8,
}

pub type dispatch_queue_t = *mut dispatch_queue_s;

pub type dispatch_queue_attr_t = *mut dispatch_queue_attr_s;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dispatch_queue_attr_s {
    pub _address: u8,
}

pub type dispatch_function_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    static mut FOO: u8 = 0;

    #[no_mangle]
    pub unsafe extern "C" fn foo_func(_arg1: *mut ::std::os::raw::c_void) {
        FOO = 66;
    }

    #[test]
    fn test_dispatch_sync_f() {
        assert_eq!(unsafe { FOO }, 0);

        let label = CStr::from_bytes_with_nul(b"bar\0").unwrap();
        let queue = unsafe { dispatch_queue_create(label.as_ptr(), std::ptr::null_mut()) };
        unsafe { dispatch_sync_f(queue, std::ptr::null_mut(), Some(foo_func)) };
        assert_eq!(unsafe { FOO }, 66);
    }
}
