// // Benchmarks from the isislovecruft/dalek-benchmarks repository.
// // With a few modifications.

// use libc::{c_int, c_uchar, size_t};

// #[link(name = "ed25519orlp")]
// extern "C" {
//     fn ed25519_create_keypair(pk: *mut c_uchar, sk: *mut c_uchar, seed: *const c_uchar);
//     fn ed25519_create_seed(seed: *mut c_uchar);
// }
// // int ed25519_create_seed(unsigned char *seed) {

// // void ed25519_create_keypair(unsigned char *public_key, unsigned char *private_key, const unsigned char *seed) {
// //     ge_p3 A;

// //     sha512(seed, 32, private_key);
// //     private_key[0] &= 248;
// //     private_key[31] &= 63;
// //     private_key[31] |= 64;

// //     ge_scalarmult_base(&A, private_key);
// //     ge_p3_tobytes(public_key, &A);
// // }

// pub fn ed25519_orlp_seed(seed: &mut [u8; 32]) {
//     unsafe {
//         ed25519_create_seed(seed.as_mut_ptr());
//     }
// }

// pub fn ed25519_orlp_keypair(
//     public_key: &mut [u8; 32],
//     secret_key: &mut [u8; 64],
//     seed: &[u8; 32],
// ) {
//     unsafe {
//         ed25519_create_keypair(
//             public_key.as_mut_ptr(),
//             secret_key.as_mut_ptr(),
//             seed.as_ptr(),
//         );
//     }
// }

// // pub fn ed25519_donna_sign(
// //     message: &[u8],
// //     secret_key: &[u8; 32],
// //     public_key: &[u8; 32],
// // ) -> [u8; 64] {
// //     let mut signature: [u8; 64] = [0u8; 64];
// //     let message_len: size_t = message.len();

// //     unsafe {
// //         ed25519_sign(
// //             message.as_ptr(),
// //             message_len,
// //             secret_key.as_ptr(),
// //             public_key.as_ptr(),
// //             signature.as_mut_ptr(),
// //         );
// //     }

// //     signature
// // }

// // pub fn ed25519_donna_sign_open(message: &[u8], public_key: &[u8; 32], signature: &[u8; 64]) -> i32 {
// //     let message_len: size_t = message.len();

// //     unsafe {
// //         ed25519_sign_open(
// //             message.as_ptr(),
// //             message_len,
// //             public_key.as_ptr(),
// //             signature.as_ptr(),
// //         )
// //     }
// // }
