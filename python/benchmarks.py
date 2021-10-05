from src.benches.pyca_bench import PycaBench
from src.benches.pynacl_bench import PyNaClBench


def main():
    PycaBench.().bench()
    PyNaClBench().bench()


if __name__ == '__main__':
    main()
