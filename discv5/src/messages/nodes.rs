// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::RecordRlpEncoded;

use crate::types::RequestId;

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct Nodes {
    pub request_id: RequestId,
    pub total: u64,
    pub rlp_encoded_records: Vec<RecordRlpEncoded>,
}

impl Message for Nodes {
    const TYPE: Type = Type::Nodes;
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use crate::messages;

    use super::*;

    #[test]
    fn nodes() {
        let request_id_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let request_id = RequestId::from_vec(request_id_vec).unwrap();
        let record_rlp_encoded1 = RecordRlpEncoded::from_textual_form("enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8").unwrap();
        let record_rlp_encoded2 = RecordRlpEncoded::from_textual_form("enr:-HW4QF9wuyyItfemQw2A77eAwwts7FRu-V8f7FLyIL04XJV5M0NJ2iaCcoByzCo9YoVWDDNY-_VMAVEobwrTLwcGD4wBgmlkgnY0iXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTg").unwrap();

        let nodes = Nodes {
            request_id,
            total: 2,
            rlp_encoded_records: vec![record_rlp_encoded1, record_rlp_encoded2],
        };
        let encoded = messages::encode(&nodes);
        // discv5_messages: `nodes_1`
        assert_eq!(encoded, hex!("04f9010988010203040506070802f8fdf884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765ff875b8405f70bb2c88b5f7a6430d80efb780c30b6cec546ef95f1fec52f220bd385c9579334349da2682728072cc2a3d6285560c3358fbf54c0151286f0ad32f07060f8c0182696482763489736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138"));

        // assert_eq!(rlp::decode::<Nodes>(&encoded).unwrap(), nodes);
    }
}
