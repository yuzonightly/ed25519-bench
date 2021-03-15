# Ed25519 Benchmarks

Ed25519 benchmarks using Rust's FFI, considering that the overhead introduced is neglidible.

Benchmarked libraries:

- `orlp/ed25519`.

## Built With

- `cc`
- `libc`
- `criterion`

## Run Benchmarks

```console
yuzo@earth:~/ed25519-bench/ed25519-orlp$ cargo bench
```
