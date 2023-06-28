// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dispatch_sys::{
    _dispatch_queue_attr_concurrent, dispatch_queue_attr_make_initially_inactive,
    dispatch_queue_attr_t,
};

pub enum QueueKind {
    Serial,
    Concurrent,
}

pub struct QueueAttributes {
    pub kind: QueueKind,
    pub is_initially_inactive: bool,
}

impl QueueAttributes {
    pub(crate) fn to_sys(&self) -> dispatch_queue_attr_t {
        let attr = match self.kind {
            QueueKind::Serial => 0 as dispatch_queue_attr_t,
            QueueKind::Concurrent => unsafe {
                &mut _dispatch_queue_attr_concurrent as dispatch_queue_attr_t
            },
        };

        if self.is_initially_inactive {
            return unsafe { dispatch_queue_attr_make_initially_inactive(attr) };
        }

        attr
    }
}

impl Default for QueueAttributes {
    fn default() -> Self {
        Self {
            kind: QueueKind::Serial,
            is_initially_inactive: false,
        }
    }
}
