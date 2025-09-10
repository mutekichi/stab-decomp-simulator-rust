pub mod errors;
use errors::CompileError;
use stabilizer_ch_form_rust::{api::{CliffordCircuit, CliffordGate}, StabilizerCHForm};
use crate::{circuit::QuantumCircuit, prelude::{magic_states::t_state::_construct_t_tensor_state, types::{coefficient::Amplify, scalar::Scalar}, Coefficient, SimulatorState, StabilizerDecomposedState}};

/// A trait for compilers that transform a `QuantumCircuit` blueprint into a
/// computable `SimulatorState`.
pub trait CircuitCompiler<T: Coefficient> {
    fn compile(&self, circuit: &QuantumCircuit) -> Result<SimulatorState<T>, CompileError>;
}

/// A compiler that implements the stabilizer decomposition simulation method.
///
/// This compiler transforms a `QuantumCircuit` into a `SimulatorState` which
/// internally uses a `StabilizerDecomposedState`. It processes non-Clifford
/// gates (like T and Toffoli) in a batch by preparing the necessary magic
/// states and then applying gate teleportation.
pub struct StabDecompCompiler;

impl StabDecompCompiler {
    pub fn new() -> Self {
        Self
    }
}

impl<T: Coefficient + From<Scalar>> CircuitCompiler<T> for StabDecompCompiler {
    fn compile(&self, circuit: &QuantumCircuit) -> Result<SimulatorState<T>, CompileError> {
        let num_qubits_original = circuit.num_qubits;
        let mut num_t_type_gates = 0;
        let mut clifford_ops: Vec<CliffordGate> = Vec::new();

        for gate in &circuit.gates {
            if gate.is_clifford() {
                clifford_ops.push(gate.to_clifford_gate().unwrap());
            }
            else if gate.is_t_type_gate() {
                let ancilla_idx = num_qubits_original + num_t_type_gates;
                let target_idx = gate.qubits()[0];
                clifford_ops.push(CliffordGate::CX(target_idx, ancilla_idx));
                if gate.is_tdg_gate() {
                    clifford_ops.push(CliffordGate::Sdg(target_idx));
                }
                num_t_type_gates += 1;
            }
            else {
                return Err(CompileError::GateNotSupported(format!(
                    "Gate {:?} is not supported by the StabDecompCompiler.",
                    gate
                )));
            }
        }

        // If there are no T-gates, the circuit is purely Clifford.
        if num_t_type_gates == 0 {
            let mut circuit = CliffordCircuit::new(
                num_qubits_original
            );
            for gate in clifford_ops {
                circuit.add_gate(gate);
            }
            let ch_form = StabilizerCHForm::from_clifford_circuit(&circuit).unwrap();
            let stab_decomp_state = StabilizerDecomposedState {
                num_qubits: num_qubits_original,
                stabilizers: vec![ch_form],
                coefficients: vec![T::one()],
            };
            return Ok(SimulatorState::new(stab_decomp_state));
        }

        // Initialize the T-tensor state for the ancilla qubits.
        let t_tensor_state = _construct_t_tensor_state(num_t_type_gates);

        let mut final_stabilizers: Vec<StabilizerCHForm> = Vec::new();
        let mut final_coefficients: Vec<T> = Vec::new();

        // Process each stabilizer component of the |T^n> state.
        // NOTE: This process can be improved by "right-applying" the t-tensor
        // preparation to the whole circuit, instead of "left-applying" the
        // clifford operations to each stabilizer component.
        for (stab, coeff) in t_tensor_state.stabilizers.iter().zip(t_tensor_state.coefficients.iter()) {
            let mut full_stab_state = StabilizerCHForm::new(num_qubits_original).kron(stab);

            // Apply the clifford operations to the combined state.
            for gate in &clifford_ops {
                full_stab_state.apply_gate(gate);
            }

            let mut can_postselect_all = false;
            let mut num_deterministic_qubits = 0;

            // Iterate reverse to handle qubit index shifts after discards.
            for qubit in (num_qubits_original..(num_qubits_original + num_t_type_gates)).rev() {
                match full_stab_state.project(qubit, true) {
                    Ok(deterministic) => {
                        if deterministic {
                            num_deterministic_qubits += 1;
                        } else {
                            can_postselect_all = false;
                        }
                        full_stab_state.discard(qubit).unwrap();
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
                final_coefficients.push(T::from(coeff.amplify(num_t_type_gates.saturating_sub(num_deterministic_qubits))));
            }
        }

        let final_state = StabilizerDecomposedState {
            num_qubits: num_qubits_original,
            stabilizers: final_stabilizers,
            coefficients: final_coefficients,
        };

        Ok(SimulatorState::new(final_state))
    }
}
