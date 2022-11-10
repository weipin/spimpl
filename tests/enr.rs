// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use hex_literal::hex;
use secp256k1::{PublicKey, SecretKey, SECP256K1};
use spimpl::enr::{Builder, Record, Schemev4};
use std::net::Ipv4Addr;

// See "testing_helper.rs" for details.
const EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjz",
    "CBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
);

const MOCKED_EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFz",
    "ZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
);

#[test]
fn build_immutable_record() {
    let private_key = SecretKey::from_slice(&hex!(
        "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
    ))
    .unwrap();
    let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));
    let record = Builder::new()
        .with_ip4(ip4)
        .with_udp4(udp4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap();

    // Ensures the mock data is not used in the production environment.
    // See "testing_helper.rs" for details.
    assert_ne!(
        record.textual_form::<Schemev4>().unwrap(),
        MOCKED_EXAMPLE_RECORD_ADDRESS
    );
    assert_eq!(record.ip4().unwrap(), ip4);
    assert_eq!(record.udp4().unwrap(), udp4);
}

#[test]
fn build_update_mutable_record() {
    let private_key = SecretKey::from_slice(&hex!(
        "b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291"
    ))
    .unwrap();
    let public_key = PublicKey::from_secret_key(SECP256K1, &private_key);
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));
    let mut publishable_record = Builder::new()
        .with_ip4(ip4)
        .with_udp4(udp4)
        .sign_and_build::<Schemev4>(&private_key, &public_key)
        .unwrap()
        .to_publishable();

    assert_eq!(publishable_record.ip4().unwrap(), ip4);
    let (seq1, textual_form1) = publishable_record.publish::<Schemev4>().unwrap();
    assert_eq!(seq1, 1);

    let ip4_2 = Ipv4Addr::from(hex!("7f000002"));
    publishable_record.update_ip4(ip4_2);
    assert_eq!(publishable_record.ip4().unwrap(), ip4_2);
    let (seq2, textual_form2) = publishable_record.publish::<Schemev4>().unwrap();
    assert_eq!(seq2, 2);
    assert_ne!(textual_form1, textual_form2);
}

#[test]
fn record_from_textual() {
    let record = Record::from_textual_form::<Schemev4>(EXAMPLE_RECORD_ADDRESS).unwrap();
    let ip4 = Ipv4Addr::from(hex!("7f000001"));
    let udp4 = u16::from_be_bytes(hex!("765f"));

    assert_eq!(record.ip4().unwrap(), ip4);
    assert_eq!(record.udp4().unwrap(), udp4);
}
