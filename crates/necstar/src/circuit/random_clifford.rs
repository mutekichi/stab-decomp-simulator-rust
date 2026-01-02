use crate::circuit::{QuantumCircuit, QuantumGate};
use stabilizer_ch_form_rust::circuit::CliffordCircuit;

/// Generates a uniformly random n-qubit Clifford.
///
/// The resulting circuit is structured according to the canonical form U = F1 * H * S * F2.
/// See the reference for details.
///
/// ## Arguments
/// * `n` - The number of qubits. Must be greater than 0.
/// * `seed` - An optional seed for the random number generator for reproducibility.
///   If [`None`] is provided, a seed will be generated from system entropy.
///
/// ## Returns
/// A [`QuantumCircuit`] object representing the random Clifford operator.
///
/// ## Reference
/// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford
///   group," IEEE Trans. Inf. Theory 67, 5800 (2021). https://doi.org/10.1109/TIT.2021.3081415
pub(crate) fn random_clifford(n: usize, seed: Option<[u8; 32]>) -> QuantumCircuit {
    let clifford_circuit = CliffordCircuit::random_clifford(n, seed);
    let mut qc = QuantumCircuit::new(n);
    let gates_iter = clifford_circuit.gates.into_iter().map(QuantumGate::from);
    qc.gates.extend(gates_iter);

    qc
}
