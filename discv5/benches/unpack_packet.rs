// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use discv5::unpacking::{
    unpack, unpack_handshake_message, unpack_handshake_message_with_record,
    unpack_ordinary_message, unpack_whoareyou,
};
use enr::{NodeId, Schemev4};
use hex_literal::hex;

const EXAMPLE_PING_MESSAGE_PACKET_DATA: &[u8] = &hex!(
    "00000000000000000000000000000000088b3d4342774649325f313964a39e55"
    "ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
    "4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc"
);

const EXAMPLE_WHOAREYOU_PACKET_DATA: &[u8] = &hex!(
    "00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad"
    "1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d"
);

const EXAMPLE_PING_HANDSHAKE_PACKET: &[u8] = &hex!(
    "00000000000000000000000000000000088b3d4342774649305f313964a39e55"
    "ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
    "4c4f53245d08da4bb252012b2cba3f4f374a90a75cff91f142fa9be3e0a5f3ef"
    "268ccb9065aeecfd67a999e7fdc137e062b2ec4a0eb92947f0d9a74bfbf44dfb"
    "a776b21301f8b65efd5796706adff216ab862a9186875f9494150c4ae06fa4d1"
    "f0396c93f215fa4ef524f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d83"
    "9cf8");

const EXAMPLE_PING_HANDSHAKE_WITH_ENR_PACKET: &[u8] = &hex!(
    "00000000000000000000000000000000088b3d4342774649305f313964a39e55"
    "ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3"
    "4c4f53245d08da4bb23698868350aaad22e3ab8dd034f548a1c43cd246be9856"
    "2fafa0a1fa86d8e7a3b95ae78cc2b988ded6a5b59eb83ad58097252188b902b2"
    "1481e30e5e285f19735796706adff216ab862a9186875f9494150c4ae06fa4d1"
    "f0396c93f215fa4ef524e0ed04c3c21e39b1868e1ca8105e585ec17315e755e6"
    "cfc4dd6cb7fd8e1a1f55e49b4b5eb024221482105346f3c82b15fdaae36a3bb1"
    "2a494683b4a3c7f2ae41306252fed84785e2bbff3b022812d0882f06978df84a"
    "80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e"
    "4539717307a0208cd208d65093ccab5aa596a34d7511401987662d8cf62b1394"
    "71"
);

fn unpack_packet(c: &mut Criterion) {
    let dest_node_id_data =
        hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
    let dest_node_id = NodeId(dest_node_id_data);
    let read_key = hex!("00000000000000000000000000000000");
    let read_key_handshake = hex!("4f9fac6de7567d1e3b1241dffe90f662");
    let read_key_handshake_enr = hex!("53b1c075f41876423154e157470c2f48");

    c.bench_function("unpack_ping_message_packet", |b| {
        b.iter(|| {
            let (masking_iv, _, nonce, static_header, auth_data, encrypted_message_data) =
                unpack(&dest_node_id, EXAMPLE_PING_MESSAGE_PACKET_DATA).unwrap();

            let (_, _) = unpack_ordinary_message(
                &masking_iv,
                &read_key,
                &nonce,
                &static_header,
                &auth_data,
                encrypted_message_data,
            )
            .unwrap();
        })
    });

    c.bench_function("unpack_whoareyou_packet", |b| {
        b.iter(|| {
            let (_, _, _, static_header, auth_data, _) =
                unpack(&dest_node_id, EXAMPLE_WHOAREYOU_PACKET_DATA).unwrap();

            let (_, _) = unpack_whoareyou(&static_header, &auth_data).unwrap();
        })
    });

    c.bench_function("unpack_ping_handshake_packet", |b| {
        b.iter(|| {
            let (masking_iv, _, nonce, static_header, auth_data, encrypted_message_data) =
                unpack(&dest_node_id, EXAMPLE_PING_HANDSHAKE_PACKET).unwrap();

            let (_, _, _, _) = unpack_handshake_message::<Schemev4>(
                &masking_iv,
                &read_key_handshake,
                &nonce,
                &static_header,
                &auth_data,
                encrypted_message_data,
            )
            .unwrap();
        })
    });

    c.bench_function("unpack_ping_handshake_with_enr_packet", |b| {
        b.iter(|| {
            let (masking_iv, _, nonce, static_header, auth_data, encrypted_message_data) =
                unpack(&dest_node_id, EXAMPLE_PING_HANDSHAKE_WITH_ENR_PACKET).unwrap();

            let (_, _, _, _, _) = unpack_handshake_message_with_record::<Schemev4>(
                &masking_iv,
                &read_key_handshake_enr,
                &nonce,
                &static_header,
                &auth_data,
                encrypted_message_data,
            )
            .unwrap();
        })
    });
}

criterion_group!(benches, unpack_packet);
criterion_main!(benches);
