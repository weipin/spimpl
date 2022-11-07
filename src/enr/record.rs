use super::storage::Storage;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Record(pub(crate) Storage);

impl Record {
    pub fn ip4(&self) -> Option<Ipv4Addr> {
        self.0.ip4
    }

    pub fn tcp4(&self) -> Option<u16> {
        self.0.tcp4
    }

    pub fn udp4(&self) -> Option<u16> {
        self.0.udp4
    }
}
