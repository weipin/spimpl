// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;
use std::mem;

use dispatch_sys::{
    dispatch_group_async_f, dispatch_group_create, dispatch_group_notify_f, dispatch_group_t,
    dispatch_object_t, dispatch_release,
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
        let context = Box::into_raw(Box::new(work));
        let func = unsafe { mem::transmute(invoke_boxed_closure::<F> as extern "C" fn(_)) };

        unsafe {
            dispatch_group_notify_f(self.0, queue.0, context as *mut c_void, func);
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_object_t { _dg: self.0 });
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
        let context = Box::into_raw(Box::new(work));
        let func = unsafe { mem::transmute(invoke_boxed_closure::<F> as extern "C" fn(_)) };

        unsafe {
            dispatch_group_async_f(group.0, self.0, context as *mut c_void, func);
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::cell::UnsafeCell;
    use std::rc::Rc;
    use std::thread;

    use crate::{Queue, QueueConfig};

    use super::*;

    #[test]
    fn test_group_async() {
        let x = Rc::new(UnsafeCell::new(0));
        let y = x.clone();
        let z = x.clone();
        let g = x.clone();
        let group = Group::new();
        let queue = Queue::new("", &QueueConfig::default());
        queue.dispatch_group_async(&group, move || {
            unsafe {
                let v: &mut i32 = &mut *y.get(); // -- borrow --+
                *v += 1;
            }
        });
        queue.dispatch_group_async(&group, move || {
            unsafe {
                let v: &mut i32 = &mut *z.get(); // -- borrow --+
                *v += 1;
            }
        });
        group.notify(&queue, move || {
            unsafe {
                let v: &mut i32 = &mut *g.get(); // -- borrow --+
                *v += 1;
            }
        });

        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(unsafe { *x.get() }, 3);
    }

    #[test]
    fn test_group_async_with_different_queues() {
        let x = Rc::new(UnsafeCell::new(0));
        let y = Rc::new(UnsafeCell::new(0));
        let z = Rc::new(UnsafeCell::new(0));
        let group = Group::new();
        let queue_a = Queue::new("a", &QueueConfig::default());
        let queue_b = Queue::new("b", &QueueConfig::default());
        let queue_c = Queue::new("c", &QueueConfig::default());
        queue_a.dispatch_group_async(&group, {
            let x = x.clone();
            move || {
                unsafe {
                    let v: &mut i32 = &mut *x.clone().get(); // -- borrow --+
                    *v += 1;
                }
            }
        });
        queue_b.dispatch_group_async(&group, {
            let y = y.clone();
            move || {
                unsafe {
                    let v: &mut i32 = &mut *y.clone().get(); // -- borrow --+
                    *v += 1;
                }
            }
        });
        group.notify(&queue_c, {
            let z = z.clone();
            move || {
                unsafe {
                    let v: &mut i32 = &mut *z.clone().get(); // -- borrow --+
                    *v += 1;
                }
            }
        });

        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(unsafe { *x.get() }, 1);
        assert_eq!(unsafe { *y.get() }, 1);
        assert_eq!(unsafe { *z.get() }, 1);
    }
}
