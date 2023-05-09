// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implements `Content` decoding from its RLP form.

use rlp::ListIter;

use crate::content::Content;
use crate::predefined_keys::{ID_KEY, IP4_KEY, IP6_KEY, TCP4_KEY, TCP6_KEY, UDP4_KEY, UDP6_KEY};
use crate::{Error, Scheme};

impl Content {
    /// Creates a `Content` from `iter`.
    ///
    /// The signature data should have already been decoded (consumed) from the
    /// `iter` if present.
    pub(crate) fn from_rlp_list_iter<S: Scheme>(iter: &mut ListIter) -> Result<Self, Error> {
        let mut content = Content::new(b"");
        // Decodes sequence number.
        content.seq = iter.next_item().map_err(|e| {
            if e == rlp::Error::ListDecodingIterationEnded {
                Error::SeqNotFound
            } else {
                Error::RlpDecodingError(e)
            }
        })?;

        // Decodes the key/value pairs.
        //
        // "The key/value pairs must be sorted by key and must be unique."
        let mut previous_key: &[u8] = b"";
        loop {
            let key: &[u8] = match iter.next_item() {
                Ok(key) => key,
                Err(err) => {
                    if err == rlp::Error::ListDecodingIterationEnded {
                        break;
                    }
                    return Err(Error::RlpDecodingError(err));
                }
            };

            if key <= previous_key {
                return Err(Error::KeysNotSortedOrNotUnique);
            }
            previous_key = key;

            match key {
                _ if key == S::key_of_public_key() => {
                    let public_key_data: Vec<u8> = iter.next_item().map_err(pair_value_error)?;
                    if public_key_data.len() != S::ENR_REQUIRED_PUBLIC_KEY_BYTE_LENGTH {
                        return Err(Error::PublicKeyDataWithInvalidByteLength);
                    }
                    content.public_key_data = Some(public_key_data);
                }
                ID_KEY => {
                    let id: &[u8] = iter.next_item().map_err(pair_value_error)?;
                    if id == S::id() {
                        content.id = S::id();
                    } else {
                        return Err(Error::SchemeNameNotRecognized);
                    }
                }
                IP4_KEY => {
                    content.ip4 = Some(iter.next_item().map_err(pair_value_error)?);
                }
                IP6_KEY => {
                    content.ip6 = Some(iter.next_item().map_err(pair_value_error)?);
                }
                TCP4_KEY => {
                    content.tcp4 = Some(iter.next_item().map_err(pair_value_error)?);
                }
                TCP6_KEY => {
                    content.tcp6 = Some(iter.next_item().map_err(pair_value_error)?);
                }
                UDP4_KEY => {
                    content.udp4 = Some(iter.next_item().map_err(pair_value_error)?);
                }
                UDP6_KEY => {
                    content.udp6 = Some(iter.next_item().map_err(pair_value_error)?);
                }

                // Ignores unrecognized keys.
                _ => match iter.next() {
                    None => return Err(Error::PairValueNotFound),
                    Some(Ok(_)) => continue,
                    Some(Err(e)) => return Err(Error::RlpDecodingError(e)),
                },
            }
        }

        // `id` isn't present.
        if content.id != S::id() {
            return Err(Error::SchemeNameNotRecognized);
        }

        Ok(content)
    }
}

// Convenience function for error handling.
fn pair_value_error(rlp_err: rlp::Error) -> Error {
    if rlp_err == rlp::Error::ListDecodingIterationEnded {
        return Error::PairValueNotFound;
    }
    Error::RlpDecodingError(rlp_err)
}
