from src.visualization import Visualization
import pandas as pd
import numpy as np

# pop-os 20.04 / i7 9750H CPU.
# An empty string, with randomly generated keys.

# ed25519-donna:
# 64-bit:
# gcc ed25519.c -O3 -c -DDED25519_SSE2=none
# gcc bench.c ./*.o -o donna.out -lssl -lcrypto -DDED25519_SSE2=none
# 32-bit:
# gcc ed25519.c -m32 -O3 -c -DED25519_FORCE_32BIT -DDED25519_SSE2=none
# gcc -m32 bench.c ./*.o -o donna.out -lssl -lcrypto -DDED25519_SSE2=none

# valgrind --tool=massif --stacks=yes target/debug/rust
# valgrind --tool=massif --stacks=yes --depth=100 target/debug/rust

def main():
    # dict for signature generation (precomputation)
    dict = {
        'data': [

        ],
        'columns': [
            ['Precomp.', 'No precomp.'],
            ['Precomp.', 'No precomp.'],
        ],
        'impl': [
            'ed25519-fun',
        ],
        'description': [
            'ed25519-fun',
            'ed25519-donna',
            'ed25519-orlp'  # 32-bit implementations can leverage 64-bit platforms
        ],
    }

    # dict for signature generation (precomputation)
    dict_sig_gen = {
        'data': [
            [[21.737, 104.03]]
        ],
    }

    # dict for key generation (architecture)
    dict = {
        'data': [
            [[47.567, 20.444]],
            [[66.744, 31.567]],
            [[28.897, 15.370]]
        ],
        'columns': [
            ['32-bit', '64-bit'],
            ['32-bit', '64-bit'],
            ['32-bit', '64-bit'],

        ],
        'impl': [
            'C',
            'C',
            'Rust'
        ],
        'description': [
            'ed25519-donna',
            'ed25519-orlp',
            'ed25519-dalek'
        ],
    }

    # dict for signature generation (architecture)
    dict_sig_gen = {
        'data': [
            [[49.153, 20.693]],
            [[68.554, 32.136]],
            [[29.710, 16.202]]
        ],
    }

    # dict for signature verification (architecture)
    dict_sig_ver = {
        'data': [
            [[148.740, 67.071]],
            [[188.096, 87.093]],
            [[82.116, 42.711]]
        ],
    }

    # dict for key generation (overall)
    dict_sig_ver = {
        'data': [
            [[1628.575]],
            [[20.666]],
            [[20.131]],
            [[17.699]],
            [[15.390]],
            [[13.517]],
            [[10.653]]
        ],
        'columns': [
            ['pure25519'],
            ['pynacl'],
            ['donna default'],
            ['ed25519-fun'],
            ['dalek default'],
            ['dalek SIMD'],
            ['donna SSE2'],
            ['']
        ],
        'impl': [
            'Python',
            'Python',
            'C',
            'Rust',
            'Rust',
            'Rust',
            'C'
        ],
        'description': [
            'pure25519',
            'pynacl',
            'ed25519-donna',
            'ed25519-fun'
            'ed25519-dalek default',
            'ed25519-dalek simd_backend',
            'ed25519-donna SSE2'
        ],
    }
    # dict for signature generation (overall)
    dict_sig_ver = {
        'data': [
            [[1616.741]],
            [[20.449]],
            [[21.292]],
            [[22.075]],
            [[16.428]],
            [[14.311]]
        ],
    }
    # dict for signature verification (overall)
    dict_sig_ver = {
        'data': [
            [[6128.528]],
            [[46.867]],
            [[68.544]],
            [[55.892]],
            [[42.231]],
            [[26.478]],
            [[11.014]],
            [[36.698]]
        ],
    }

    data = dict['data']
    columns = dict['columns']
    impl = dict['impl']
    len_data = len(data)

    data_list = []
    for i in range(len_data):
        data_pd = pd.DataFrame(data[i], columns=columns[i]).assign(
            Implementation=impl[i])
        data_list.append(data_pd)

    cdf = pd.concat(data_list)
    Visualization(cdf).plot()


if __name__ == '__main__':
    main()
