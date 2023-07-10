// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::os::fd::RawFd;

use dispatch_sys::{
    dispatch_object_t, dispatch_release, dispatch_source_create, dispatch_source_t,
};

use crate::{Queue, Source, SourceType};

pub struct SourceRead(dispatch_source_t);

impl SourceRead {
    pub fn new(file_descriptor: RawFd, queue: &Queue) -> Self {
        let s = unsafe {
            dispatch_source_create(
                SourceType::Read.to_sys(),
                file_descriptor as usize,
                0,
                queue.0,
            )
        };
        SourceRead(s)
    }
}

impl Source for SourceRead {
    fn get_sys(&self) -> dispatch_source_t {
        self.0
    }
}

impl Drop for SourceRead {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_object_t { _ds: self.0 });
        }
    }
}

pub struct SourceWrite(dispatch_source_t);

impl SourceWrite {
    pub fn new(file_descriptor: RawFd, queue: &Queue) -> Self {
        let s = unsafe {
            dispatch_source_create(
                SourceType::Write.to_sys(),
                file_descriptor as usize,
                0,
                queue.0,
            )
        };
        SourceWrite(s)
    }
}

impl Source for SourceWrite {
    fn get_sys(&self) -> dispatch_source_t {
        self.0
    }
}

impl Drop for SourceWrite {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(dispatch_object_t { _ds: self.0 });
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;
    use std::os::fd::AsRawFd;
    use std::thread;
    use std::time::Duration;

    use crate::QueueConfig;

    use super::*;

    #[test]
    fn test_read_write() {
        const HELLO: &[u8] = b"hello world";
        const SERVER_ADDRESS: &str = "127.0.0.1:7878";
        const CLIENT_ADDRESS: &str = "127.0.0.1:7879";

        // server (read)
        static mut BUF: [u8; 64] = [0; 64];
        extern "C" fn read_event_handler(context: usize) {
            let socket = unsafe { &*(context as *mut UdpSocket) };
            match socket.recv_from(unsafe { &mut BUF }) {
                Ok(_) => {}
                Err(e) => panic!("encountered IO read error: {e}"),
            }
        }

        let socket_server = UdpSocket::bind(SERVER_ADDRESS).unwrap();
        socket_server.set_nonblocking(true).unwrap();
        let queue_server = Queue::new("server", &QueueConfig::default());
        let source_read = SourceRead::new(socket_server.as_raw_fd(), &queue_server);
        source_read.set_context(&socket_server as *const _ as usize);
        source_read.set_event_handler(read_event_handler);

        // client (write)
        extern "C" fn write_event_handler(context: usize) {
            let socket = unsafe { &*(context as *mut UdpSocket) };
            match socket.send(HELLO) {
                Ok(_) => {}
                Err(e) => panic!("encountered IO write error: {e}"),
            }
        }

        let socket_client = UdpSocket::bind(CLIENT_ADDRESS).unwrap();
        socket_client.set_nonblocking(true).unwrap();
        socket_client
            .connect(SERVER_ADDRESS)
            .expect("connect server failed");
        let queue_client = Queue::new("client", &QueueConfig::default());
        let source_write = SourceWrite::new(socket_client.as_raw_fd(), &queue_client);
        source_write.set_context(&socket_client as *const _ as usize);
        source_write.set_event_handler(write_event_handler);

        assert!(!unsafe { BUF.starts_with(HELLO) });
        source_read.activate();
        source_write.activate();
        thread::sleep(Duration::from_millis(500));
        assert!(unsafe { BUF.starts_with(HELLO) });
    }
}
