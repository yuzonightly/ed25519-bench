# Ed25519 Benchmarks

Ed25519 benchmarks using Rust's FFI, considering that the overhead introduced is negligible.

Benchmarked libraries:

- `floodyberry/ed25519-donna`.

## Built With

- `cc`
- `libc`
- `criterion`

## Run Benchmarks

```console
yuzo@earth:~/ed25519-bench/ed25519-donna$ cargo bench
```
