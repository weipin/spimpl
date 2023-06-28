// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::mem;

use dispatch_sys::{
    dispatch_group_async_f, dispatch_group_create, dispatch_group_notify_f, dispatch_group_t,
    dispatch_release,
};

use crate::closure_func::invoke_boxed_closure;
use crate::Queue;

pub struct Group(dispatch_group_t);

impl Group {
    pub fn new() -> Group {
        Group(unsafe { dispatch_group_create() })
    }

    pub fn notify<F>(&self, queue: &Queue, work: F)
    where
        F: 'static + FnOnce(),
    {
        let (context, func) = unsafe {
            (
                mem::transmute(Box::new(work)),
                mem::transmute(invoke_boxed_closure::<F> as extern "C" fn(_)),
            )
        };

        unsafe {
            dispatch_group_notify_f(self.0, queue.0, context, func);
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_sys::dispatch_object_t { _dg: self.0 });
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue {
    pub fn dispatch_group_async<F>(&self, group: &Group, work: F)
    where
        F: 'static + FnOnce(),
    {
        let (context, func) = unsafe {
            (
                mem::transmute(Box::new(work)),
                mem::transmute(invoke_boxed_closure::<F> as extern "C" fn(_)),
            )
        };

        unsafe {
            dispatch_group_async_f(group.0, self.0, context, func);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::thread;

    use crate::{Queue, QueueConfig};

    use super::*;

    #[test]
    fn test_group_async() {
        let x = Rc::new(RefCell::new(0));
        let y = x.clone();
        let z = x.clone();
        let g = x.clone();
        let group = Group::new();
        let queue = Queue::new("", &QueueConfig::default());
        queue.dispatch_group_async(&group, move || {
            y.replace_with(|&mut old| old + 1);
        });
        queue.dispatch_group_async(&group, move || {
            z.replace_with(|&mut old| old + 1);
        });
        group.notify(&queue, move || {
            g.replace_with(|&mut old| old + 1);
        });

        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(*x.borrow(), 3);
    }
}
