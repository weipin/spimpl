import rlp


def encode_uint_0():
    return rlp.encode(0)


def encode_uint_1():
    return rlp.encode(1)


def encode_uint_123():
    return rlp.encode(123)


def encode_uint_127():
    return rlp.encode(127)


def encode_uint_128():
    return rlp.encode(128)


def encode_uint_129():
    return rlp.encode(129)


def encode_uint_255():
    return rlp.encode(255)


def encode_uint_256():
    return rlp.encode(256)


def encode_uint_65536():
    return rlp.encode(65536)


def encode_left_padded_bytes():
    return rlp.encode(bytes([0, 1]))


def encode_bytes_0():
    return rlp.encode(bytes([0]))


def encode_bytes_1_2_3():
    return rlp.encode(bytes([1, 2, 3]))


def encode_vec_of_uint_0_1():
    return rlp.encode([0, 1])


def encode_vec_of_uint_1_2_3():
    return rlp.encode([1, 2, 3])


def encode_vec_of_uint_1_2_3_65536():
    return rlp.encode([1, 2, 3, 65536])


def encode_vec_of_bytes_1_2_3():
    return rlp.encode([bytes([1, 2, 3]), bytes([1, 2, 3]), bytes([1, 2, 3])])


def encode_vec_of_bytes_1_2_3_a():
    return rlp.encode([bytes([1, 2, 3])])


def encode_vec_of_uint_1_bytes_1_2_3_bytes_4_5_6():
    return rlp.encode([[1, bytes([1, 2, 3]), bytes([4, 5, 6])]])


def encode_uint_1_bytes_1_2_3_bytes_4_5_6():
    return rlp.encode([1, bytes([1, 2, 3]), bytes([4, 5, 6])])


def encode_uint_65536_bytes_1_2_3_bytes_4_5_6():
    return rlp.encode([65536, bytes([1, 2, 3]), bytes([4, 5, 6])])


def encode_uint_1_bytes_1_2_3_bytes_4_5_6_uint_0():
    return rlp.encode([1, bytes([1, 2, 3]), bytes([4, 5, 6]), 0])


def encode_uint_1_bytes_1_2_3():
    return rlp.encode([1, bytes([1, 2, 3])])


# first byte edge cases
# [0x00, 0x7f]
def first_byte_eq_0():
    encoded = rlp.encode(bytes([0]))
    assert encoded[0] == 0x00
    return encoded


def first_byte_lt_0x7f():
    encoded = rlp.encode(bytes([0x66]))
    assert encoded[0] < 0x7f
    return encoded


def first_byte_eq_0x7f():
    encoded = rlp.encode(bytes([0x7f]))
    assert encoded[0] == 0x7f
    return encoded


# [0x80, 0xb7]
def first_byte_eq_0x80():
    encoded = rlp.encode(bytes([]))
    assert encoded[0] == 0x80
    return encoded


def first_byte_lt_0xb7_a():
    encoded = rlp.encode(bytes([0x80]))
    assert encoded[0] < 0xb7
    return encoded


def first_byte_lt_0xb7_b():
    encoded = rlp.encode(bytes([1, 2, 3, 4, 5]))
    assert encoded[0] < 0xb7
    return encoded


def first_byte_eq_0xb7():
    encoded = rlp.encode(bytes(list(range(0, 55))))
    assert encoded[0] == 0xb7
    return encoded


# [0xb8, 0xbf]
def first_byte_eq_0xb8():
    encoded = rlp.encode(bytes(list(range(0, 56))))
    assert encoded[0] == 0xb8
    return encoded


def first_byte_lt_0xbf():
    encoded = rlp.encode(bytes(list(range(0, 60))))
    assert encoded[0] < 0xbf
    return encoded


# [0xc0, 0xf7]
def first_byte_eq_0xc0():
    encoded = rlp.encode([])
    assert encoded[0] == 0xc0
    return encoded


def first_byte_lt_0xf7():
    encoded = rlp.encode([1, 2, 3])
    assert encoded[0] < 0xf7
    return encoded


def first_byte_eq_0xf7():
    encoded = rlp.encode(list(range(0, 55)))
    assert encoded[0] == 0xf7
    return encoded


# [0xf8, 0xff]
def first_byte_eq_0xf8():
    encoded = rlp.encode(list(range(0, 56)))
    assert encoded[0] == 0xf8
    return encoded


def first_byte_lt_ff():
    encoded = rlp.encode(list(range(0, 60)))
    assert encoded[0] < 0xff
    return encoded


if __name__ == '__main__':
    import run_all_callable

    run_all_callable.do(locals(), __name__)
