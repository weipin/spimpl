// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::types::{NodeProtocolDistance, RequestId};

use super::{Message, Type};

#[derive(rlp::Encode, rlp::Decode, Debug, PartialEq)]
pub struct FindNode<'a> {
    pub request_id: RequestId<'a>,
    pub distances: Vec<NodeProtocolDistance>,
}

impl<'a> Message<'a> for FindNode<'a> {
    const TYPE: Type = Type::FindNode;
}

#[cfg(test)]
mod tests {
    use crate::messages;

    use super::*;

    #[test]
    fn test_findnode() {
        let request_id_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let request_id = RequestId::from_vec(request_id_vec).unwrap();
        let findnode = FindNode {
            request_id,
            distances: vec![
                NodeProtocolDistance(1),
                NodeProtocolDistance(2),
                NodeProtocolDistance(3),
            ],
        };

        let encoded = messages::encode(&findnode);
        // discv5_messages: `findnode_1`
        assert_eq!(encoded, hex_literal::hex!("03cd880102030405060708c3010203"));

        assert_eq!(rlp::decode::<FindNode>(&encoded[1..]).unwrap(), findnode);
    }
}
