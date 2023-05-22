from discv5_packet import _pack_whoareyou, _build_whoareyou_header, _pack_header

EMPTY = bytes([])
NONCE = bytes.fromhex('0102030405060708090a0b0c')
ID_NONCE = bytes.fromhex('0102030405060708090a0b0c0d0e0f10')
ENR_SEQ = bytes.fromhex('0000000000000000')
DEST_NODE_ID = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
MASKING_IV = bytes.fromhex('00000000000000000000000000000000')

EXTRA_BYTES = bytes([7])
DEST_NODE_ID2 = bytes.fromhex('7777777777777777777777777777777777777777777777777777777777777777')


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
    return _pack_whoareyou(nonce, DEST_NODE_ID, MASKING_IV, ID_NONCE, ENR_SEQ)


# invalid header masking
def invalid_header_masking():
    dest_node_id = DEST_NODE_ID2
    return _pack_whoareyou(NONCE, dest_node_id, MASKING_IV, ID_NONCE, ENR_SEQ)


# invalid nonce
def invalid_nonce():
    nonce = NONCE + EXTRA_BYTES
    return _pack_whoareyou(nonce, DEST_NODE_ID, MASKING_IV, ID_NONCE, ENR_SEQ)


# invalid dest_node_id
def invalid_dest_node_id():
    dest_node_id = EXTRA_BYTES + DEST_NODE_ID
    return _pack_whoareyou(NONCE, dest_node_id, MASKING_IV, ID_NONCE, ENR_SEQ)


# invalid masking_iv
def invalid_masking_iv():
    packet = _pack_whoareyou(NONCE, DEST_NODE_ID, MASKING_IV, ID_NONCE, ENR_SEQ)
    ary = bytearray(packet)
    ary[0] = 7
    return ary


# invalid id_nonce
def invalid_id_nonce():
    id_nonce = ID_NONCE + EXTRA_BYTES
    return _pack_whoareyou(NONCE, DEST_NODE_ID, MASKING_IV, id_nonce, ENR_SEQ)


# invalid enr_seq
def invalid_enr_seq_2_byte():
    enr_seq = bytes.fromhex('0001')
    return _pack_whoareyou(NONCE, DEST_NODE_ID, MASKING_IV, ID_NONCE, enr_seq)


def invalid_enr_seq_9_byte():
    enr_seq = bytes.fromhex('000000000000000001')
    return _pack_whoareyou(NONCE, DEST_NODE_ID, MASKING_IV, ID_NONCE, enr_seq)


# invalid flag
def invalid_flag_unexpected_value():
    flag = bytes([77])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, flag=flag)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_flag_2_bytes_a():
    flag = bytes([1, 77])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, flag=flag)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_flag_2_bytes_b():
    flag = bytes([77, 77])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, flag=flag)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


# invalid authdata_size
# AUTHDATA_SIZE_WHOAREYOU = bytes([0, 24])
def invalid_authdata_size_incorrect_value_a():
    authdata_size = bytes([0, 23])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, authdata_size=authdata_size)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_authdata_size_incorrect_value_b():
    authdata_size = bytes([0, 25])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, authdata_size=authdata_size)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_authdata_size_1_byte():
    authdata_size = bytes([24])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, authdata_size=authdata_size)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_authdata_size_3_byte():
    authdata_size = bytes([0, 24, 24])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, authdata_size=authdata_size)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


# invalid protocol_id
# PROTOCOL_ID = b'discv5'
def invalid_protocol_id_unexpected_value():
    protocol_id = b'discz7'
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, protocol_id=protocol_id)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_protocol_id_shorter():
    protocol_id = b'disc'
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, protocol_id=protocol_id)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_protocol_id_longer_a():
    protocol_id = b'discv555'
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, protocol_id=protocol_id)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_protocol_id_longer_b():
    protocol_id = b'discv5' + bytes([0, 1])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, protocol_id=protocol_id)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


# invalid version
# VERSION = bytes([0, 1])
def invalid_version_unexpected_value():
    version = bytes([0, 2])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, version=version)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_version_shorter():
    version = bytes([2])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, version=version)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_version_longer_a():
    version = bytes([0, 1, 0])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, version=version)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_version_longer_b():
    version = bytes([0, 1, 255])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, version=version)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


def invalid_version_longer_c():
    version = bytes([0, 2, 0])
    header = _build_whoareyou_header(NONCE, ID_NONCE, ENR_SEQ, version=version)
    return _pack_header(MASKING_IV, DEST_NODE_ID, header)


if __name__ == '__main__':
    import _run_all_callable as run_all_callable

    run_all_callable.do(locals(), __name__)
