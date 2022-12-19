// Copyright 2022 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use crate::enr::{Scheme, Schemev4};
    use hex_literal::hex;

    #[test]
    fn test_ecdh() {
        // https://github.com/ethereum/devp2p/blob/master/discv5/discv5-wire-test-vectors.md#ecdh
        let public_key_data =
            hex!("039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231");
        let private_key_data =
            hex!("fb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736");

        let public_key = Schemev4::value_to_public_key(&public_key_data).unwrap();
        let private_key = Schemev4::value_to_private_key(&private_key_data).unwrap();
        let shared_secret_data = Schemev4::ecdh(&public_key, &private_key);
        assert_eq!(
            shared_secret_data,
            hex!("033b11a2a1f214567e1537ce5e509ffd9b21373247f2a3ff6841f4976f53165e7e")
        );
    }
}
