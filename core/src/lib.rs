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
//! ## Typical Workflow
//!
//! 1. Construct a quantum circuit using [`QuantumCircuit`].
//! 2. Compile the circuit into a [`QuantumState`] using [`from_circuit`].
//! 3. Perform operations such as [`measure`], [`sample`], or [`exp_value`] (expectation value
//!    calculation).
//! 4. If needed, additional Clifford gates can be applied directly to the state using methods like
//!    [`apply_x`], [`apply_h`], etc.
//!
//! ## Examples
//!
//! ```rust
//! use stab_decomp_simulator_rust::prelude::{QuantumCircuit, QuantumState};
//! use stab_decomp_simulator_rust::types::PauliString;
//! use std::str::FromStr;
//!
//! // 1. Build a quantum circuit
//! let mut circuit = QuantumCircuit::new(2);
//! circuit.apply_h(0);
//! circuit.apply_cx(0, 1);
//! circuit.apply_t(1); // A non-Clifford gate
//!
//! // 2. Compile the circuit into a QuantumState
//! let mut state = QuantumState::from_circuit(&circuit).unwrap();
//!
//! // (optional) Apply a gate directly to the state
//! state.apply_x(0).unwrap();
//!
//! // 3. Perform operations on the state
//! // - Sample measurement outcomes
//! let shots = 1024;
//! let samples = state.sample(&[0, 1], shots, None).unwrap();
//! println!("Measurement samples: {:?}", samples);
//!
//! // - Calculate an expectation value
//! let pauli_z0 = PauliString::from_str("ZI").unwrap();
//! let exp_val = state.exp_value(&pauli_z0).unwrap();
//! println!("Expectation value of Z on qubit 0: {}", exp_val);
//! ```
//!
//! [`QuantumCircuit`]: crate::circuit::QuantumCircuit
//! [`QuantumState`]: crate::state::QuantumState
//! [`measure`]: crate::state::QuantumState::measure
//! [`sample`]: crate::state::QuantumState::sample
//! [`exp_value`]: crate::state::QuantumState::exp_value
//! [`from_circuit`]: crate::state::QuantumState::from_circuit
//! [`apply_x`]: crate::state::QuantumState::apply_x
//! [`apply_h`]: crate::state::QuantumState::apply_h

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

// TODO: Add appropriate references, improved the "feature" comments.
