def do(local_objs, module_name):
    for key, value in local_objs.items():
        if callable(value) and not key.startswith('_') and value.__module__ == module_name:
            print(f'> [{key}]: {smart_repr(value())}')


def smart_repr(obj):
    if type(obj) == str:
        return obj
    if type(obj) is bytes or bytearray:
        return f'0x{obj.hex()}'
    return obj
