from hashlib import sha256

from ecdsa import SigningKey, SECP256k1
from ecdsa.util import sigencode_string_canonize

EXTRA_ENTROPY = bytes.fromhex('baaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaad')
KEY_DATA = 0xb71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291
KEY = SigningKey.from_secret_exponent(KEY_DATA, curve=SECP256k1)


def id_nonce_signing_example():
    key_n = 0xfb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736;
    key = SigningKey.from_secret_exponent(key_n, curve=SECP256k1)
    challenge_data = bytes.fromhex(
        '000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000')
    ephemeral_pubkey = bytes.fromhex(
        '039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231')
    node_id_b = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')

    signature = _build_id_signature(challenge_data, ephemeral_pubkey, node_id_b, True, key)

    return signature


def id_nonce_signing_example_without_extra_entropy():
    key_n = 0xfb757dc581730490a1d7a00deea65e9b1936924caaea8f44d476014856b68736;
    key = SigningKey.from_secret_exponent(key_n, curve=SECP256k1)
    challenge_data = bytes.fromhex(
        '000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000')
    ephemeral_pubkey = bytes.fromhex(
        '039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231')
    node_id_b = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')

    signature = _build_id_signature(challenge_data, ephemeral_pubkey, node_id_b, False, key)
    assert signature == bytes.fromhex(
        '94852a1e2318c4e5e9d422c98eaf19d1d90d876b29cd06ca7cb7546d0fff7b484fe86c09a064fe72bdbef73ba8e9c34df0cd2b53e9d65528c2c7f336d5dfc6e6')

    return signature


def ping_handshake_example_id_nonce_signing_without_extra_entropy():
    node_a_key_n = 0xeef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f;
    key = SigningKey.from_secret_exponent(node_a_key_n, curve=SECP256k1)
    challenge_data = bytes.fromhex(
        '000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000001')
    ephemeral_pubkey = bytes.fromhex(
        '039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    node_id_b = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')

    signature = _build_id_signature(challenge_data, ephemeral_pubkey, node_id_b, False, key)
    assert signature == bytes.fromhex(
        'c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db')

    return signature


def ping_handshake_with_record_example_id_nonce_signing_without_extra_entropy():
    node_a_key_n = 0xeef77acb6c6a6eebc5b363a475ac583ec7eccdb42b6481424c60f59aa326547f;
    key = SigningKey.from_secret_exponent(node_a_key_n, curve=SECP256k1)
    challenge_data = bytes.fromhex(
        '000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000')
    ephemeral_pubkey = bytes.fromhex(
        '039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    node_id_b = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')

    signature = _build_id_signature(challenge_data, ephemeral_pubkey, node_id_b, False, key)
    assert signature == bytes.fromhex(
        'a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555')

    return signature


def id_nonce_signing_example_hash():
    challenge_data = bytes.fromhex(
        '000000000000000000000000000000006469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000')
    ephemeral_pubkey = bytes.fromhex(
        '039961e4c2356d61bedb83052c115d311acb3a96f5777296dcf297351130266231')
    node_id_b = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    id_signature_input = b"discovery v5 identity proof" + challenge_data + ephemeral_pubkey + node_id_b
    return sha256(id_signature_input).digest()


def _build_id_signature(challenge_data, ephemeral_pubkey, node_id_b, extra_entropy=True, key=None):
    if key is None:
        key = KEY

    id_signature_input = b"discovery v5 identity proof" + challenge_data + ephemeral_pubkey + node_id_b
    h = sha256(id_signature_input).digest()
    if extra_entropy:
        signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                  sigencode=sigencode_string_canonize,
                                                  extra_entropy=EXTRA_ENTROPY)
    else:
        signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                  sigencode=sigencode_string_canonize)

    return signature


if __name__ == '__main__':
    import _run_all_callable as run_all_callable

    run_all_callable.do(locals(), __name__)
