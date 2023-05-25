// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use enr::RecordRlpEncoded;

use crate::types::RequestId;

use super::{Message, Type};

#[derive(Debug, PartialEq)]
pub struct Nodes<'a> {
    pub request_id: RequestId<'a>,
    pub total: u64,
    pub records: Vec<RecordRlpEncoded<'a>>,
}

impl<'a> Message<'a> for Nodes<'a> {
    const TYPE: Type = Type::Nodes;
}

// Implements the trait `rlp::Encode` manually instead of leveraging
// `#[derive(rlp::Encode)]`, to handle the exception of `RecordRlpEncoded` --
// encoding the bytes as it is (rlp data).
impl rlp::Encode for Nodes<'_> {
    fn encode_to(&self, output: &mut Vec<u8>) {
        let mut payload = vec![];
        rlp::encode_to(&self.request_id, &mut payload);
        rlp::encode_to(&self.total, &mut payload);

        let mut records_payload = vec![];
        self.records.iter().for_each(|element| {
            // Encodes as it is
            records_payload.extend(element.bytes());
        });
        rlp::ItemPayloadSlice(&records_payload).encode_as_list(&mut payload);

        rlp::ItemPayloadSlice(&payload).encode_as_list(output);
    }
}

// Implements the trait `rlp::Decode` manually instead of leveraging
// `#[derive(rlp::Decode)]`, to handle the exception of `RecordRlpEncoded` --
// decoding the bytes as it is (rlp data).
impl<'a> rlp::Decode<'a> for Nodes<'a> {
    const TYPE: rlp::ItemType = rlp::ItemType::List;

    fn decode(payload: rlp::ItemPayloadSlice<'a>) -> Result<Self, rlp::Error> {
        let mut list_iter = payload.list_iter_unchecked();
        let request_id: RequestId<'a> = list_iter.next_item()?;
        let total: u64 = list_iter.next_item()?;

        // let (item_type, item_payload) = (list_iter
        //     .next()
        //     .ok_or(rlp::Error::ListDecodingNumberDoesNotMatch)?)?;
        let (item_type, item_payload) = match list_iter.next() {
            Some(result) => result?,
            None => return Err(rlp::Error::ListDecodingNumberDoesNotMatch),
        };
        if item_type != rlp::ItemType::List {
            return Err(rlp::Error::ItemTypeDoesNotMatch);
        }
        let mut records_list_iter = item_payload.list_iter_unchecked();
        let mut records = vec![];
        while let Some(result) = records_list_iter.next_itemdata() {
            match result {
                Ok((item_type, _, item_data)) => {
                    if item_type != rlp::ItemType::List {
                        return Err(rlp::Error::ItemTypeDoesNotMatch);
                    }

                    // Decodes as it is
                    let record = RecordRlpEncoded::from_slice(item_data.0).map_err(|e| {
                        if e == enr::Error::MaximumRecordRlpEncodedByteLengthExceeded {
                            rlp::Error::ItemPayloadByteLengthTooLarge
                        } else {
                            rlp::Error::InvalidByteRepresentaion
                        }
                    })?;
                    records.push(record);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if list_iter.next().is_some() {
            return Err(rlp::Error::ListDecodingNumberDoesNotMatch);
        }

        Ok(Nodes {
            request_id,
            total,
            records,
        })
    }
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
        let record1 = RecordRlpEncoded::from_textual_form("enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8").unwrap();
        let record2 = RecordRlpEncoded::from_textual_form("enr:-HW4QF9wuyyItfemQw2A77eAwwts7FRu-V8f7FLyIL04XJV5M0NJ2iaCcoByzCo9YoVWDDNY-_VMAVEobwrTLwcGD4wBgmlkgnY0iXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTg").unwrap();

        let nodes = Nodes {
            request_id,
            total: 2,
            records: vec![record1, record2],
        };
        let encoded = messages::encode(&nodes);
        // discv5_playground: `nodes_1`
        assert_eq!(encoded, hex!("04f9010988010203040506070802f8fdf884b8407098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c01826964827634826970847f00000189736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd31388375647082765ff875b8405f70bb2c88b5f7a6430d80efb780c30b6cec546ef95f1fec52f220bd385c9579334349da2682728072cc2a3d6285560c3358fbf54c0151286f0ad32f07060f8c0182696482763489736563703235366b31a103ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138"));

        assert_eq!(rlp::decode::<Nodes>(&encoded[1..]).unwrap(), nodes);
    }
}
