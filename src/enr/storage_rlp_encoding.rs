// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements storage RLP encoding.

use super::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use super::scheme::Scheme;
use super::storage::Storage;
use bytes::BytesMut;
use fastrlp::{Encodable, Header};

impl Storage {
    pub(crate) fn to_rlp<S: Scheme>(&self, with_signature: bool) -> Vec<u8> {
        debug_assert!(self.id.is_some());
        debug_assert!(self.public_key_value.is_some());

        // TODO: with_capacity
        let mut rlp_header_data = BytesMut::new();
        let mut rlp_items_data = BytesMut::new();
        let mut header = Header {
            list: true,
            payload_length: 0,
        };

        // [signature, seq, k, v, ...]
        // signature
        if with_signature {
            debug_assert!(self.signature_value.is_some());
            let signature_value = self.signature_value.as_ref().unwrap().as_slice();
            header.payload_length += signature_value.length();
            signature_value.encode(&mut rlp_items_data);
        }

        // seq
        header.payload_length += self.seq.length();
        self.seq.encode(&mut rlp_items_data);

        // The key/value pairs must be sorted by key and must be unique:
        // id, ip, ip6, secp256k1, tcp, tcp6, udp, udp6

        // id
        header.payload_length += ID_KEY.length();
        header.payload_length += S::id().length();
        ID_KEY.encode(&mut rlp_items_data);
        S::id().encode(&mut rlp_items_data);

        // ip
        if let Some(ip4) = self.ip4 {
            header.payload_length += IP4_KEY.length();
            header.payload_length += ip4.octets().length();
            IP4_KEY.encode(&mut rlp_items_data);
            ip4.octets().encode(&mut rlp_items_data);
        }

        // ip6
        if let Some(ip6) = self.ip6 {
            header.payload_length += IP6_KEY.length();
            header.payload_length += ip6.octets().length();
            IP6_KEY.encode(&mut rlp_items_data);
            ip6.octets().encode(&mut rlp_items_data);
        }

        // public key value (secp256k1)
        let public_key_value = self.public_key_value.as_ref().unwrap().as_slice();
        header.payload_length += S::public_key_key().length();
        header.payload_length += public_key_value.length();
        S::public_key_key().encode(&mut rlp_items_data);
        public_key_value.encode(&mut rlp_items_data);

        // tcp
        if let Some(tcp4) = self.tcp4 {
            header.payload_length += TCP4_KEY.length();
            header.payload_length += tcp4.length();
            TCP4_KEY.encode(&mut rlp_items_data);
            tcp4.encode(&mut rlp_items_data);
        }

        // tcp6
        if let Some(tcp6) = self.tcp6 {
            header.payload_length += TCP6_KEY.length();
            header.payload_length += tcp6.length();
            TCP6_KEY.encode(&mut rlp_items_data);
            tcp6.encode(&mut rlp_items_data);
        }

        // udp
        if let Some(udp4) = self.udp4 {
            header.payload_length += UDP4_KEY.length();
            header.payload_length += udp4.length();
            UDP4_KEY.encode(&mut rlp_items_data);
            udp4.encode(&mut rlp_items_data);
        }

        // udp6
        if let Some(udp6) = self.udp6 {
            header.payload_length += UDP6_KEY.length();
            header.payload_length += udp6.length();
            UDP6_KEY.encode(&mut rlp_items_data);
            udp6.encode(&mut rlp_items_data);
        }

        header.encode(&mut rlp_header_data);
        [rlp_header_data, rlp_items_data].concat()
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
            .with_id(b"v4")
            .with_ip4(Ipv4Addr::from([127, 0, 0, 1]))
            .with_udp4(30303)
            .with_public_key_value(hex!("03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138").to_vec());
        assert_eq!(builder.0.to_rlp::<Schemev4>(false), hex!("f84201826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f"));
        assert_eq!(builder.0.to_rlp::<Schemev4>(true), hex!("f884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765f"));
    }
}
