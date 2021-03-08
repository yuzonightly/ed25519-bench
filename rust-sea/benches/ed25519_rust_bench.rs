extern crate criterion;
extern crate ed25519_fun;

use ed25519_fun::{Keypair, Signature};

use criterion::{criterion_group, criterion_main, Criterion};

fn keypair_generation(c: &mut Criterion) {
    c.bench_function("ed25519-fun key generation", move |b| {
        b.iter(|| Keypair::generate())
    });
}

fn signature_generation(c: &mut Criterion) {
    let keypair = Keypair::generate();
    let message: &[u8] = b"";

    c.bench_function("ed25519-fun signature generation", move |b| {
        b.iter(|| keypair.sign(message))
    });
}

fn signature_verification(c: &mut Criterion) {
    let keypair = Keypair::generate();
    let message: &[u8] = b"";
    let signature: Signature = keypair.sign(message);

    c.bench_function("ed25519-fun signature verification", move |b| {
        b.iter(|| keypair.verify(message, signature))
    });
}

criterion_group! {
    name = ed25519_rust_bench;
    config = Criterion::default();
    targets = keypair_generation,
              signature_generation,
              signature_verification
}

criterion_main!(ed25519_rust_bench);
