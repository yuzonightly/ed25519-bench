// Benchmarks from the isislovecruft/dalek-benchmarks repository.
// With a few modifications.

extern crate cc;

fn main() {
    cc::Build::new()
        .file("ed25519/src/seed.c")
        .file("ed25519/src/keypair.c")
        .file("ed25519/src/sign.c")
        .file("ed25519/src/verify.c")
        .file("ed25519/src/add_scalar.c")
        .file("ed25519/src/fe.c")
        .file("ed25519/src/ge.c")
        .file("ed25519/src/key_exchange.c")
        .file("ed25519/src/sc.c")
        .file("ed25519/src/sha512.c")
        .flag("-lssl")
        .flag("-lcrypto")
        .include("ed25519/src")
        .compile("libed25519orlp.a");
}
