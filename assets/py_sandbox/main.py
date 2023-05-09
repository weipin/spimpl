if __name__ == '__main__':
    import os

    playgrounds = [('RLP', 'eth_rlp.py'), ('ENR', 'eth_enr_v4.py'),
                   ('DISCV5_ID_SIGNATURE', 'discv5_id_signature.py')]
    for name, filename in playgrounds:
        print(f'### {name} ###{os.linesep}')
        os.system(f'python3 ./playgrounds/{filename}')
        print(f'{os.linesep}')
