// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::dispatch_queue_t;

extern "C" {
    pub fn dispatch_queue_get_qos_class(
        queue: dispatch_queue_t,
        relative_priority_ptr: *mut ::std::os::raw::c_int,
    ) -> dispatch_qos_class_t;
}

pub const QOS_CLASS_USER_INTERACTIVE: _bindgen_ty_2 = 33;
pub const QOS_CLASS_USER_INITIATED: _bindgen_ty_2 = 25;
pub const QOS_CLASS_DEFAULT: _bindgen_ty_2 = 21;
pub const QOS_CLASS_UTILITY: _bindgen_ty_2 = 17;
pub const QOS_CLASS_BACKGROUND: _bindgen_ty_2 = 9;
pub const QOS_CLASS_UNSPECIFIED: _bindgen_ty_2 = 0;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub type qos_class_t = ::std::os::raw::c_uint;

pub type dispatch_qos_class_t = qos_class_t;
