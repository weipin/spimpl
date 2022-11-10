// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// The one from the spec:
// https://github.com/ethereum/devp2p/blob/master/enr.md
pub(crate) const EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjz",
    "CBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
);

// Uses the example record from the ENR spec.
//
// This implementation creates ECDSA signatures with additional random data.
// Under the unit testing environment, the mock value `MOCK_ECDSA_NONCE_ADDITIONAL_DATA`
// is always used.
//
// The expected ENR textual form `expected_enr_base64` is constructed by a Python script:
// ```
// key = SigningKey.from_secret_exponent(
//     0xb71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291, curve=SECP256k1)
//
// # Builds content RLP
// rlp_data = encode([1, 'id', 'v4', 'ip', 0x7f000001, 'secp256k1', bytes.fromhex(
//     '03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138'), 'udp', 0x765f])
// rlp_data_hash = keccak(rlp_data)
//
// # Signs the content RLP **with** the additional data.
// additional_data = bytes.fromhex(
//     'baaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaad')
// content_signature = key.sign_digest_deterministic(rlp_data_hash, hashfunc=sha256,
//                                                   sigencode=sigencode_string_canonize,
//                                                   extra_entropy=additional_data)
// rlp_with_signature = encode(
//     [content_signature, 1, 'id', 'v4', 'ip', 0x7f000001, 'secp256k1', bytes.fromhex(
//         '03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138'), 'udp', 0x765f])
// textual_form = "enr:" + urlsafe_b64encode(rlp_with_signature).decode('utf-8').rstrip('=')
// ```
pub(crate) const MOCKED_EXAMPLE_RECORD_ADDRESS: &str = concat!(
    "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFz",
    "ZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsx",
    "oQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8",
);
