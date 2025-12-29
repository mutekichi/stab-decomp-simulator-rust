pub mod error;
use crate::{
    circuit::QuantumCircuit,
    state::{
        InternalState, StabilizerDecomposedState,
        magic_states::t_state::construct_t_tensor_state,
        types::{coefficient::Amplify, scalar::Scalar},
    },
};
use error::{Error as CompileError, Result as CompileResult};
use num_traits::One;
use stabilizer_ch_form_rust::{
    StabilizerCHForm,
    circuit::{CliffordCircuit, CliffordGate},
};

/// A trait for compilers that transform a [`QuantumCircuit`] blueprint into a
/// computable [`InternalState`].
pub(crate) trait CircuitCompiler {
    fn compile(&self, circuit: &QuantumCircuit) -> Result<InternalState, CompileError>;
}

/// A compiler that implements the stabilizer decomposition simulation method.
///
/// This compiler transforms a [`QuantumCircuit`] into a [`InternalState`] which
/// internally uses a [`StabilizerDecomposedState`]. It processes non-Clifford
/// gates (like T and Toffoli) in a batch by preparing the necessary magic
/// states and then applying gate teleportation.
pub(crate) struct StabDecompCompiler;

impl StabDecompCompiler {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl CircuitCompiler for StabDecompCompiler {
    /// Compiles a [`QuantumCircuit`] into an [`InternalState`] using stabilizer decomposition.
    /// NOTE: Currently only supports Clifford + T circuits.
    /// TODO: Generalize by abstracting magic state preparation and gate teleportation
    /// to support arbitrary non-Clifford gates for better extensibility.
    fn compile(&self, circuit: &QuantumCircuit) -> CompileResult<InternalState> {
        let num_qubits_original = circuit.n_qubits;
        let mut num_t_type_gates = 0;
        let mut clifford_ops: Vec<CliffordGate> = Vec::new();

        for gate in &circuit.gates {
            if gate.is_clifford() {
                clifford_ops.push(gate.to_clifford_gate().unwrap());
            } else if gate.is_t_type_gate() {
                let ancilla_idx = num_qubits_original + num_t_type_gates;
                let target_idx = gate.qubits()[0];
                clifford_ops.push(CliffordGate::CX(target_idx, ancilla_idx));
                if gate.is_tdg_gate() {
                    clifford_ops.push(CliffordGate::Sdg(target_idx));
                }
                num_t_type_gates += 1;
            } else {
                return Err(CompileError::GateNotSupported(gate.name().to_string()));
            }
        }

        // If there are no T-gates, the circuit is purely Clifford.
        if num_t_type_gates == 0 {
            let mut circuit = CliffordCircuit::new(num_qubits_original);
            for gate in clifford_ops {
                circuit.add_gate(gate);
            }
            let ch_form = StabilizerCHForm::from_clifford_circuit(&circuit).unwrap();
            let stab_decomp_state = StabilizerDecomposedState::new(
                num_qubits_original,
                vec![ch_form],
                vec![Scalar::one()],
            );
            return Ok(InternalState::StabilizerDecomposedStateScalar(
                stab_decomp_state,
            ));
        }

        // Initialize the T-tensor state for the ancilla qubits.
        let t_tensor_state = construct_t_tensor_state(num_t_type_gates).unwrap();

        let mut final_stabilizers: Vec<StabilizerCHForm> = Vec::new();
        let mut final_coefficients: Vec<Scalar> = Vec::new();

        // Process each stabilizer component of the |T^n> state.
        // NOTE: This process may be improved by "right-applying" the t-tensor
        // preparation to the whole circuit, instead of "left-applying" the
        // clifford operations to each stabilizer component.
        for (stab, coeff) in t_tensor_state
            .stabilizers
            .iter()
            .zip(t_tensor_state.coefficients.iter())
        {
            let mut full_stab_state = StabilizerCHForm::new(num_qubits_original)?.kron(stab)?;

            // Apply the clifford operations to the combined state.
            for gate in &clifford_ops {
                full_stab_state.apply_gate(gate)?;
            }

            let mut can_postselect_all = true;
            let mut num_deterministic_qubits = 0;

            // Iterate reverse to handle qubit index shifts after discards.
            for qubit in (num_qubits_original..(num_qubits_original + num_t_type_gates)).rev() {
                match full_stab_state.project(qubit, false) {
                    Ok(deterministic) => {
                        if deterministic {
                            num_deterministic_qubits += 1;
                        }
                    }
                    Err(_) => {
                        can_postselect_all = false;
                        break;
                    }
                }
            }

            // If the post-selection was successful, add the resulting state with the normalized coefficient.
            if can_postselect_all {
                // Reversely discard the ancilla qubits
                for qubit in (num_qubits_original..(num_qubits_original + num_t_type_gates)).rev() {
                    full_stab_state.discard(qubit).unwrap();
                }
                final_stabilizers.push(full_stab_state);
                final_coefficients.push(coeff.amplify(num_deterministic_qubits));
            }
        }

        let final_state = StabilizerDecomposedState::new(
            num_qubits_original,
            final_stabilizers,
            final_coefficients,
        );

        Ok(InternalState::StabilizerDecomposedStateScalar(final_state))
    }
}
