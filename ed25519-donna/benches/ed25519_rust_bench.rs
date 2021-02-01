extern crate criterion;
extern crate ed25519_rust;

use ed25519::{Keypair, Signature};
use ed25519_rust::ed25519;

use criterion::{criterion_group, criterion_main, Criterion};

fn keypair_generation(c: &mut Criterion) {
    c.bench_function("ed25519-rust public key", move |b| {
        b.iter(|| Keypair::generate_keypair())
    });
}

fn signature_generation(c: &mut Criterion) {
    let keypair = Keypair::generate_keypair();
    let message: &[u8] = b"";

    c.bench_function("ed25519-rust sign", move |b| {
        b.iter(|| keypair.sign(message))
    });
}

fn signature_verification(c: &mut Criterion) {
    let keypair = Keypair::generate_keypair();
    let message: &[u8] = b"";
    let signature: Signature = keypair.sign(message);

    c.bench_function("ed25519-rust verify", move |b| {
        b.iter(|| keypair.verify(message, &signature.0))
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
