//! A quantum circuit simulator based on stabilizer decomposition.
//!
//! This library provides tools to build and simulate quantum circuits,
//! with a focus on circuits dominated by Clifford gates.
//!
//! # Example
//!
//! ```rust
//! use stab_decomp_simulator_rust::prelude::*;
//!
//! // Create a 2-qubit circuit
//! let mut circuit = QuantumCircuit::new(2);
//! circuit.apply_h(0);
//! circuit.apply_cx(0, 1);
//!
//! // Simulate the circuit
//! let state = SimulatorState::from_circuit(&circuit).unwrap();
//!
//! // (Further operations like calculating expectation values, etc.)
//! ```

// Make circuit-related structures public.
pub mod circuit;
// Make simulator-related structures public.
pub mod simulator;

// The prelude module provides a convenient way to import the most common types.
pub mod prelude {
    pub use crate::circuit::{QuantumCircuit, QuantumGate, from_qasm_file, from_qasm_str};
    pub use crate::simulator::SimulatorState;
    // As you add more public APIs, re-export them here.
}

// Hide test_utils from the public documentation.
#[cfg(test)]
pub mod test_utils;