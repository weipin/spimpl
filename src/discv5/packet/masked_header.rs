// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rand::{CryptoRng, Rng};

use crate::discv5::crypto::aesctr;
use crate::enr::NodeId;

pub(crate) fn mask_header_data(
    dest_id: &NodeId,
    iv: &MaskingIv,
    header_data_pt_in_ct_out: &mut [u8],
) {
    // masked-header = aesctr_encrypt(masking-key, masking-iv, header)
    // masking-key   = dest-id[:16]
    aesctr::encrypt(
        &dest_id.0[..MASKING_KEY_BYTE_LENGTH],
        &iv.0,
        header_data_pt_in_ct_out,
    );
}

// masking-key   = dest-id[:16]
const MASKING_KEY_BYTE_LENGTH: usize = 16;

// masking-iv    = uint128   -- random data unique to packet
pub(crate) struct MaskingIv(pub(crate) [u8; 16]);

impl MaskingIv {
    fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        MaskingIv(csprng.gen())
    }
}
