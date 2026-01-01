//! # NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust
//!
//! A high-performance quantum circuit simulator designed for the strong simulation of
//! near-Clifford circuits based on the stabilizer decomposition method [1].
//!
//! NECSTAR is particularly effective for circuits dominated by Clifford gates
//! but also containing a small number of non-Clifford gates. Currently, NECSTAR supports
//! only T-gates as non-Clifford operations, but future versions may include additional
//! non-Clifford gates.
//!
//! # Features
//!
//! * **Stabilizer Decomposition Core**: The simulator represents the quantum state as a linear
//!   combination of stabilizer states \[1\]. This approach avoids the memory overhead of dense
//!   state vectors and is efficient for circuits with low non-Clifford gate counts.
//!
//! * **Magic State Teleportation**: Non-Clifford gates are applied via the gate teleportation
//!   protocol using magic states. A T-gate is implemented by consuming a T-state and the tensor
//!   product of T-states is automatically decomposed into stabilizer states \[2\].
//!
//! * **Intuitive Declarative API**: Users can define quantum computations by building a
//!   [`QuantumCircuit`]. This is compiled into a [`QuantumState`], which manages the
//!   internal stabilizer decomposition and provides a clean interface for simulation.
//!
//! # References
//!
//! - \[1\] S. Bravyi, D. Browne, P. Calpin, E. Campbell, D. Gosset, and M. Howard,
//!   "Simulation of quantum circuits by low-rank stabilizer decompositions",
//!   Quantum 3, 181 (2019). <https://doi.org/10.22331/q-2019-09-02-181>
//! - \[2\] H. Qassim, H. Pashayan, and D. Gosset,
//!   "Improved upper bounds on the stabilizer rank of magic states",
//!   Quantum 5, 604 (2021). <https://doi.org/10.22331/q-2021-12-20-606>
//!
//! ## Typical Workflow
//!
//! 1. Construct a quantum circuit using [`QuantumCircuit`].
//! 2. Compile the circuit into a [`QuantumState`] using [`from_circuit`].
//! 3. Perform operations such as [`measure`], [`sample`], or [`exp_value`].
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
//! circuit.apply_t(1); // Non-Clifford T-gate
//!
//! // 2. Compile into a QuantumState (internally decomposes T-states)
//! let mut state = QuantumState::from_circuit(&circuit).unwrap();
//!
//! // 3. Perform operations
//! let shots = 1024;
//! let samples = state.sample(&[0, 1], shots, None).unwrap();
//! println!("Samples: {:?}", samples);
//!
//! let pauli_z0 = PauliString::from_str("ZI").unwrap();
//! let exp_val = state.exp_value(&pauli_z0).unwrap();
//! println!("Expectation value: {}", exp_val);
//!
//! // (Optional) Check the stabilizer rank
//! println!("Stabilizer rank: {}", state.stabilizer_rank());
//! ```
//!
//! [`QuantumCircuit`]: crate::circuit::QuantumCircuit
//! [`QuantumState`]: crate::state::QuantumState
//! [`measure`]: crate::state::QuantumState::measure
//! [`sample`]: crate::state::QuantumState::sample
//! [`exp_value`]: crate::state::QuantumState::exp_value
//! [`from_circuit`]: crate::state::QuantumState::from_circuit

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

#[cfg(test)]
pub mod test_utils;
