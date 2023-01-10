// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::rlp;
use rand::{CryptoRng, Rng};

// nonce         = uint96    -- nonce of message
pub(crate) struct Nonce(pub(crate) [u8; 12]);

impl Nonce {
    fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        Nonce(csprng.gen())
    }
}

// The first element of every message-data list is the request ID.
// request-id is an RLP byte array of length <= 8 bytes. For requests,
// this value is assigned by the requester.
// The recipient of a message must mirror the value in the request-id element of the response.
// The selection of appropriate values for request IDs is left to the implementation.
#[derive(Debug)] // RlpEncodable
pub(crate) struct RequestId(pub(crate) Vec<u8>);

impl rlp::Encodable for &RequestId {
    fn encode(self, output: &mut Vec<u8>) {
        rlp::encode(self.0.as_slice(), output);
    }
}
