// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::NodeId;
use ethnum::U256;

/// Base 2 logarithm of the 'distance' between two node IDs, rounded down.
/// [0, 255]
#[derive(Clone, Debug, PartialEq)]
pub struct NodeLog2Distance(pub u8);

/// [0, 256]
/// 0: returns the current node itself
/// 1-256: "the logarithmic distance (i.e. length of differing suffix in bits)"
#[derive(rlp::Encode, rlp::Decode, Clone, Copy, Debug, PartialEq)]
pub struct NodeProtocolDistance(pub u16);

/// Returns None if the 'distance' between two node IDs is 0.
pub fn log2_distance_between_node_ids(a: &NodeId, b: &NodeId) -> Option<NodeLog2Distance> {
    let a = U256::from_be_bytes(*a.bytes());
    let b = U256::from_be_bytes(*b.bytes());
    let distance = a ^ b;
    if distance == 0 {
        return None;
    }

    let rounded_down_log2 = (U256::BITS - 1) as u8 - distance.leading_zeros() as u8;
    Some(NodeLog2Distance(rounded_down_log2))
}

impl TryFrom<NodeProtocolDistance> for Option<NodeLog2Distance> {
    type Error = Error;

    fn try_from(value: NodeProtocolDistance) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(None),
            1..=256 => Ok(Some(NodeLog2Distance((value.0 - 1) as u8))),
            257..=u16::MAX => Err(Error::Overflow),
        }
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("node protocol distance overflow")]
    Overflow,
}

#[cfg(test)]
mod tests {
    use ::quickcheck_macros::quickcheck;
    use test_extensions::QuickCheckArray;

    use super::*;

    #[test]
    fn test_log2_distance_between_node_ids() {
        let test_data = [
            (
                "log2 distance 255",
                [0u8; 32],
                [255u8; 32],
                Some(NodeLog2Distance(255)),
            ),
            ("distance 0", [6; 32], [6; 32], None),
            (
                "log2 distance 0",
                [0; 32],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ],
                Some(NodeLog2Distance(0)),
            ),
            (
                "log2 distance 1",
                [0; 32],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 3,
                ],
                Some(NodeLog2Distance(1)),
            ),
        ];
        for (test_name, a, b, distance) in test_data {
            let a = NodeId::from_array(a);
            let b = NodeId::from_array(b);
            assert_eq!(
                log2_distance_between_node_ids(&a, &b),
                distance,
                "{test_name}"
            );
        }
    }

    #[test]
    fn test_protocol_to_log2() {
        let test_data = [
            (0, Ok(None) as Result<Option<NodeLog2Distance>, Error>),
            (1, Ok(Some(NodeLog2Distance(0)))),
            (255, Ok(Some(NodeLog2Distance(254)))),
            (256, Ok(Some(NodeLog2Distance(255)))),
            (257, Err(Error::Overflow)),
        ];

        for (protocol_distance, log2_distance) in test_data {
            let protocol_distance = NodeProtocolDistance(protocol_distance);
            assert_eq!(
                protocol_distance.try_into(),
                log2_distance,
                "protocol distance {protocol_distance:?}"
            );
        }
    }

    #[quickcheck]
    fn test_with_sigp_discv5(a: QuickCheckArray<u8, 32>, b: QuickCheckArray<u8, 32>) -> bool {
        let key1: sigp_discv5::Key<sigp_discv5::enr::NodeId> =
            sigp_discv5::enr::NodeId::new(&a.0).into();
        let key2: sigp_discv5::Key<sigp_discv5::enr::NodeId> =
            sigp_discv5::enr::NodeId::new(&b.0).into();
        let sigp_log2_distance = key1.log2_distance(&key2);

        let a = NodeId::from_array(a.0);
        let b = NodeId::from_array(b.0);
        let log2_distance = log2_distance_between_node_ids(&a, &b);

        if sigp_log2_distance.is_none() {
            return log2_distance.is_none();
        }
        u8::try_from(sigp_log2_distance.unwrap() - 1).unwrap() == log2_distance.unwrap().0
    }
}
