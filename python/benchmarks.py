from src.benches.pure25519_bench import Pure25519Bench
from src.benches.pynacl_bench import PyNaClBench


def main():
    Pure25519Bench().bench()
    PyNaClBench().bench()


if __name__ == '__main__':
    main()
