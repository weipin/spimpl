// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Key names with pre-defined meaning.

/// Key name of identity scheme, e.g. "v4".
pub const ID_KEY: &[u8] = b"id";
/// Key name of IPv4 address.
pub const IP4_KEY: &[u8] = b"ip";
/// Key name of IPv6 address.
pub const IP6_KEY: &[u8] = b"ip6";
/// Key name of TCP port.
pub const TCP4_KEY: &[u8] = b"tcp";
/// Key name of IPv6-specific TCP port.
pub const TCP6_KEY: &[u8] = b"tcp6";
/// Key name of UDP port.
pub const UDP4_KEY: &[u8] = b"udp";
/// Key name of IPv6-specific UDP port.
pub const UDP6_KEY: &[u8] = b"udp6";

/// Key name of compressed secp256k1 public key
pub const SCHEME_V4_KEY_OF_PUBLIC_KEY: &[u8] = b"secp256k1";
