//! # Stabilizer State CH-Form Simulator
//!
//! A Rust library for simulating quantum stabilizer states using the CH-form representation,
//! based on the work in arXiv:1808.00128.

pub mod circuit;
pub mod error;
#[doc(hidden)]
pub mod form;
pub mod types;

#[doc(inline)]
pub use form::StabilizerCHForm;
pub mod prelude {
    pub use crate::circuit::{CliffordCircuit, CliffordGate};
    pub use crate::error::{Error, Result};
    pub use crate::form::StabilizerCHForm;
}

#[cfg(test)]
mod test_utils;
