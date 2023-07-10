// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;
use std::mem;

use dispatch_sys::{
    _dispatch_source_type_read, _dispatch_source_type_timer, _dispatch_source_type_write,
    dispatch_activate, dispatch_object_t, dispatch_resume, dispatch_set_context,
    dispatch_source_cancel, dispatch_source_get_handle, dispatch_source_set_cancel_handler_f,
    dispatch_source_set_event_handler_f, dispatch_source_t, dispatch_source_type_t,
    dispatch_suspend,
};

pub type Handler = extern "C" fn(context: usize);

#[allow(drop_bounds)]
pub trait Source: Drop {
    fn get_sys(&self) -> dispatch_source_t;

    fn set_context(&self, context: usize) {
        unsafe {
            dispatch_set_context(
                dispatch_object_t {
                    _ds: self.get_sys(),
                },
                context as *mut c_void,
            );
        }
    }

    fn set_event_handler(&self, handler: Handler) {
        unsafe {
            let func = mem::transmute(handler as extern "C" fn(_));
            dispatch_source_set_event_handler_f(self.get_sys(), Some(func));
        }
    }

    fn set_cancel_handler(&self, handler: Handler) {
        unsafe {
            let func = mem::transmute(handler as extern "C" fn(_));
            dispatch_source_set_cancel_handler_f(self.get_sys(), Some(func));
        }
    }

    fn get_handle(&self) -> usize {
        unsafe { dispatch_source_get_handle(self.get_sys()) }
    }

    fn cancel(&self) {
        unsafe {
            dispatch_source_cancel(self.get_sys());
        }
    }

    fn activate(&self) {
        unsafe {
            dispatch_activate(dispatch_object_t {
                _ds: self.get_sys(),
            });
        }
    }

    fn resume(&self) {
        unsafe {
            dispatch_resume(dispatch_object_t {
                _ds: self.get_sys(),
            });
        }
    }

    fn suspend(&self) {
        unsafe {
            dispatch_suspend(dispatch_object_t {
                _ds: self.get_sys(),
            });
        }
    }
}

pub enum SourceType {
    Timer,
    Read,
    Write,
}

impl SourceType {
    pub fn to_sys(&self) -> dispatch_source_type_t {
        match self {
            SourceType::Timer => unsafe { &_dispatch_source_type_timer as dispatch_source_type_t },
            SourceType::Read => unsafe { &_dispatch_source_type_read as dispatch_source_type_t },
            SourceType::Write => unsafe { &_dispatch_source_type_write as dispatch_source_type_t },
        }
    }
}
