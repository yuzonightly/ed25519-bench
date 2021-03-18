extern crate criterion;
extern crate ed25519_dalek;
extern crate rand;

use ed25519_dalek::ExpandedSecretKey;
use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use rand::prelude::ThreadRng;
use rand::thread_rng;

use criterion::{criterion_group, criterion_main, Criterion};

fn dalek_keypair_generation(c: &mut Criterion) {
    let mut csprng: ThreadRng = thread_rng();
    c.bench_function("ed25519-dalek key generation", move |b| {
        b.iter(|| Keypair::generate(&mut csprng))
    });
}

fn dalek_signature_generation(c: &mut Criterion) {
    let mut csprng: ThreadRng = thread_rng();
    let keypair = Keypair::generate(&mut csprng);
    let message: &[u8] = b"";

    c.bench_function("ed25519-dalek signature generation", move |b| {
        b.iter(|| keypair.sign(message))
    });
}

fn dalek_signature_verification(c: &mut Criterion) {
    let mut csprng: ThreadRng = thread_rng();
    let keypair = Keypair::generate(&mut csprng);
    let message: &[u8] = b"";
    let signature: Signature = keypair.sign(message);

    c.bench_function("ed25519-dalek signature verification", move |b| {
        b.iter(|| keypair.verify(message, &signature))
    });
}

criterion_group! {
    name = ed25519_fun_bench;
    config = Criterion::default();
    targets = dalek_keypair_generation,
              dalek_signature_generation,
              dalek_signature_verification
}

criterion_main!(ed25519_fun_bench);
