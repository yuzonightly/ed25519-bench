extern crate criterion;
extern crate ed25519_donna;

use ed25519_donna::ffi;

// use ffi::ed25519_donna_curved25519_scalarmult_basepoint;
use ffi::ed25519_donna_public_key;
use ffi::ed25519_donna_sign;
use ffi::ed25519_donna_sign_open;

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

fn bench_ed25519_donna_publickey(c: &mut Criterion) {
    let mut public_key: [u8; 32] = [0u8; 32];
    c.bench_function("Ed25519-donna public key", move |b| {
        b.iter(|| ed25519_donna_public_key(&ED25519_SECRET_KEY_BYTES, &mut public_key));
    });
}

fn bench_ed25519_donna_sign(c: &mut Criterion) {
    c.bench_function("Ed25519-donna sign", move |b| {
        b.iter(|| {
            ed25519_donna_sign(
                &MESSAGE.as_bytes(),
                &ED25519_SECRET_KEY_BYTES,
                &ED25519_PUBLIC_KEY_BYTES,
            )
        });
    });
}

fn bench_ed25519_donna_sign_open(c: &mut Criterion) {
    let signature: [u8; 64] = ed25519_donna_sign(
        &MESSAGE.as_bytes(),
        &ED25519_SECRET_KEY_BYTES,
        &ED25519_PUBLIC_KEY_BYTES,
    );
    c.bench_function("Ed25519-donna verify", move |b| {
        b.iter(|| {
            ed25519_donna_sign_open(&MESSAGE.as_bytes(), &ED25519_PUBLIC_KEY_BYTES, &signature)
        });
    });
}

criterion_group! {
    name = ed25519_donna_bench;
    config = Criterion::default();
    targets = bench_ed25519_donna_publickey,
    bench_ed25519_donna_sign,
    bench_ed25519_donna_sign_open,
}

criterion_main!(ed25519_donna_bench);
