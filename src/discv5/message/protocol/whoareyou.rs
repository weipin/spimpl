// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::enr;

// authdata      = id-nonce || enr-seq
// authdata-size = 24
// id-nonce      = uint128   -- random bytes
// enr-seq       = uint64    -- ENR sequence number of the requesting node
pub(crate) struct IdNonce(pub(crate) [u8; 16]);

pub(crate) struct Whoareyou {
    pub(crate) id_nonce: IdNonce,
    pub(crate) enr_seq: enr::SequenceNumber,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discv5::auth_data::handshake::HandshakeInitialMessageAuthData;
    use crate::discv5::message;
    use crate::discv5::message::protocol::whoareyou::{IdNonce, Whoareyou};
    use crate::discv5::packet::flag::Flag;
    use crate::discv5::packet::header::{
        encode_fixed_size_header_to_buffer, fixed_size_encoded_header_byte_length,
    };
    use crate::discv5::packet::masked_header::MaskingIv;
    use crate::discv5::packet::packing_whoareyou::pack_whoareyou;
    use crate::enr::{NodeId, Schemev4};
    use hex_literal::hex;

    #[test]
    fn test_packaging_whoareyou() {
        let request_nonce_data = hex!("0102030405060708090a0b0c");
        let id_nonce_data = hex!("0102030405060708090a0b0c0d0e0f10");
        let enr_seq = 0;
        let dest_node_id_data =
            hex!("bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9");
        let masking_iv_data = hex!("00000000000000000000000000000000");

        let whoareyou = Whoareyou {
            id_nonce: IdNonce(id_nonce_data),
            enr_seq,
        };
        let mut header_data = Vec::with_capacity(fixed_size_encoded_header_byte_length::<
            HandshakeInitialMessageAuthData<Schemev4>,
        >());
        encode_fixed_size_header_to_buffer(
            &mut header_data,
            &whoareyou,
            Flag::Whoareyou,
            &message::Nonce(request_nonce_data),
        );
        let package_data = pack_whoareyou(
            &mut header_data,
            &NodeId(dest_node_id_data),
            &MaskingIv(masking_iv_data),
        );
        assert_eq!(
            package_data,
            hex!(
                "00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad
                 1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d"
            )
        );
    }
}
