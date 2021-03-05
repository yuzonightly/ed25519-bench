// Benchmarks from the isislovecruft/dalek-benchmarks repository.
// With a few modifications.

use libc::{c_int, c_uchar, size_t};

#[link(name = "ed25519donna")]
extern "C" {
    fn ed25519_publickey(sk: *const c_uchar, pk: *mut c_uchar);
    fn ed25519_sign(
        m: *const c_uchar,
        mlen: size_t,
        sk: *const c_uchar,
        pk: *const c_uchar,
        RS: *mut c_uchar,
    );
    fn ed25519_sign_open(
        m: *const c_uchar,
        mlen: size_t,
        pk: *const c_uchar,
        RS: *const c_uchar,
    ) -> c_int;
}

pub fn ed25519_donna_public_key(secret_key: &[u8; 32], public_key: &mut [u8; 32]) {
    unsafe {
        ed25519_publickey(secret_key.as_ptr(), public_key.as_mut_ptr());
    }
}

pub fn ed25519_donna_sign(
    message: &[u8],
    secret_key: &[u8; 32],
    public_key: &[u8; 32],
) -> [u8; 64] {
    let mut signature: [u8; 64] = [0u8; 64];
    let message_len: size_t = message.len();

    unsafe {
        ed25519_sign(
            message.as_ptr(),
            message_len,
            secret_key.as_ptr(),
            public_key.as_ptr(),
            signature.as_mut_ptr(),
        );
    }

    signature
}

pub fn ed25519_donna_sign_open(message: &[u8], public_key: &[u8; 32], signature: &[u8; 64]) -> i32 {
    let message_len: size_t = message.len();

    unsafe {
        ed25519_sign_open(
            message.as_ptr(),
            message_len,
            public_key.as_ptr(),
            signature.as_ptr(),
        )
    }
}
