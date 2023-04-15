// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements Recursive-length prefix (RLP) serialization.
//!
//! - [Ethereum Yellow Paper][1], Appendix B. Recursive Length Prefix
//! - [Recursive Length Prefix (RLP) serialization][2]
//! - [Ethereum's Recursive Length Prefix in ACL2][3]
//!
//!
//! [1]: https://ethereum.github.io/yellowpaper/paper.pdf
//! [2]: https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/
//! [3]: https://arxiv.org/abs/2009.13769

#![warn(missing_docs)]

pub(crate) mod constants;
mod decoder;
mod decoding;
mod encoder;
mod encoding;
mod error;
mod list_iter;
mod prelude;
mod traits;
mod types;

pub use decoder::{decode, decode_payload};
pub use decoding::decode_header_unchecked;
pub use encoder::{encode, encode_to};
pub use error::Error;
pub use list_iter::ListIter;
pub use prelude::U8;
pub use traits::{Decode, Encode};
pub use types::{
    ByteLengthOfPayloadByteLength, ItemDataSlice, ItemPayloadSlice, ItemType, PayloadByteLength,
};
