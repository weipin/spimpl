use quickcheck::{Arbitrary, Gen};

#[derive(Debug)]
pub(crate) struct Ipv4AddrOctets(pub(crate) [u8; 4]);

impl Clone for Ipv4AddrOctets {
    fn clone(&self) -> Self {
        Ipv4AddrOctets(self.0.clone())
    }
}

impl Arbitrary for Ipv4AddrOctets {
    fn arbitrary(g: &mut Gen) -> Self {
        let octets = [
            u8::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
            u8::arbitrary(g),
        ];
        Ipv4AddrOctets(octets)
    }
}

#[derive(Debug)]
pub(crate) struct Ipv6AddrOctets(pub(crate) [u16; 8]);

impl Clone for Ipv6AddrOctets {
    fn clone(&self) -> Self {
        Ipv6AddrOctets(self.0.clone())
    }
}

impl Arbitrary for Ipv6AddrOctets {
    fn arbitrary(g: &mut Gen) -> Self {
        let octets = [
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
            u16::arbitrary(g),
        ];
        Ipv6AddrOctets(octets)
    }
}
