from base64 import b64encode, urlsafe_b64encode
from hashlib import sha256

from ecdsa import SigningKey, SECP256k1
from ecdsa.util import sigencode_string_canonize
from eth_hash.auto import keccak
from rlp import encode

KEY_DATA = 0xb71c71a67e1177ad4e901695e1b4b9ee17ae16c6668d313eac2f96dbcda3f291
KEY = SigningKey.from_secret_exponent(KEY_DATA, curve=SECP256k1)
IP4 = 0x7f000001  # 127.0.0.1
IP4_1 = 0xc0a80001  # 192.168.0.1
UDP4 = 30303
UDP4_1 = 65535
TCP = 30302
IP6 = bytes.fromhex('00000000000000000000ffffc00a02ff')  # 0:0:0:0:0:ffff:c00a:2ff
UDP6 = 65535
TCP6 = 65534
PUBLIC_KEY_DATA = bytes.fromhex(
    '03ca634cae0d49acb401d8a4c6b6fe8c55b70d115bf400769cc1400f3258cd3138')
EXTRA_ENTROPY = bytes.fromhex('baaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaadbaaaaaad')

# Ok cases
def example_record_without_extra_entropy():
    address = _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4],
                            False)
    assert address == 'enr:-IS4QHCYrYZbAKWCBRlAy5zzaDZXJBGkcnh4MHcBFZntXNFrdvJjX04jRzjzCBOonrkTfj499SZuOh8R33Ls8RRcy5wBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCdl8'
    return address


def example_record():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def example_record_published_with_updated_ip4_udp4():
    return _node_address([2, 'id', 'v4', 'ip', IP4_1, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4_1])


def minimal_record():
    return _node_address([1, 'id', 'v4', 'secp256k1', PUBLIC_KEY_DATA])


def example_record_mixed_with_unknown_pairs_address():
    return _node_address(
        [1, 'a_', 'xxx', 'id', 'v4', 'ie_', 'xxx', 'ip', IP4, 'iz_', 'xxx', 'secp256k1',
         PUBLIC_KEY_DATA, 'sz', 'xxx', 'udp', UDP4, 'uz', 'xxx'])


def record_encoded_size_eq_300_base64_size_eq_400():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4,
                          'z', 'x' * 162])


def full_record():
    return _node_address(
        [1, 'id', 'v4', 'ip', IP4, 'ip6', IP6, 'secp256k1', PUBLIC_KEY_DATA, 'tcp', TCP, 'tcp6',
         TCP6, 'udp', UDP4,
         'udp6', UDP6])


def construct_with_signature_and_content_items():
    signature_hex = '7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c'
    content_signature = bytes.fromhex(signature_hex)
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


# error cases
def record_encoded_size_eq_301_base64_size_eq_402():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4,
                          'z', 'x' * 163])


def example_record_urlbase64_with_padding():
    return _node_address_with_padding_enabled(
        [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def example_record_without_extra_entropy_urlbase64_with_padding():
    return _node_address_with_padding_enabled(
        [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4], extra_entropy=False)


def example_record_base64():
    return _node_address_with_base64(
        [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def ip4_overflow():
    return _node_address([1, 'id', 'v4', 'ip', 0x7f00000001])


def udp_overflow():
    return _node_address([1, 'id', 'v4', 'udp', 65536])


def empty_address():
    return ''


def address_not_start_with_enr():
    return 'zzz:-IS4QHCYrYZ...'


def invalid_address_prefix():
    return 'er:'


def invalid_address_1():
    return 'enr:xxxx'


def invalid_address_2():
    return 'xxxx'


def invalid_scheme_name():
    return _node_address([1, 'id', 'v777'])


def empty_scheme_name():
    return _node_address([1, 'id', ''])


def missing_scheme_name():
    return _node_address([1])


def pair_not_sorted():
    return _node_address([1, 'udp', UDP4, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA])


def pair_not_sorted_unknown_pair():
    return _node_address(
        [1, 'zz', 'xx', 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def duplicate_pair_id_not_continuous():
    return _node_address(
        [1, 'id', 'v4', 'ip', IP4, 'id', 'v4', 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def duplicate_pair_id_continuous():
    return _node_address(
        [1, 'id', 'v4', 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def duplicate_pair_unknown_key_not_continuous():
    return _node_address(
        [1, 'aa_', 'xx', 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'aa_', 'xx', 'udp',
         UDP4])


def duplicate_pair_unknown_key_continuous():
    return _node_address(
        [1, 'aa_', 'xx', 'aa_', 'xx', 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp',
         UDP4])


def id_not_followed_by_value():
    return _node_address([1, 'id', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def udp_not_followed_by_value():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp'])


def public_key_data_not_followed_by_value():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', 'udp', UDP4])


def missing_seq():
    return _node_address(['id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4])


def empty_content():
    return _node_address([])


def invalid_public_key_data_byte_length():
    return _node_address(
        [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA + b'x', 'udp', UDP4])


def invalid_public_key_data():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', b'x' * 33, 'udp', UDP4])


def empty_public_key_data():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'secp256k1', b'', 'udp', UDP4])


def missing_public_key_data():
    return _node_address([1, 'id', 'v4', 'ip', IP4, 'udp', UDP4])


def invalid_signature_data_byte_length():
    content_signature = b'x' * 11
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def empty_signature_data():
    content_signature = b''
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def invalid_signature_data():
    content_signature = b'x' * 64
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def invalid_signature():
    # See `construct_with_signature_and_content_items` for the correct version
    signature_hex = '7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c'
    signature_hex = 'f' + signature_hex[1:]
    content_signature = bytes.fromhex(signature_hex)
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def invalid_signature_a():
    signature_hex = '7098ad865b00a582051940cb9cf36836572411a47278783077011599ed5cd16b76f2635f4e234738f30813a89eb9137e3e3df5266e3a1f11df72ecf1145ccb9c'
    content_signature = bytes.fromhex(signature_hex)
    # UDP4 -> UDP4_1
    content_items = [1, 'id', 'v4', 'ip', IP4, 'secp256k1', PUBLIC_KEY_DATA, 'udp', UDP4_1]

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def _node_address(content_items, extra_entropy=True, key=None):
    if key is None:
        key = KEY

    content_rlp_encoded = encode(content_items)
    h = keccak(content_rlp_encoded)
    if extra_entropy:
        content_signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                          sigencode=sigencode_string_canonize,
                                                          extra_entropy=EXTRA_ENTROPY)
    else:
        content_signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                          sigencode=sigencode_string_canonize)

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


def _node_address_with_padding_enabled(content_items, extra_entropy=True, key=None):
    if key is None:
        key = KEY

    content_rlp_encoded = encode(content_items)
    h = keccak(content_rlp_encoded)
    if extra_entropy:
        content_signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                          sigencode=sigencode_string_canonize,
                                                          extra_entropy=EXTRA_ENTROPY)
    else:
        content_signature = key.sign_digest_deterministic(h, hashfunc=sha256,
                                                          sigencode=sigencode_string_canonize)

    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + urlsafe_b64encode(record_rlp_encoded).decode('utf-8')


def _node_address_with_base64(content_items):
    content_rlp_encoded = encode(content_items)
    h = keccak(content_rlp_encoded)
    content_signature = KEY.sign_digest_deterministic(h, hashfunc=sha256,
                                                      sigencode=sigencode_string_canonize,
                                                      extra_entropy=EXTRA_ENTROPY)
    record_rlp_items = [content_signature] + content_items
    record_rlp_encoded = encode(record_rlp_items)

    return "enr:" + b64encode(record_rlp_encoded).decode('utf-8').rstrip('=')


if __name__ == '__main__':
    import run_all_callable

    run_all_callable.do(locals(), __name__)
