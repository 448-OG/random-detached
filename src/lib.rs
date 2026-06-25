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
