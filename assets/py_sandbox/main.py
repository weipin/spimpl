if __name__ == '__main__':
    import os
    from pathlib import Path

    for _, _, files in os.walk('./playgrounds/'):
        for filename in files:
            if filename.startswith('_'):
                continue
            if not filename.endswith('.py'):
                continue

            label = Path(filename).stem.upper()
            print(f'### {label} ###{os.linesep}')
            os.system(f'python3 ./playgrounds/{filename}')
            print(f'{os.linesep}')
