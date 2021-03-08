// #![cfg_attr(feature = "bench", feature(test))]
// #[cfg(all(test, feature = "bench"))]
extern crate libc;

// #[cfg(all(test, feature = "bench"))]
extern crate criterion;

// #[cfg(all(test, feature = "bench"))]
pub mod ffi;

// #[cfg(all(test, feature = "bench"))]
// mod bench;

pub use crate::ffi::*;

// | tee bench_results