//! # Stabilizer State CH-Form Simulator
//!
//! A Rust library for simulating quantum stabilizer states using the CH-form representation,
//! based on the work in arXiv:1808.00128.

pub mod api;
pub mod core;
pub mod types;
pub mod error;

pub use core::StabilizerCHForm;
pub mod prelude {
    pub use crate::api::*;
    pub use crate::core::StabilizerCHForm;
    pub use crate::types::*;
}
