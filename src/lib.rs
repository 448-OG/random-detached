#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

mod generator;
pub use generator::*;

#[cfg(feature = "random_bytes")]
mod bytes;
#[cfg(feature = "random_bytes")]
pub use bytes::*;

#[cfg(feature = "chars")]
mod chars;
#[cfg(feature = "chars")]
pub use chars::*;

// Re-export
pub use getrandom;
pub use zeroize;

#[cfg(feature = "blake3_hash")]
pub use blake3;
#[cfg(feature = "chars")]
pub use rand_chacha;
#[cfg(feature = "const_cmp")]
pub use subtle;
