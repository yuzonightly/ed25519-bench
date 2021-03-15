// Benchmarks from the isislovecruft/dalek-benchmarks repository.
// With a few modifications.

extern crate criterion;
extern crate ed25519_orlp;
extern crate hex;
extern crate itertools;

use ed25519_orlp::orlp_ffi;

use orlp_ffi::ed25519_orlp_keypair;
use orlp_ffi::ed25519_orlp_sign;
use orlp_ffi::ed25519_orlp_verify;

use criterion::{criterion_group, criterion_main, Criterion};

const MESSAGE: &'static str = "This is a test of the tsunumi alert system. This is just a test.";

const ED25519_SECRET_KEY_BYTES: [u8; 32] = [
    157, 097, 177, 157, 239, 253, 090, 096, 186, 132, 074, 244, 146, 236, 044, 196, 068, 073, 197,
    105, 123, 050, 105, 025, 112, 059, 172, 003, 028, 174, 127, 096,
];

const ED25519_PUBLIC_KEY_BYTES: [u8; 32] = [
    215, 090, 152, 001, 130, 177, 010, 183, 213, 075, 254, 211, 201, 100, 007, 058, 014, 225, 114,
    243, 218, 166, 035, 037, 175, 002, 026, 104, 247, 007, 081, 026,
];

fn bench_ed25519_orlp_keypair(c: &mut Criterion) {
    let mut public_key: [u8; 32] = [0u8; 32];
    let mut secret_key: [u8; 64] = [0u8; 64];

    let seed_hex =
        hex::decode("e86f53df947cf83e92f3d36adde45b97db324fa85693b8f47f4caf222cd71999").unwrap();
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&seed_hex);

    c.bench_function("ed25519-orlp key generation", move |b| {
        b.iter(|| ed25519_orlp_keypair(&mut public_key, &mut secret_key, &seed));
    });
}

fn bench_ed25519_orlp_sign(c: &mut Criterion) {
    c.bench_function("ed25519-orlp signature generation", move |b| {
        b.iter(|| {
            ed25519_orlp_sign(
                &MESSAGE.as_bytes(),
                &ED25519_SECRET_KEY_BYTES,
                &ED25519_PUBLIC_KEY_BYTES,
            )
        });
    });
}

fn bench_ed25519_orlp_sign_open(c: &mut Criterion) {
    let signature: [u8; 64] = ed25519_orlp_sign(
        &MESSAGE.as_bytes(),
        &ED25519_SECRET_KEY_BYTES,
        &ED25519_PUBLIC_KEY_BYTES,
    );
    c.bench_function("ed25519-orlp signature verification", move |b| {
        b.iter(|| ed25519_orlp_verify(&MESSAGE.as_bytes(), &ED25519_PUBLIC_KEY_BYTES, &signature));
    });
}

criterion_group! {
    name = ed25519_orlp_bench;
    config = Criterion::default();
    targets = bench_ed25519_orlp_keypair,
    bench_ed25519_orlp_sign,
    bench_ed25519_orlp_sign_open,
}

criterion_main!(ed25519_orlp_bench);
