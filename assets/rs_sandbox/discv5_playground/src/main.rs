use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use hex_literal::hex;

use discv5::enr::{CombinedKey, Enr};
use discv5::rpc::{self, Message};

fn ping_1() {
    let enr_seq = 7;

    let message = Message::Request(rpc::Request {
        id: foo_request_id(),
        body: rpc::RequestBody::Ping { enr_seq },
    });
    println!("ping_1: {}", hex::encode(message.encode()));
}

fn ping_2() {
    let request_id = rpc::RequestId(hex!("00000001").to_vec());
    let enr_seq = 2;

    let message = Message::Request(rpc::Request {
        id: request_id,
        body: rpc::RequestBody::Ping { enr_seq },
    });
    println!("ping_2: {}", hex::encode(message.encode()));
}

fn ping_3() {
    let request_id = rpc::RequestId(hex!("00000001").to_vec());
    let enr_seq = 1;

    let message = Message::Request(rpc::Request {
        id: request_id,
        body: rpc::RequestBody::Ping { enr_seq },
    });
    println!("ping_3: {}", hex::encode(message.encode()));
}

fn pong_ipv4_ipv6() {
    let enr_seq = 7;
    let ip4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ip6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff));
    let port = u16::MAX;

    let message = Message::Response(rpc::Response {
        id: foo_request_id(),
        body: rpc::ResponseBody::Pong {
            enr_seq,
            ip: ip4,
            port,
        },
    });
    println!("pong_ipv4: {}", hex::encode(message.encode()));

    let message = Message::Response(rpc::Response {
        id: foo_request_id(),
        body: rpc::ResponseBody::Pong {
            enr_seq,
            ip: ip6,
            port,
        },
    });
    println!("pong_ipv6: {}", hex::encode(message.encode()));
}

fn findnode_1() {
    let distances = vec![1, 2, 3];

    let message = Message::Request(rpc::Request {
        id: foo_request_id(),
        body: rpc::RequestBody::FindNode { distances },
    });
    println!("findnode_1: {}", hex::encode(message.encode()));
}

fn nodes_1() {
    // eth_enr_v4: `example_record_without_extra_entropy`
    let enr1_address = "-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";
    // `minimal_record`
    let enr2_address = "-HW4QF9wuyyItfemQw2A77eAwwts7FRu-V8f7FLyIL04XJV5M0NJ2iaCcoByzCo9YoVWDDNY-_VMAVEobwrTLwcGD4wBgmlkgnY0iXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTg";
    let enr1 = enr1_address.parse::<Enr<CombinedKey>>().unwrap();
    let enr2 = enr2_address.parse::<Enr<CombinedKey>>().unwrap();

    let message = Message::Response(rpc::Response {
        id: foo_request_id(),
        body: rpc::ResponseBody::Nodes {
            total: 2,
            nodes: vec![enr1, enr2],
        },
    });
    println!("nodes_1: {}", hex::encode(message.encode()));
}

fn talkreq_1() {
    let message = Message::Request(rpc::Request {
        id: foo_request_id(),
        body: rpc::RequestBody::Talk {
            protocol: vec![1, 2, 3],
            request: vec![4, 5, 6],
        },
    });
    println!("talkreq_1: {}", hex::encode(message.encode()));
}

fn talkresp_1() {
    let message = Message::Response(rpc::Response {
        id: foo_request_id(),
        body: rpc::ResponseBody::Talk {
            response: vec![7, 8, 9],
        },
    });
    println!("talkresp_1: {}", hex::encode(message.encode()));
}

fn foo_request_id() -> rpc::RequestId {
    rpc::RequestId(vec![1, 2, 3, 4, 5, 6, 7, 8])
}

fn main() {
    ping_1();
    ping_2();
    ping_3();
    pong_ipv4_ipv6();
    findnode_1();
    nodes_1();
    talkreq_1();
    talkresp_1();
}
