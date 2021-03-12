// #![cfg_attr(feature = "bench", feature(test))]
// #[cfg(all(test, feature = "bench"))]
extern crate libc;

// #[cfg(all(test, feature = "bench"))]
extern crate criterion;

pub mod donna_ffi;
pub mod orlp_ffi;

// #[cfg(all(test, feature = "bench"))]
// mod bench;

pub use crate::donna_ffi::*;
pub use crate::orlp_ffi::*;