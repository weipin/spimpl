from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.ciphers import Cipher
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
from cryptography.hazmat.primitives.ciphers.algorithms import AES
from cryptography.hazmat.primitives.ciphers.modes import CTR

PROTOCOL_ID = b'discv5'
VERSION = bytes([0, 1])
FLAG_ORDINARY_MESSAGE = bytes([0])
FLAG_WHOAREYOU = bytes([1])
FLAG_HANDSHAKE_MESSAGE = bytes([2])

AUTHDATA_SIZE_ORDINARY_MESSAGE = bytes([0, 32])
AUTHDATA_SIZE_WHOAREYOU = bytes([0, 24])
AUTHDATA_SIZE_HANDSHAKE_MESSAGE_FIXED = bytes([0, 131])

SIG_SIZE = bytes([64])
PUBKEY_SIZE = bytes([33])


# Handshake message with ENR packet
def pack_handshake_message_with_record_example():
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
    dest_node_id = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    masking_iv = bytes.fromhex('00000000000000000000000000000000')
    initiator_key = bytes.fromhex('53b1c075f41876423154e157470c2f48')
    id_signature = bytes.fromhex(
        'a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555')
    eph_pubkey = bytes.fromhex('039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    message_type = bytes([1])
    # discv5_playground: `ping_3`
    message_rlp_encoded = bytes.fromhex('c6840000000101')
    record_rlp_encoded = bytes.fromhex(
        'f87db84017e1b073918da32d640642c762c0e2781698e4971f8ab39a77746adad83f01e76ffc874c5924808bbe7c50890882c2b8a01287a0b08312d1d53a17d517f5eb2701826964827634826970847f00000189736563703235366b31a10313d14211e0287b2361a1615890a9b5212080546d0a257ae4cff96cf534992cb9')

    packed = _pack_handshake_message_with_record(nonce, src_node_id, dest_node_id, masking_iv,
                                                 initiator_key, message_type, message_rlp_encoded,
                                                 id_signature, eph_pubkey, record_rlp_encoded)
    assert packed.hex() == (
        '00000000000000000000000000000000088b3d4342774649305f313964a39e55ea96c005ad539c8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d34c4f53245d08da4bb236988'
        '68350aaad22e3ab8dd034f548a1c43cd246be98562fafa0a1fa86d8e7a3b95ae78cc2b988ded6a5b59eb83ad58097252188b902b21481e30e5e285f19735796706adff216ab862a9186875f9494150c'
        '4ae06fa4d1f0396c93f215fa4ef524e0ed04c3c21e39b1868e1ca8105e585ec17315e755e6cfc4dd6cb7fd8e1a1f55e49b4b5eb024221482105346f3c82b15fdaae36a3bb12a494683b4a3c7f2ae413'
        '06252fed84785e2bbff3b022812d0882f06978df84a80d443972213342d04b9048fc3b1d5fcb1df0f822152eced6da4d3f6df27e70e4539717307a0208cd208d65093ccab5aa596a34d751140198766'
        '2d8cf62b139471')
    return packed


def _pack_handshake_message_with_record(nonce, src_node_id, dest_node_id,
                                        masking_iv, initiator_key, message_type,
                                        message_rlp_encoded,
                                        id_signature, eph_pubkey, record_rlp_encoded, sig_size=None,
                                        pubkey_size=None,
                                        flag=None, authdata_size=None, protocol_id=None,
                                        version=None):
    header = _build_handshake_message_header_with_record(nonce, src_node_id, id_signature,
                                                         eph_pubkey, record_rlp_encoded,
                                                         sig_size=sig_size, pubkey_size=pubkey_size,
                                                         flag=flag, authdata_size=authdata_size,
                                                         protocol_id=protocol_id,
                                                         version=version)
    packed_header = _pack_header(masking_iv, dest_node_id, header)
    packed_message = _pack_message(initiator_key, nonce, masking_iv, header, message_type,
                                   message_rlp_encoded)

    return packed_header + packed_message


def build_handshake_message_header_with_record_example():
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
    id_signature = bytes.fromhex(
        'a439e69918e3f53f555d8ca4838fbe8abeab56aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555')
    eph_pubkey = bytes.fromhex('039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    # eth_enr_v4: `discv5_example_record_without_extra_entropy`
    record_rlp_encoded = bytes.fromhex(
        'f87db84017e1b073918da32d640642c762c0e2781698e4971f8ab39a77746adad83f01e76ffc874c5924808bbe7c50890882c2b8a01287a0b08312d1d53a17d517f5eb2701826964827634826970847f00000189736563703235366b31a10313d14211e0287b2361a1615890a9b5212080546d0a257ae4cff96cf534992cb9')

    packed = _build_handshake_message_header_with_record(nonce, src_node_id, id_signature,
                                                         eph_pubkey, record_rlp_encoded)
    assert packed.hex() == (
        '646973637635000102ffffffffffffffffffffffff0102aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb4021a439e69918e3f53f555d8ca4838fbe8abeab5'
        '6aa55b056a2ac4d49c157ee719240a93f56c9fccfe7742722a92b3f2dfa27a5452f5aca8adeeab8c4d5d87df555039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5f8'
        '7db84017e1b073918da32d640642c762c0e2781698e4971f8ab39a77746adad83f01e76ffc874c5924808bbe7c50890882c2b8a01287a0b08312d1d53a17d517f5eb2701826964827634826970847f0'
        '0000189736563703235366b31a10313d14211e0287b2361a1615890a9b5212080546d0a257ae4cff96cf534992cb9')
    return packed


def _build_handshake_message_header_with_record(nonce, src_node_id, id_signature,
                                                eph_pubkey, record_rlp_encoded, sig_size=None,
                                                pubkey_size=None,
                                                flag=None, authdata_size=None, protocol_id=None,
                                                version=None):
    if sig_size is None:
        sig_size = SIG_SIZE
    if pubkey_size is None:
        pubkey_size = PUBKEY_SIZE
    if flag is None:
        flag = FLAG_HANDSHAKE_MESSAGE
    if authdata_size is None:
        n = int.from_bytes(AUTHDATA_SIZE_HANDSHAKE_MESSAGE_FIXED, "big") + len(record_rlp_encoded)
        authdata_size = n.to_bytes(2, 'big')
    if protocol_id is None:
        protocol_id = PROTOCOL_ID
    if version is None:
        version = VERSION

    return protocol_id + version + flag + nonce + authdata_size + src_node_id + sig_size + pubkey_size + id_signature + eph_pubkey + record_rlp_encoded


# Handshake message packet
def pack_handshake_message_example():
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
    dest_node_id = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    masking_iv = bytes.fromhex('00000000000000000000000000000000')
    initiator_key = bytes.fromhex('4f9fac6de7567d1e3b1241dffe90f662')
    id_signature = bytes.fromhex(
        'c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db')
    eph_pubkey = bytes.fromhex('039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    message_type = bytes([1])
    # discv5_playground: `ping_3`
    message_rlp_encoded = bytes.fromhex('c6840000000101')

    packed = _pack_handshake_message(nonce, src_node_id, dest_node_id, masking_iv,
                                     initiator_key, message_type, message_rlp_encoded, id_signature,
                                     eph_pubkey)
    assert packed.hex() == '00000000000000000000000000000000088b3d4342774649305f313964a39e55ea96c005ad521d8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d34c4f53245d08da4bb252012b2cba3f4f374a90a75cff91f142fa9be3e0a5f3ef268ccb9065aeecfd67a999e7fdc137e062b2ec4a0eb92947f0d9a74bfbf44dfba776b21301f8b65efd5796706adff216ab862a9186875f9494150c4ae06fa4d1f0396c93f215fa4ef524f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d839cf8'
    return packed


def _pack_handshake_message(nonce, src_node_id, dest_node_id,
                            masking_iv, initiator_key, message_type, message_rlp_encoded,
                            id_signature, eph_pubkey, sig_size=None, pubkey_size=None,
                            flag=None, authdata_size=None, protocol_id=None, version=None):
    header = _build_handshake_message_header(nonce, src_node_id, id_signature,
                                             eph_pubkey, sig_size=sig_size, pubkey_size=pubkey_size,
                                             flag=flag, authdata_size=authdata_size,
                                             protocol_id=protocol_id,
                                             version=version)
    packed_header = _pack_header(masking_iv, dest_node_id, header)
    packed_message = _pack_message(initiator_key, nonce, masking_iv, header, message_type,
                                   message_rlp_encoded)

    return packed_header + packed_message


def build_handshake_message_header_example():
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
    id_signature = bytes.fromhex(
        'c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db')
    eph_pubkey = bytes.fromhex('039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')

    packed = _build_handshake_message_header(nonce, src_node_id, id_signature, eph_pubkey)
    assert packed.hex() == '646973637635000102ffffffffffffffffffffffff0083aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb4021c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5'
    return packed


def _build_handshake_message_header(nonce, src_node_id, id_signature,
                                    eph_pubkey, sig_size=None, pubkey_size=None,
                                    flag=None, authdata_size=None, protocol_id=None,
                                    version=None):
    if sig_size is None:
        sig_size = SIG_SIZE
    if pubkey_size is None:
        pubkey_size = PUBKEY_SIZE
    if flag is None:
        flag = FLAG_HANDSHAKE_MESSAGE
    if authdata_size is None:
        authdata_size = AUTHDATA_SIZE_HANDSHAKE_MESSAGE_FIXED
    if protocol_id is None:
        protocol_id = PROTOCOL_ID
    if version is None:
        version = VERSION

    return protocol_id + version + flag + nonce + authdata_size + src_node_id + sig_size + pubkey_size + id_signature + eph_pubkey


# Whoareyou packet
def pack_whoareyou_example():
    nonce = bytes.fromhex('0102030405060708090a0b0c')
    id_nonce = bytes.fromhex('0102030405060708090a0b0c0d0e0f10')
    enr_seq = bytes.fromhex('0000000000000000')
    dest_node_id = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    masking_iv = bytes.fromhex('00000000000000000000000000000000')

    packed = _pack_whoareyou(nonce, dest_node_id, masking_iv, id_nonce, enr_seq)
    assert packed.hex() == '00000000000000000000000000000000088b3d434277464933a1ccc59f5967ad1d6035f15e528627dde75cd68292f9e6c27d6b66c8100a873fcbaed4e16b8d'
    return packed


def _pack_whoareyou(nonce, dest_node_id, masking_iv, id_nonce, enr_seq,
                    flag=None, authdata_size=None, protocol_id=None, version=None):
    header = _build_whoareyou_header(nonce, id_nonce, enr_seq, flag=flag,
                                     authdata_size=authdata_size, protocol_id=protocol_id,
                                     version=version)
    packed_header = _pack_header(masking_iv, dest_node_id, header)

    return packed_header


def build_whoareyou_header_example():
    nonce = bytes.fromhex('0102030405060708090a0b0c')
    id_nonce = bytes.fromhex('0102030405060708090a0b0c0d0e0f10')
    enr_seq = bytes.fromhex('0000000000000000')

    header = _build_whoareyou_header(nonce, id_nonce, enr_seq)
    assert header.hex() == '6469736376350001010102030405060708090a0b0c00180102030405060708090a0b0c0d0e0f100000000000000000'
    return header


def _build_whoareyou_header(nonce, id_nonce, enr_seq, flag=None, authdata_size=None,
                            protocol_id=None, version=None):
    if flag is None:
        flag = FLAG_WHOAREYOU
    if authdata_size is None:
        authdata_size = AUTHDATA_SIZE_WHOAREYOU
    if protocol_id is None:
        protocol_id = PROTOCOL_ID
    if version is None:
        version = VERSION

    return protocol_id + version + flag + nonce + authdata_size + id_nonce + enr_seq


# Ordinary message packet
def pack_ordinary_message_example():
    # Ping message packet
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')
    dest_node_id = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    initiator_key = bytes.fromhex('00000000000000000000000000000000')
    masking_iv = bytes.fromhex('00000000000000000000000000000000')
    message_type = bytes([1])
    # discv5_playground: `ping_2`
    message_rlp_encoded = bytes.fromhex('c6840000000102')

    packed = _pack_ordinary_message(nonce, src_node_id, dest_node_id, masking_iv, initiator_key,
                                    message_type, message_rlp_encoded)
    assert packed.hex() == ('00000000000000000000000000000000088b3d4342774649325f313964a39e55'
                            'ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d3'
                            '4c4f53245d08dab84102ed931f66d1492acb308fa1c6715b9d139b81acbdcc')
    return packed


def _pack_ordinary_message(nonce, src_node_id, dest_node_id, masking_iv,
                           initiator_key, message_type, message_rlp_encoded,
                           flag=None, authdata_size=None, protocol_id=None, version=None):
    header = _build_ordinary_message_header(nonce, src_node_id, flag=flag,
                                            authdata_size=authdata_size, protocol_id=protocol_id,
                                            version=version)
    packed_header = _pack_header(masking_iv, dest_node_id, header)
    packed_message = _pack_message(initiator_key, nonce, masking_iv, header, message_type,
                                   message_rlp_encoded)

    return packed_header + packed_message


def build_ordinary_message_header_example():
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    src_node_id = bytes.fromhex('aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')

    header = _build_ordinary_message_header(nonce, src_node_id)
    assert header.hex() == '646973637635000100ffffffffffffffffffffffff0020aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb'
    return header


def _build_ordinary_message_header(nonce, src_node_id, flag=None, authdata_size=None,
                                   protocol_id=None, version=None):
    if flag is None:
        flag = FLAG_ORDINARY_MESSAGE
    if authdata_size is None:
        authdata_size = AUTHDATA_SIZE_ORDINARY_MESSAGE
    if protocol_id is None:
        protocol_id = PROTOCOL_ID
    if version is None:
        version = VERSION

    return protocol_id + version + flag + nonce + authdata_size + src_node_id


# Common
def pack_header_example():
    masking_iv = bytes.fromhex('00000000000000000000000000000000')
    dest_id = bytes.fromhex('bbbb9d047f0488c0b5a93c1c3f2d8bafc7c8ff337024a55434a0d0555de64db9')
    header = bytes.fromhex(
        '646973637635000100ffffffffffffffffffffffff0020aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb')

    packed = _pack_header(masking_iv, dest_id, header)
    assert packed.hex() == '00000000000000000000000000000000088b3d4342774649325f313964a39e55ea96c005ad52be8c7560413a7008f16c9e6d2f43bbea8814a546b7409ce783d34c4f53245d08da'
    return packed


def _pack_header(masking_iv, dest_id, header):
    encrypted_header = _aesctr(dest_id[:16], masking_iv, header)
    return masking_iv + encrypted_header


def pack_message_example():
    initiator_key = bytes.fromhex('4f9fac6de7567d1e3b1241dffe90f662')
    nonce = bytes.fromhex('ffffffffffffffffffffffff')
    masking_iv = bytes.fromhex('00000000000000000000000000000000')
    header = bytes.fromhex(
        '646973637635000102ffffffffffffffffffffffff0083aaaa8419e9f49d0083561b48287df592939a8d19947d8c0ef88f2a4856a69fbb4021c0a04b36f276172afc66a62848eb0769800c670c4edbefab8f26785e7fda6b56506a3f27ca72a75b106edd392a2cbf8a69272f5c1785c36d1de9d98a0894b2db039a003ba6517b473fa0cd74aefe99dadfdb34627f90fec6362df85803908f53a5')
    message_type = bytes([1])
    message_rlp_encoded = bytes.fromhex('c6840000000101')

    packed = _pack_message(initiator_key, nonce, masking_iv, header, message_type,
                           message_rlp_encoded)
    assert packed.hex() == 'f1eadf5f0f4126b79336671cbcf7a885b1f8bd2a5d839cf8'
    return packed


def _pack_message(initiator_key, nonce, masking_iv, header, message_type, message_rlp_encoded):
    message_pt = message_type + message_rlp_encoded
    message_ad = masking_iv + header
    message_ct = _aesgcm(initiator_key, nonce, message_ad, message_pt)
    return message_ct


def aesctr_example():
    output = _aesctr(bytes([1] * 16), bytes([2] * 16), bytes([7] * 32))
    assert output.hex() == '10d113f47eae329770ee5270fa36c50d248a8a65ff9599f2d3de012cf7fc705a'
    return output


def _aesctr(key, iv, pt):
    cipher = Cipher(AES(key), CTR(iv), backend=default_backend())
    encryptor = cipher.encryptor()
    return encryptor.update(pt) + encryptor.finalize()


def aesgcm_example():
    key = bytes.fromhex('9f2d77db7004bf8a1a85107ac686990b')
    nonce = bytes.fromhex('27b5af763c446acd2749fe8e')
    ad = bytes.fromhex('93a7400fa0d6a694ebc24d5cf570f65d04215b6ac00757875e3f3a5f42107903')
    pt = bytes.fromhex('01c20101')

    output = _aesgcm(key, nonce, ad, pt)
    assert output.hex() == 'a5d12a2d94b8ccb3ba55558229867dc13bfa3648'
    return output


def _aesgcm(key, nonce, ad, pt):
    aesgcm = AESGCM(key)
    cipher_text = aesgcm.encrypt(nonce, pt, ad)
    return cipher_text


if __name__ == '__main__':
    import _run_all_callable as run_all_callable

    run_all_callable.do(locals(), __name__)
