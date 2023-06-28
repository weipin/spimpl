// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::CString;
use std::mem;

use dispatch_sys::{
    dispatch_async_f, dispatch_queue_attr_make_with_qos_class, dispatch_queue_create,
    dispatch_queue_create_with_target, dispatch_queue_t, dispatch_release, dispatch_sync_f,
};

use crate::closure_func::{invoke_boxed_closure, invoke_closure};
use crate::{Qos, QueueConfig};

pub struct Queue(pub(crate) dispatch_queue_t);

impl Queue {
    pub fn new(label: &str, config: &QueueConfig) -> Queue {
        let label = CString::new(label).expect("CString::new failed");
        let attr = {
            let attr = config.attributes.to_sys();
            if config.qos == Qos::Unspecified {
                attr
            } else {
                unsafe { dispatch_queue_attr_make_with_qos_class(attr, config.qos.to_sys(), 0) }
            }
        };

        let queue = match &config.target {
            Some(target) => unsafe {
                dispatch_queue_create_with_target(label.as_ptr(), attr, target.0)
            },
            None => unsafe { dispatch_queue_create(label.as_ptr(), attr) },
        };

        Queue(queue)
    }

    pub fn dispatch_async<F>(&self, work: F)
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
            dispatch_async_f(self.0, context, func);
        }
    }

    pub fn dispatch_sync<F>(&self, work: F)
    where
        F: FnMut(),
    {
        let (context, func) = unsafe {
            (
                // `&&`: reference to fat pointer
                mem::transmute(&&work),
                mem::transmute(invoke_closure::<F> as extern "C" fn(_)),
            )
        };

        unsafe {
            dispatch_sync_f(self.0, context, func);
        }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_sys::dispatch_object_t { _dq: self.0 });
        }
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::thread;

    use super::*;

    #[test]
    fn test_dispatch_async() {
        let x = Rc::new(RefCell::new(0));
        let y = x.clone();
        let queue = Queue::new("", &QueueConfig::default());
        queue.dispatch_async(move || {
            y.replace(66);
        });
        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(*x.borrow(), 66);
    }

    #[test]
    fn test_dispatch_async_with_target() {
        let x = Rc::new(RefCell::new(0));
        let y = x.clone();
        let target = Queue::new("my target", &QueueConfig::default());
        let queue = Queue::new(
            "",
            &QueueConfig {
                target: Some(&target),
                ..Default::default()
            },
        );
        queue.dispatch_async(move || {
            y.replace(66);
        });
        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(*x.borrow(), 66);
    }

    #[test]
    fn test_dispatch_sync() {
        let mut x = 0;
        let queue = Queue::new("", &QueueConfig::default());
        queue.dispatch_sync(|| {
            x = 66;
        });
        assert_eq!(x, 66);
    }
}
