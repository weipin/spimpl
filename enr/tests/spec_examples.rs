// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::Ipv4Addr;

use hex_literal::hex;

use enr::{Builder, Record, Scheme, Schemev4};

const EXAMPLE_RECORD_ADDRESS: &str = "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";
// eth_enr_v4.py: `example_record`
const EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY: &str = "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFzZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";
const PRIVATE_KEY_DATA: &[u8] =
    &hex!("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291");
const EXAMPLE_IP4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const EXAMPLE_UDP4: u16 = 30303;

#[test]
fn build_immutable_example_record() {
    let private_key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
    let public_key = Schemev4::new_public_key_from_private_key(&private_key);
    let record = Builder::new::<Schemev4>()
        .with_seq(1)
        .with_ip4(EXAMPLE_IP4)
        .with_udp4(EXAMPLE_UDP4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap();

    let record_address = record.to_textual_form::<Schemev4>().unwrap();
    // Ensures mock data is not used in the production environment.
    assert_ne!(record_address, EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY);
}

#[test]
fn create_records_from_textual_form() {
    let record = Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS).unwrap();
    assert_eq!(record.seq(), 1);
    assert_eq!(record.ip4().unwrap(), EXAMPLE_IP4);
    assert_eq!(record.udp4().unwrap(), EXAMPLE_UDP4);

    let record =
        Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY).unwrap();
    assert_eq!(record.seq(), 1);
    assert_eq!(record.ip4().unwrap(), EXAMPLE_IP4);
    assert_eq!(record.udp4().unwrap(), EXAMPLE_UDP4);
}

#[test]
fn test_publishable_record() {
    let private_key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
    let public_key = Schemev4::new_public_key_from_private_key(&private_key);
    let mut publishable_record = Builder::new::<Schemev4>()
        .with_seq(1)
        .with_ip4(EXAMPLE_IP4)
        .with_udp4(EXAMPLE_UDP4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap()
        .to_publishable::<Schemev4>();

    let (seq, _) = publishable_record
        .publish::<Schemev4>(&private_key)
        .unwrap();
    assert_eq!(seq, 1);

    publishable_record.update_ip4(EXAMPLE_IP4);
    let (seq, _) = publishable_record
        .publish::<Schemev4>(&private_key)
        .unwrap();
    assert_eq!(seq, 1);

    publishable_record.update_ip4(Ipv4Addr::new(192, 168, 0, 1));
    publishable_record.update_udp4(u16::MAX);
    let (seq, address) = publishable_record
        .publish::<Schemev4>(&private_key)
        .unwrap();
    assert_eq!(seq, 2);

    let record = Record::from_textual_form::<Schemev4>(&address).unwrap();
    assert_eq!(record.seq(), 2);
    assert_eq!(record.ip4().unwrap(), Ipv4Addr::new(192, 168, 0, 1));
    assert_eq!(record.udp4().unwrap(), u16::MAX);
}
