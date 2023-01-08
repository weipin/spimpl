// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements storage RLP encoding.

use super::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use super::scheme::Scheme;
use super::storage::Storage;
use crate::rlp;
use crate::rlp::RlpItemType;

impl Storage {
    pub(crate) fn to_rlp<S: Scheme>(&self, with_signature: bool) -> Vec<u8> {
        debug_assert!(self.id.is_some());
        debug_assert!(self.public_key_value.is_some());

        let mut rlp_list_payload = vec![];

        // [signature, seq, k, v, ...]
        // signature
        if with_signature {
            debug_assert!(self.signature_value.is_some());
            let signature_value = self.signature_value.as_ref().unwrap().as_slice();
            rlp::encode(signature_value, &mut rlp_list_payload);
        }

        // seq
        rlp::encode(self.seq, &mut rlp_list_payload);

        // The key/value pairs must be sorted by key and must be unique:
        // id, ip, ip6, secp256k1, tcp, tcp6, udp, udp6

        // id
        rlp::encode(ID_KEY, &mut rlp_list_payload);
        rlp::encode(S::id(), &mut rlp_list_payload);

        // ip
        if let Some(ip4) = self.ip4 {
            rlp::encode(IP4_KEY, &mut rlp_list_payload);
            rlp::encode(&ip4.octets(), &mut rlp_list_payload);
        }

        // ip6
        if let Some(ip6) = self.ip6 {
            rlp::encode(IP6_KEY, &mut rlp_list_payload);
            rlp::encode(&ip6.octets(), &mut rlp_list_payload);
        }

        // public key value (secp256k1)
        let public_key_value = self.public_key_value.as_ref().unwrap().as_slice();
        rlp::encode(S::public_key_key(), &mut rlp_list_payload);
        rlp::encode(public_key_value, &mut rlp_list_payload);

        // tcp
        if let Some(tcp4) = self.tcp4 {
            rlp::encode(TCP4_KEY, &mut rlp_list_payload);
            rlp::encode(tcp4, &mut rlp_list_payload);
        }

        // tcp6
        if let Some(tcp6) = self.tcp6 {
            rlp::encode(TCP6_KEY, &mut rlp_list_payload);
            rlp::encode(tcp6, &mut rlp_list_payload);
        }

        // udp
        if let Some(udp4) = self.udp4 {
            rlp::encode(UDP4_KEY, &mut rlp_list_payload);
            rlp::encode(udp4, &mut rlp_list_payload);
        }

        // udp6
        if let Some(udp6) = self.udp6 {
            rlp::encode(UDP6_KEY, &mut rlp_list_payload);
            rlp::encode(udp6, &mut rlp_list_payload);
        }

        let mut rlp_data = vec![];
        rlp::encode_item(&mut rlp_data, RlpItemType::List, &rlp_list_payload);
        rlp_data
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
