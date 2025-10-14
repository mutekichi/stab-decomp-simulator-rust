//! # NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust
//!
//! A high-performance quantum circuit simulator designed for the strong simulation of
//! near-Clifford circuits.
//!
//! NECSTAR is particularly effective for circuits dominated by Clifford gates
//! but also contain a small number of non-Clifford gates, such as the T-gate.
//! It provides an intuitive API for building and simulating quantum circuits.
//!
//! # Features
//!
//! * **Stabilizer Decomposition Core**: The simulator's engine is built on the stabilizer
//!   decomposition method. Instead of representing the state vector in a memory-intensive
//!   dense vector, it maintains the quantum state as a linear combination of stabilizer states.
//!   This approach is highly efficient for circuits with low non-Clifford gate counts and
//!   relatively large qubit numbers.
//!
//! * **Magic State Teleportation**: Non-Clifford gates are handled using the gate teleportation
//!   protocol. The required magic states (e.g., T-states) are themselves represented using
//!   stabilizer decompositions, allowing the entire simulation to remain within the stabilizer
//!   formalism.
//!
//! * **Intuitive Declarative API**: Users can define quantum computations by declaratively
//!   building a [`QuantumCircuit`]. This circuit object is then compiled into a [`QuantumState`],
//!   which provides a clean interface for simulation tasks, abstracting away the complex
//!   internal state representation.
//!
//! * **Strong, Exact Simulation**: Necstar performs strong simulation, calculating the full
//!   final quantum state with exact amplitudes. No approximations are used, ensuring results
//!   are accurate and suitable for verifying quantum algorithms or investigating the power of
//!   non-Clifford resources.
//!
//! # Usage Examples
//!
//! ```rust
//! // TODO: Add example code here
//! ```
//!
//! [`QuantumCircuit`]: crate::circuit::QuantumCircuit
//! [`QuantumState`]: crate::state::QuantumState

pub mod circuit;
pub mod error;
pub mod state;
pub mod types;

pub mod prelude {
    pub use crate::circuit::*;
    pub use crate::error::*;
    pub use crate::state::QuantumState;
    pub use crate::types::*;
}

// Hide test_utils from the public documentation.
#[cfg(test)]
pub mod test_utils;
