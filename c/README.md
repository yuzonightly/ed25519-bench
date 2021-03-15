# Ed25519 Benchmarks

Ed25519 benchmarks in C.

## Built With

- `orlp/ed25519` repository.
- Using the repository's provided benchmarks.

## Run Benchmarks

```console
yuzo@earth:~/ed25519-bench/c$ gcc -c ed25519/src/*.c -O3 -lssl -lcrypto
yuzo@earth:~/ed25519-bench/c$ gcc ed25519/test.c *.o -o orlp.out
yuzo@earth:~/ed25519-bench/c$ ./orlp.out
```
