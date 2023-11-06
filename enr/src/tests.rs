// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use hex_literal::hex;
    use rand::rngs::OsRng;

    use crate::constants::SEQUENCE_NUMBER_INITIAL;
    use crate::{
        Builder, Error, Record, Scheme, SchemeKeyPair, Schemev4, Schemev4Secp256k1, SeqNum,
    };

    // eth_enr: `example_record`
    const EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY: &str = "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFzZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8";

    const PRIVATE_KEY_DATA: &[u8] =
        &hex!("b71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291");
    const EXAMPLE_IP4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const EXAMPLE_IP6: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
    const EXAMPLE_UDP4: u16 = 30303;
    const EXAMPLE_TCP4: u16 = 30302;
    const EXAMPLE_UDP6: u16 = u16::MAX;
    const EXAMPLE_TCP6: u16 = u16::MAX - 1;

    type SigpDefaultEnr = sigp_enr::Enr<sigp_enr::k256::ecdsa::SigningKey>;

    #[test]
    fn test_record_to_address_and_from_address_libsecp256k1() {
        test_record_to_address_and_from_address_with_scheme::<Schemev4Secp256k1>();
    }

    #[cfg(feature = "k256")]
    #[test]
    fn test_record_to_address_and_from_address_k256() {
        test_record_to_address_and_from_address_with_scheme::<crate::Schemev4K256>();
    }

    fn test_record_to_address_and_from_address_with_scheme<S: Scheme>() {
        let test_data = [
            (
                // eth_enr_v4.py: `example_record`
                "example_record",
                false,
                ContentData {
                    seq: 1,
                    ip4: Some(EXAMPLE_IP4),
                    udp4: Some(EXAMPLE_UDP4),
                    ..Default::default()
                },
                EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY,
            ),
            (
                "construct_with_signature_and_content_items",
                false,
                ContentData {
                    seq: 1,
                    ip4: Some(EXAMPLE_IP4),
                    udp4: Some(EXAMPLE_UDP4),
                    ..Default::default()
                },
                EXAMPLE_RECORD_ADDRESS_WITH_EXTRA_ENTROPY,
            ),
            (
                "minimal_record",
                false,
                ContentData {
                    seq: 1,
                    ..Default::default()
                },
                "enr:-HW4QF9wuyyItfemQw2A77eAwwts7FRu-V8f7FLyIL04XJV5M0NJ2iaCcoByzCo9YoVWDDNY-_VMAVEobwrTLwcGD4wBgmlkgnY0iXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTg",
            ),
            (
                "full_record",
                false,
                ContentData {
                    seq: 1,
                    ip4: Some(EXAMPLE_IP4),
                    udp4: Some(EXAMPLE_UDP4),
                    tcp4: Some(EXAMPLE_TCP4),
                    ip6: Some(EXAMPLE_IP6),
                    udp6: Some(EXAMPLE_UDP6),
                    tcp6: Some(EXAMPLE_TCP6),
                },
                "enr:-LC4QAqty4LtjwXT4hZ8npmnvtYYinC6xs2UcKMV0X-Kj52YbY8VFeEbO-KeRwpti67IYynVRze8rrkvkgu52zMTo4UBgmlkgnY0gmlwhH8AAAGDaXA2kAAAAAAAAAAAAAD__8AKAv-Jc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN0Y3CCdl6EdGNwNoL__oN1ZHCCdl-EdWRwNoL__w",
            ),
            (
                "example_record_mixed_with_unknown_pairs_address",
                true,
                ContentData {
                    seq: 1,
                    ip4: Some(EXAMPLE_IP4),
                    udp4: Some(EXAMPLE_UDP4),
                    ..Default::default()
                },
                "enr:-Km4QBamCR-qZyVpUKk3yhJ4g9qDS5Yvt1U0eTl-1CHXgRI4R92EwjNbfE9LmBqUVkE5yDWq_hqTQCsDNdnUwSMKKfEBgmFfg3h4eIJpZIJ2NINpZV-DeHh4gmlwhH8AAAGDaXpfg3h4eIlzZWNwMjU2azGhA8pjTK4NSay0Adikxrb-jFW3DRFb9AB2nMFADzJYzTE4gnN6g3h4eIN1ZHCCdl-CdXqDeHh4",
            ),
            (
                "record_encoded_size_eq_300_base64_size_eq_400",
                true,
                ContentData {
                    seq: 1,
                    ip4: Some(EXAMPLE_IP4),
                    udp4: Some(EXAMPLE_UDP4),
                    ..Default::default()
                },
                "enr:-QEpuEDaLyrPP4gxBI9YL7QE9U1tZig_Nt8rue8bRIuYv_IMziFc8OEt3LQMwkwt6da-Z0Y8BaqkDalZbBq647UtV2eiAYJpZIJ2NIJpcIR_AAABiXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTiDdWRwgnZferiieHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4",
            ),
        ];

        let private_key = S::new_private_key_from_bytes(&PRIVATE_KEY_DATA).unwrap();
        let scheme_keypair = SchemeKeyPair::from_private_key(private_key);

        for (test_name, test_from_address_only, content_data, address) in test_data {
            if !test_from_address_only {
                let mut builder = Builder::new::<S>();
                builder.with_seq(content_data.seq);
                if let Some(ip4) = content_data.ip4 {
                    builder.with_ip4(ip4);
                }
                if let Some(tcp4) = content_data.tcp4 {
                    builder.with_tcp4(tcp4);
                }
                if let Some(udp4) = content_data.udp4 {
                    builder.with_udp4(udp4);
                }
                if let Some(ip6) = content_data.ip6 {
                    builder.with_ip6(ip6);
                }
                if let Some(tcp6) = content_data.tcp6 {
                    builder.with_tcp6(tcp6);
                }
                if let Some(udp6) = content_data.udp6 {
                    builder.with_udp6(udp6);
                }
                let record = builder.sign_and_build::<S>(&scheme_keypair).unwrap();
                let record_address = record.to_textual_form::<S>().unwrap();
                assert_eq!(record_address, address, "{test_name}");
            }

            let record = Record::from_textual_form::<S>(address).unwrap();
            assert_eq!(record.id(), S::id());
            assert_eq!(record.seq(), 1);
            assert_eq!(record.ip4(), content_data.ip4);
            assert_eq!(record.udp4(), content_data.udp4);
            assert_eq!(record.tcp4(), content_data.tcp4);
            assert_eq!(record.ip6(), content_data.ip6);
            assert_eq!(record.udp6(), content_data.udp6);
            assert_eq!(record.tcp6(), content_data.tcp6);

            let sigp_enr = address.parse::<SigpDefaultEnr>().unwrap();
            assert_eq!(sigp_enr.id().unwrap().as_bytes(), S::id());
            assert_eq!(sigp_enr.seq(), 1);
            assert_eq!(sigp_enr.ip4(), content_data.ip4);
            assert_eq!(sigp_enr.udp4(), content_data.udp4);
            assert_eq!(sigp_enr.tcp4(), content_data.tcp4);
            assert_eq!(sigp_enr.ip6(), content_data.ip6);
            assert_eq!(sigp_enr.udp6(), content_data.udp6);
            assert_eq!(sigp_enr.tcp6(), content_data.tcp6);
        }
    }

    #[test]
    fn test_publishable() {
        let private_key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
        let scheme_keypair = SchemeKeyPair::from_private_key(private_key);
        let mut publishable_record = Builder::new::<Schemev4>()
            .with_seq(1)
            .with_ip4(EXAMPLE_IP4)
            .with_udp4(EXAMPLE_UDP4)
            .sign_and_build::<Schemev4>(&scheme_keypair)
            .unwrap()
            .to_publishable::<Schemev4>();

        let (seq, _) = publishable_record
            .publish::<Schemev4>(&private_key)
            .unwrap();
        assert_eq!(seq, 1);

        publishable_record.update_ip4(EXAMPLE_IP4);
        let (seq, _) = publishable_record
            .publish::<Schemev4>(&private_key)
            .unwrap();
        assert_eq!(seq, 1);

        let new_ipv4 = Ipv4Addr::new(192, 168, 0, 1);
        let new_udp4 = u16::MAX;
        publishable_record.update_ip4(new_ipv4);
        publishable_record.update_udp4(new_udp4);
        let (seq, address) = publishable_record
            .publish::<Schemev4>(&private_key)
            .unwrap();
        assert_eq!(seq, 2);

        let record = Record::from_textual_form::<Schemev4>(&address).unwrap();
        assert_eq!(record.seq(), 2);
        assert_eq!(record.ip4().unwrap(), new_ipv4);
        assert_eq!(record.udp4().unwrap(), new_udp4);

        let sigp_enr = address.parse::<SigpDefaultEnr>().unwrap();
        assert_eq!(sigp_enr.seq(), 2);
        assert_eq!(sigp_enr.ip4().unwrap(), new_ipv4);
        assert_eq!(sigp_enr.udp4().unwrap(), new_udp4);
    }

    #[test]
    fn test_publishable_seq_overflow() {
        let private_key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
        let scheme_keypair = SchemeKeyPair::from_private_key(private_key);
        let mut publishable_record = Builder::new::<Schemev4>()
            .with_seq(SeqNum::MAX)
            .sign_and_build::<Schemev4>(&scheme_keypair)
            .unwrap()
            .to_publishable::<Schemev4>();

        let (seq, _) = publishable_record
            .publish::<Schemev4>(&private_key)
            .unwrap();
        assert_eq!(seq, SeqNum::MAX);

        publishable_record.update_ip4(EXAMPLE_IP4);
        let err = publishable_record
            .publish::<Schemev4>(&private_key)
            .unwrap_err();
        assert_eq!(err, Error::SeqOverflow);
    }

    #[test]
    #[should_panic]
    fn test_publishable_publish_with_different_private_key() {
        let private_key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
        let scheme_keypair = SchemeKeyPair::from_private_key(private_key);
        let mut publishable_record = Builder::new::<Schemev4>()
            .with_seq(SeqNum::MAX)
            .sign_and_build::<Schemev4>(&scheme_keypair)
            .unwrap()
            .to_publishable::<Schemev4>();
        let private_key2 = Schemev4::new_private_key(&mut OsRng).unwrap();
        publishable_record
            .publish::<Schemev4>(&private_key2)
            .unwrap();
    }

    #[test]
    fn test_errors_libsecp256k1() {
        test_errors_with_scheme::<Schemev4Secp256k1>();
    }

    fn test_errors_with_scheme<S: Scheme>() {
        let test_data = [
            (
                // eth_enr_v4.py: `record_encoded_size_eq_301_base64_size_eq_402`
                "record_encoded_size_eq_301_base64_size_eq_402",
                Error::MaximumRecordRlpEncodedByteLengthExceeded,
                "enr:-QEquEBxABglcZbIGKJ8RHDCp2Ft59tdf61RhV3XXf2BKTlKE2XwzNfihH-46hKkANsXaGRwH8Dp7a3lTrKiv2FMMaFYAYJpZIJ2NIJpcIR_AAABiXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTiDdWRwgnZferijeHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eA",
            ),
            (
                "example_record_urlbase64_with_padding",
                Error::DecodingFailedForInvalidInput,
                "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFzZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8="
            ),
            (
                "example_record_urlbase64_with_trailingbits",
                Error::DecodingFailedForInvalidInput,
                "enr:-IS4QLJYdRwxdy-AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0__YkgFzZwNUuNhm1BDN6c4-UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl9"
            ),
            (
                "example_record_base64",
                Error::DecodingFailedForInvalidInput,
                "enr:+IS4QLJYdRwxdy+AbzWC6wL9ooB6O6uvCvJsJ36rbJztiAs1JzPY0//YkgFzZwNUuNhm1BDN6c4+UVRCJP9bXNCmoDYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2/oxVtw0RW/QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "ip4_overflow",
                Error::RlpDecodingError(rlp::Error::InvalidByteRepresentaion),
                "enr:-FK4QDVBPT0_ui83m7L8tu--KWyFNGke-WR8wQnCzpx5ZPzYMAtwzsObWMP_VZobiy-hdC8XPtB9QJs3QDWpeNXfM-wBgmlkgnY0gmlwhX8AAAAB"
            ),
            (
                "udp_overflow",
                Error::RlpDecodingError(rlp::Error::ItemPayloadByteLengthTooLarge),
                "enr:-FG4QJOYbH3CV8ZzZDYFqZzFR5yqvTnVc4lVOdpcSgFhWFr-UHMp9VN1tXwtLVhWbPZ2U2eeCJp1-GZZoTSyFWKvXUIBgmlkgnY0g3VkcIMBAAA"
            ),
            (
                "empty_address",
                Error::DecodingFailedForInvalidInput,
                ""
            ),
            (
                "address_not_start_with_enr",
                Error::DecodingFailedForInvalidInput,
                "zzz:-IS4QHCYrYZ..."
            ),
            (
                "invalid_address_prefix",
                Error::DecodingFailedForInvalidInput,
                "er:"
            ),
            (
                "invalid_address_1",
                Error::RlpDecodingError(rlp::Error::ItemDataWithInvalidByteLength),
                "enr:xxxx"
            ),
            (
                "invalid_address_2",
                Error::DecodingFailedForInvalidInput,
                "xxxx"
            ),
            (
                "invalid_scheme_name",
                Error::SchemeNameNotRecognized,
                "enr:-Eu4QL0BBo3DsyT80LddQ9hJNJOSa5yn8pZP17icE_sC-EuBDs9l9qZkhe580MMYsGXmIEa6YZlL_rLONKikeDPgzWMBgmlkhHY3Nzc"
            ),
            (
                "empty_scheme_name",
                Error::SchemeNameNotRecognized,
                "enr:-Ee4QC1wawSGJ5OKutVz6PdSobRFaaMYaoyh3bkCBfui4TOUZNUKn4PF4CtSC_fLJH7ATMt5cz8qBNrs7ntSMv1hj-YBgmlkgA"
            ),
            (
                "missing_scheme_name",
                Error::SchemeNameNotRecognized,
                "enr:-EO4QM4Mu2P3SMKAyJFS2qlHkSeOF5OLA2HABpS4Bc7Q1L2xaLdCk8anGikAhJEll-TMRTsBfi3dVOB1F8wE20uOdaYB"
            ),
            (
                "pair_not_sorted",
                Error::KeysNotSortedOrNotUnique,
                "enr:-IS4QHcwuUBGDWqdhAOnB70jfk5070KqrcTmtw4MQvO3qnl6f4fHLgq_OCi0aWlF_jibCN0_vf9yavPJN6SSAffVRf0Bg3VkcIJ2X4JpZIJ2NIJpcIR_AAABiXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTg"
            ),
            (
                "pair_not_sorted_unknown_pair",
                Error::KeysNotSortedOrNotUnique,
                "enr:-Iq4QDHLBahrHQl7KG8ZiEwWHq17vsgiiliTPCAQznvpqPntTAp_MkYI5fFiM2D5MRsBfO5-j7BFDxoXXsH3bIo477YBgnp6gnh4gmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "duplicate_pair_id_not_continuous",
                Error::KeysNotSortedOrNotUnique,
                "enr:-Iq4QGMDE0RCdH6ks2pf-Cwu1PheNvgYwOX3R9_aaj2_mdIAdNEhZt4ALT9DWaVjsH9l8plaq48YT4r53QQdozQaH_kBgmlkgnY0gmlwhH8AAAGCaWSCdjSJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "duplicate_pair_id_continuous",
                Error::KeysNotSortedOrNotUnique,
                "enr:-Iq4QAr8XcrlA0jl2GvZOJn28VMup5uK2FIj-EFZTB6j-yJkHNv776d2uKZ_sBYRefKfp0Z5NGf7w-q_bjJR5kauLKABgmlkgnY0gmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "duplicate_pair_unknown_key_not_continuous",
                Error::KeysNotSortedOrNotUnique,
                "enr:-JK4QJOKiVlxFa5aTQrL9gugAfJg1w6755nzoVEj4i80knjfXFQ8tawsLGPteM4uyRzYsLim2lchlpEER6z_aUkimYIBg2FhX4J4eIJpZIJ2NIJpcIR_AAABiXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTiDYWFfgnh4g3VkcIJ2Xw"
            ),
            (
                "duplicate_pair_unknown_key_continuous",
                Error::KeysNotSortedOrNotUnique,
                "enr:-JK4QEPNZdG9VMduJ9eOvN-yZUzV2wdIhTOlbss5DTKkUiwZLAykgAkIY-AIeS3QGAskzxabxteE0KI4MgEbhGSLlIgBg2FhX4J4eINhYV-CeHiCaWSCdjSCaXCEfwAAAYlzZWNwMjU2azGhA8pjTK4NSay0Adikxrb-jFW3DRFb9AB2nMFADzJYzTE4g3VkcIJ2Xw"
            ),
            (
                "id_not_followed_by_value",
                Error::SchemeNameNotRecognized,
                "enr:-IG4QKpzfa6DpD1H4yuYV0K2e8c78fdpo7UVFFYPlwDO1-aqJy9IDlDDNKs_WHEqgjvrMA7danB4wW9Vaeamc2kcrwkBgmlkgmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "udp_not_followed_by_value",
                Error::PairValueNotFound,
                "enr:-IG4QMC8ZHSTlirEq_JJR63dus-AHlryCRAs8Z800ADYugJGVxdeSlNw3zphjSgR4DZb-TdTpi3h1zVvfcjTwewEVPABgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHA"
            ),
            (
                "public_key_data_not_followed_by_value",
                Error::PublicKeyDataWithInvalidByteLength,
                "enr:-GK4QB5bEqXqhVGLPks_WTmlJPqOYq1AHsq88LW_1sg5pgCiHgEOV8oTZl_ihRbp2bp0bKOV52fYt5AvWCgjLZnLwuYBgmlkgnY0gmlwhH8AAAGJc2VjcDI1Nmsxg3VkcIJ2Xw"
            ),
            (
                "missing_seq",
                Error::KeysNotSortedOrNotUnique,
                "enr:-IO4QCL_Ljm8xvFEjGWvhLKq6GUKFtUYFYmxncUXZFeOFTZSWJdbTBswFigWozCZdsvKfScQ4u7K42kz1do6PQPcWTqCaWSCdjSCaXCEfwAAAYlzZWNwMjU2azGhA8pjTK4NSay0Adikxrb-jFW3DRFb9AB2nMFADzJYzTE4g3VkcIJ2Xw"
            ),
            (
                "empty_content",
                Error::SeqNotFound,
                "enr:-EK4QP4vfS_wIL1xrZBrt_G3kB34a7mRmKLRfSgi8D9fCK8sbQgOtxG0DdhUMusls_K5XYYKz4_fwNpEcXkjyFpL3U0"
            ),
            (
                "invalid_public_key_data_byte_length",
                Error::PublicKeyDataWithInvalidByteLength,
                "enr:-IW4QBXkmNd1n3JaIVFknshz3lYdryy6T_r7Abf-iirgG1ToNEXI7BowdpmXKCl3WUnfs2qBPLvYWMtFruyAp64WE-UBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxogPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOHiDdWRwgnZf"
            ),
            (
                "invalid_public_key_data",
                Error::InvalidPublicKeyData("malformed public key".to_string()),
                "enr:-IS4QEibKLoPsgr90EPYGKoLxeEZy8RSrBD-r3MpOJPYTVyYY4hmnnxmE7BqAfFSA-jXrBX9d0ginjzDrlbwNSL9j98BgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoXh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eIN1ZHCCdl8"
            ),
            (
                "empty_public_key_data",
                Error::PublicKeyDataWithInvalidByteLength,
                "enr:-GO4QKFYYgWUgqs6dZCbFBWFP6ogdeb9p8e2ehNrjWZFhDNjJWYVVxtC0srZksTcMAKfj8c6c3qHRmV_aXE1MDJQvx8BgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxgIN1ZHCCdl8"
            ),
            (
                "missing_public_key_data",
                Error::SignatureVerifyingFailedForMissingPublicKey,
                "enr:-Fi4QMkubIQ_tj51Z57wotKYnjxyZ0DKFKCo1zdRkjCimszLUnI3zxaBR1-Mi8Vx-Y3otiPEPCsWA4Tf3z9LqsetVOwBgmlkgnY0gmlwhH8AAAGDdWRwgnZf"
            ),
            (
                "invalid_signature_data_byte_length",
                Error::SignatureDataWithInvalidByteLength,
                "enr:-E6LeHh4eHh4eHh4eHgBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "empty_signature_data",
                Error::SignatureDataWithInvalidByteLength,
                "enr:-EOAAYJpZIJ2NIJpcIR_AAABiXNlY3AyNTZrMaEDymNMrg1JrLQB2KTGtv6MVbcNEVv0AHacwUAPMljNMTiDdWRwgnZf"
            ),
            (
                "invalid_signature_data",
                Error::InvalidSignature,
                "enr:-IS4QHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHh4eHgBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "invalid_signature",
                Error::InvalidSignature,
                "enr:-IS4QPCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8"
            ),
            (
                "invalid_signature_a",
                Error::InvalidSignature,
                "enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCC__8"
            ),
        ];

        for (test_name, err, address) in test_data {
            assert_eq!(
                Record::from_textual_form::<S>(address).unwrap_err(),
                err,
                "{test_name}"
            );

            // assert!(address.parse::<SigpDefaultEnr>().is_err(), "sigp: {test_name}")
        }
    }

    #[test]
    fn test_new_node_id() {
        // example from the spec
        let key = Schemev4::new_private_key_from_bytes(PRIVATE_KEY_DATA).unwrap();
        let public_key = Schemev4::new_public_key_from_private_key(&key);
        let node_id = Schemev4::new_node_id(&public_key);

        assert_eq!(
            hex::encode(node_id.bytes()),
            "a448f24c6d18e575453db13171562b71999873db5b286df957af199ec94617f7"
        );
    }

    struct ContentData {
        seq: SeqNum,
        ip4: Option<Ipv4Addr>,
        tcp4: Option<u16>,
        udp4: Option<u16>,
        ip6: Option<Ipv6Addr>,
        tcp6: Option<u16>,
        udp6: Option<u16>,
    }

    impl Default for ContentData {
        fn default() -> Self {
            Self {
                seq: SEQUENCE_NUMBER_INITIAL,
                ip4: None,
                tcp4: None,
                udp4: None,
                ip6: None,
                tcp6: None,
                udp6: None,
            }
        }
    }
}
