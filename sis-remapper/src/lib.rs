#![feature(stdarch_x86_avx512)]

#[cfg(feature = "testable_privates")]
pub mod corsair;
#[cfg(feature = "testable_privates")]
pub mod simd_corsair;
