# Ed25519 Benchmarks

Ed25519 benchmarks in C.

Benchmarked libraries:

- `orlp/ed25519`
- `floodberry/ed25519-donna`

## Built With

- Using built-in benchmarks

## Run Benchmarks

```bash
gcc -c ed25519/src/*.c -O3 -lssl -lcrypto
gcc ed25519/test.c *.o -o orlp.out
./orlp.out
```
