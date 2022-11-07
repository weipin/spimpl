use super::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use super::scheme::Scheme;
use super::storage::Storage;
use bytes::BytesMut;
use fastrlp::{Encodable, Header};

impl Storage {
    pub(crate) fn to_rlp<S: Scheme>(&self, with_signature: bool) -> Vec<u8> {
        debug_assert!(self.id.is_some());
        debug_assert!(self.public_key_value.is_some());
        // content = [signature, seq, k, v, ...]

        // TODO: with_capacity
        let mut rlp_data = BytesMut::new();

        // The key/value pairs must be sorted by key and must be unique
        // order:
        // id, ip, ip6, secp256k1, tcp, tcp6, udp, udp6

        // Header
        let mut h = Header {
            list: true,
            payload_length: 0,
        };
        // signature
        if with_signature {
            debug_assert!(self.signature_value.is_some());
            h.payload_length += self.signature_value.as_ref().unwrap().as_slice().length();
        }

        // seq
        h.payload_length += self.seq.length();

        // id
        h.payload_length += ID_KEY.length();
        h.payload_length += S::id().as_bytes().length();
        // ip
        if let Some(ip4) = self.ip4 {
            h.payload_length += IP4_KEY.length();
            h.payload_length += ip4.octets().length();
        }
        // ip6
        if let Some(ip6) = self.ip6 {
            h.payload_length += IP6_KEY.length();
            h.payload_length += ip6.octets().length();
        }
        // public key value (secp256k1)
        h.payload_length += S::public_key_key().as_bytes().length();
        h.payload_length += (self.public_key_value.as_ref().unwrap() as &[u8]).length();
        // tcp
        if let Some(tcp4) = self.tcp4 {
            h.payload_length += TCP4_KEY.length();
            h.payload_length += tcp4.length();
        }
        // tcp6
        if let Some(tcp6) = self.tcp6 {
            h.payload_length += TCP6_KEY.length();
            h.payload_length += tcp6.length();
        }
        // udp
        if let Some(udp4) = self.udp4 {
            h.payload_length += UDP4_KEY.length();
            h.payload_length += udp4.length();
        }
        // udp6
        if let Some(udp6) = self.udp6 {
            h.payload_length += UDP6_KEY.length();
            h.payload_length += udp6.length();
        }

        // Encodes header
        h.encode(&mut rlp_data);

        // Encodes items
        // signature
        if with_signature {
            debug_assert!(self.signature_value.is_some());
            self.signature_value
                .as_ref()
                .unwrap()
                .as_slice()
                .encode(&mut rlp_data);
        }
        // seq
        self.seq.encode(&mut rlp_data);

        // id
        ID_KEY.encode(&mut rlp_data);
        S::id().as_bytes().encode(&mut rlp_data);

        // ip
        if let Some(ip4) = self.ip4 {
            IP4_KEY.encode(&mut rlp_data);
            ip4.octets().encode(&mut rlp_data);
        }

        // ip6
        if let Some(ip6) = self.ip6 {
            IP6_KEY.encode(&mut rlp_data);
            ip6.octets().encode(&mut rlp_data);
        }

        // public key value (secp256k1)
        S::public_key_key().as_bytes().encode(&mut rlp_data);
        self.public_key_value
            .as_ref()
            .unwrap()
            .as_slice()
            .encode(&mut rlp_data);

        // tcp
        if let Some(tcp4) = self.tcp4 {
            TCP4_KEY.encode(&mut rlp_data);
            tcp4.encode(&mut rlp_data);
        }

        // tcp6
        if let Some(tcp6) = self.tcp6 {
            TCP6_KEY.encode(&mut rlp_data);
            tcp6.encode(&mut rlp_data);
        }

        // udp
        if let Some(udp4) = self.udp4 {
            UDP4_KEY.encode(&mut rlp_data);
            udp4.encode(&mut rlp_data);
        }
        // udp6
        if let Some(udp6) = self.udp6 {
            UDP6_KEY.encode(&mut rlp_data);
            udp6.encode(&mut rlp_data);
        }

        rlp_data.to_vec()
    }
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum RlpEncodingError {
    #[error("Maximum record size exceeded")]
    MaximumEncodedByteLengthExceeded,
}

#[cfg(test)]
mod tests {
    use crate::enr::builder::Builder;
    use crate::enr::Schemev4;
    use hex_literal::hex;
    use std::net::Ipv4Addr;

    #[test]
    fn test_with_spec_sample_node() {
        let mut builder = Builder::new();
        builder.with_signature_value(hex!("7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c").to_vec())
            .with_seq(1)
            .with_id("v4")
            .with_ip4(Ipv4Addr::from([127, 0, 0, 1]))
            .with_udp4(30303)
            .with_public_key_value(hex!("03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138").to_vec());
        assert_eq!(builder.0.to_rlp::<Schemev4>(false), hex!("f84201826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f"));
        assert_eq!(builder.0.to_rlp::<Schemev4>(true), hex!("f884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f"));
    }
}
