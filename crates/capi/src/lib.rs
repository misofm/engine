//! Stable Engine V2 C ABI declarations and ownership boundary.
//!
//! The checked-in C header is the public contract. This crate mirrors that fixed-width contract
//! and keeps all raw-pointer operations in the private `ffi` module.

mod abi;
mod ffi;
mod runtime;

pub use abi::*;
pub use ffi::*;
