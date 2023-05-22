from discv5_packet import _pack_whoareyou

EMPTY = bytes([])
NONCE = bytes.fromhex('0102030405060708090a0b0c')
ID_NONCE = bytes.fromhex('0102030405060708090a0b0c0d0e0f10')
ENR_SEQ = bytes.fromhex('0000000000000000')
DEST_NODE_ID = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
MASKING_IV = bytes.fromhex('00000000000000000000000000000000')


def enr_seq_max():
    enr_seq = bytes.fromhex('ffffffffffffffff')
    return _pack_whoareyou(NONCE, DEST_NODE_ID, MASKING_IV, ID_NONCE, enr_seq)


if __name__ == '__main__':
    import _run_all_callable as run_all_callable

    run_all_callable.do(locals(), __name__)
