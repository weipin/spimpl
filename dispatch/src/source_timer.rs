// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;

use dispatch_sys::{
    dispatch_release, dispatch_source_create, dispatch_source_set_timer, dispatch_source_t,
    DISPATCH_TIMER_STRICT,
};

use crate::{DispatchTime, Queue, Source, SourceType};

pub struct SourceTimer(dispatch_source_t);

impl SourceTimer {
    pub fn new(flag: TimerFlag, queue: &Queue) -> Self {
        let s = unsafe {
            dispatch_source_create(SourceType::Timer.to_sys(), 0, flag as usize, queue.0)
        };
        SourceTimer(s)
    }

    pub fn set_timer(&self, start: DispatchTime, interval: Duration, leeway: Duration) {
        unsafe {
            dispatch_source_set_timer(
                self.0,
                start.0,
                interval.as_nanos().try_into().unwrap(),
                leeway.as_nanos().try_into().unwrap(),
            );
        }
    }
}

impl Source for SourceTimer {
    fn get_sys(&self) -> dispatch_source_t {
        self.0
    }
}

impl Drop for SourceTimer {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_sys::dispatch_object_t { _ds: self.0 });
        }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum TimerFlag {
    None = 0,
    Strict = DISPATCH_TIMER_STRICT,
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::QueueConfig;

    use super::*;

    #[test]
    fn test_source_timer() {
        let counter = 0_usize;
        let not_cancelled = true;

        extern "C" fn event_handler(context: usize) {
            unsafe {
                *(context as *mut usize) += 1;
            }
        }

        extern "C" fn cancel_handler(context: usize) {
            unsafe {
                *(context as *mut bool) = false;
            }
        }

        let queue = Queue::new("", &QueueConfig::default());
        let source = SourceTimer::new(TimerFlag::None, &queue);
        source.set_timer(
            DispatchTime::now(),
            Duration::from_millis(100),
            Duration::from_nanos(0),
        );
        source.set_context(&counter as *const _ as usize);
        source.set_event_handler(event_handler);
        source.set_cancel_handler(cancel_handler);
        source.activate();

        thread::sleep(Duration::from_millis(500));
        assert!(counter > 3);
        assert!(not_cancelled);

        source.set_context(&not_cancelled as *const _ as usize);
        source.cancel();
        thread::sleep(Duration::from_millis(100));
        assert_eq!(not_cancelled, false);
    }
}
