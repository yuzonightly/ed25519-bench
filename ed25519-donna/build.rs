// Benchmarks from the isislovecruft/dalek-benchmarks repository.
// With a few modifications.

extern crate cc;

fn main() {
    cc::Build::new()
        .file("ed25519-donna/ed25519.c")
        .define("ED25519_SSE2", None)
        .include("/usr/include/openssl")
        .include("/usr/include")
        .include("ed25519-donna")
        .define("ED25519_REFHASH", None)
        // .flag("-m64")
        .flag("-lssl")
        .flag("-lcrypto")
        .compile("libed25519donna.a");
}
