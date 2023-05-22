from discv5_packet import _pack_handshake_message, _build_handshake_message_header, _pack_header, \
    _pack_message

EMPTY = bytes([])
NONCE = bytes.fromhex('ffffffffffffffffffffffff')
SRC_NODE_ID = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
DEST_NODE_ID = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
MASKING_IV = bytes.fromhex('00000000000000000000000000000000')
INITIATOR_KEY = bytes.fromhex('4f9fac6de7567d1e3b1241dffe90f662')
MESSAGE_TYPE = bytes([1])
# discv5_playground: `ping_3`
MESSAGE_RLP_ENCODED = bytes.fromhex('c6840000000101')
ID_SIGNATURE = bytes.fromhex(
    'a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555')
EPH_PUBKEY = bytes.fromhex('039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')

EXTRA_BYTES = bytes([7])
SRC_NODE_ID2 = bytes.fromhex('6666666666666666666666666666666666666666666666666666666666666666')
DEST_NODE_ID2 = bytes.fromhex('7777777777777777777777777777777777777777777777777777777777777777')
INITIATOR_KEY2 = bytes.fromhex('66666666666666666666666666666666')


# too small
def empty():
    return EMPTY


def three_bytes():
    return bytes([1, 2, 3])


def max_minus_1():
    return bytes([1] * (63 - 1))


# too large

# missing fields
def missing_nonce():
    nonce = EMPTY
    header = _build_handshake_message_header(nonce, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


# invalid header masking
def invalid_header_masking():
    dest_node_id = DEST_NODE_ID2
    return _pack_handshake_message(NONCE, SRC_NODE_ID, dest_node_id,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, EPH_PUBKEY)


# invalid nonce
def invalid_nonce():
    nonce = NONCE + EXTRA_BYTES
    return _pack_handshake_message(nonce, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, EPH_PUBKEY)


# invalid dest_node_id
def invalid_dest_node_id():
    dest_node_id = EXTRA_BYTES + DEST_NODE_ID
    return _pack_handshake_message(NONCE, SRC_NODE_ID, dest_node_id,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, EPH_PUBKEY)


# invalid masking_iv
def invalid_masking_iv():
    packet = _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                     MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                     ID_SIGNATURE, EPH_PUBKEY)
    ary = bytearray(packet)
    ary[0] = 7
    return ary


#
# invalid initiator_key
def invalid_initiator_key():
    initiator_key = INITIATOR_KEY2
    return _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, initiator_key, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, EPH_PUBKEY)


# invalid src_node_id
def invalid_src_node_id():
    src_node_id = EXTRA_BYTES + SRC_NODE_ID
    return _pack_handshake_message(NONCE, src_node_id, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, EPH_PUBKEY)


# invalid sig-s
# SIG_SIZE = bytes([64])
def invalid_sig_size_unexpected_value_a():
    sig_size = bytes([63])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             sig_size=sig_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_sig_size_unexpected_value_b():
    sig_size = bytes([65])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             sig_size=sig_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_sig_size_2_bytes_a():
    sig_size = bytes([0, 64])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             sig_size=sig_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_sig_size_2_bytes_b():
    sig_size = bytes([64, 0])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             sig_size=sig_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


# invalid k-s
# PUBKEY_SIZE = bytes([33])
def invalid_pubkey_size_unexpected_value_a():
    pubkey_size = bytes([32])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             pubkey_size=pubkey_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_pubkey_size_unexpected_value_b():
    pubkey_size = bytes([34])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             pubkey_size=pubkey_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_pubkey_size_2_bytes_a():
    pubkey_size = bytes([0, 33])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             pubkey_size=pubkey_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_pubkey_size_2_bytes_b():
    pubkey_size = bytes([33, 0])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             pubkey_size=pubkey_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


# invalid id-signature
def invalid_id_signature_shorter():
    id_signature = bytes.fromhex('a439e6')
    return _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   id_signature, EPH_PUBKEY)


def invalid_id_signature_longer():
    id_signature = ID_SIGNATURE + EXTRA_BYTES
    return _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   id_signature, EPH_PUBKEY)


# invalid eph-pubkey
def invalid_eph_pubkey_shorter():
    eph_pubkey = bytes.fromhex('039a00')
    return _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, eph_pubkey)


def invalid_eph_pubkey_longer():
    eph_pubkey = EPH_PUBKEY + EXTRA_BYTES
    return _pack_handshake_message(NONCE, SRC_NODE_ID, DEST_NODE_ID,
                                   MASKING_IV, INITIATOR_KEY, MESSAGE_TYPE, MESSAGE_RLP_ENCODED,
                                   ID_SIGNATURE, eph_pubkey)


# invalid flag
def invalid_flag_unexpected_value():
    flag = bytes([77])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             flag=flag)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_flag_2_bytes_a():
    flag = bytes([0, 77])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             flag=flag)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_flag_2_bytes_b():
    flag = bytes([77, 77])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             flag=flag)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


# invalid authdata_size
# size_of_handshake_message_authdata_fixed_part::<Schemev4>() = bytes([0, 131])
def invalid_authdata_size_incorrect_value_a():
    authdata_size = bytes([0, 130])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             authdata_size=authdata_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_authdata_size_incorrect_value_b():
    authdata_size = bytes([0, 132])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             authdata_size=authdata_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_authdata_size_1_byte():
    authdata_size = bytes([131])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             authdata_size=authdata_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_authdata_size_3_byte():
    authdata_size = bytes([0, 131, 131])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             authdata_size=authdata_size)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


# invalid protocol_id
# PROTOCOL_ID = b'discv5'
def invalid_protocol_id_unexpected_value():
    protocol_id = b'discz7'
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             protocol_id=protocol_id)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_protocol_id_shorter():
    protocol_id = b'disc'
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             protocol_id=protocol_id)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_protocol_id_longer_a():
    protocol_id = b'discv555'
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             protocol_id=protocol_id)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_protocol_id_longer_b():
    protocol_id = b'discv5' + bytes([0, 1])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             protocol_id=protocol_id)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


#
# invalid version
# VERSION = bytes([0, 1])
def invalid_version_unexpected_value():
    version = bytes([0, 2])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             version=version)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_version_shorter():
    version = bytes([2])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             version=version)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_version_longer_a():
    version = bytes([0, 1, 0])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             version=version)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_version_longer_b():
    version = bytes([0, 1, 255])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             version=version)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


def invalid_version_longer_c():
    version = bytes([0, 2, 0])
    header = _build_handshake_message_header(NONCE, SRC_NODE_ID, ID_SIGNATURE, EPH_PUBKEY,
                                             version=version)
    packed_header = _pack_header(MASKING_IV, DEST_NODE_ID, header)
    packed_message = _pack_message(INITIATOR_KEY, NONCE, MASKING_IV, header, MESSAGE_TYPE,
                                   MESSAGE_RLP_ENCODED)

    return packed_header + packed_message


if __name__ == '__main__':
    import _run_all_callable as run_all_callable

    run_all_callable.do(locals(), __name__)
