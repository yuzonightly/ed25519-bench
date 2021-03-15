// Benchmarks from the isislovecruft/dalek-benchmarks repository.
// With a few modifications.

use libc::{c_uchar, size_t};

#[link(name = "ed25519orlp")]
extern "C" {
    fn ed25519_create_seed(seed: *mut c_uchar);
    fn ed25519_create_keypair(pk: *mut c_uchar, sk: *mut c_uchar, seed: *const c_uchar);
    fn ed25519_sign(
        signature: *mut c_uchar,
        message: *const c_uchar,
        mlen: size_t,
        pk: *const c_uchar,
        sk: *const c_uchar,
    );
    fn ed25519_verify(
        signature: *const c_uchar,
        message: *const c_uchar,
        mlen: size_t,
        pk: *const c_uchar,
    ) -> i32;
}

pub fn ed25519_orlp_seed(seed: &mut [u8; 32]) {
    unsafe {
        ed25519_create_seed(seed.as_mut_ptr());
    }
}

pub fn ed25519_orlp_keypair(public_key: &mut [u8; 32], secret_key: &mut [u8; 64], seed: &[u8; 32]) {
    unsafe {
        ed25519_create_keypair(
            public_key.as_mut_ptr(),
            secret_key.as_mut_ptr(),
            seed.as_ptr(),
        );
    }
}

pub fn ed25519_orlp_sign(message: &[u8], secret_key: &[u8; 32], public_key: &[u8; 32]) -> [u8; 64] {
    let mut signature: [u8; 64] = [0u8; 64];
    let message_len: size_t = message.len();

    unsafe {
        ed25519_sign(
            signature.as_mut_ptr(),
            message.as_ptr(),
            message_len,
            public_key.as_ptr(),
            secret_key.as_ptr(),
        );
    }

    signature
}

pub fn ed25519_orlp_verify(message: &[u8], public_key: &[u8; 32], signature: &[u8; 64]) -> i32 {
    let message_len: size_t = message.len();

    unsafe {
        ed25519_verify(
            signature.as_ptr(),
            message.as_ptr(),
            message_len,
            public_key.as_ptr(),
        )
    }
}
