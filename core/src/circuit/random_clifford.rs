use crate::circuit::{QuantumCircuit, QuantumGate};
use stabilizer_ch_form_rust::circuit::CliffordCircuit;

/// Generates a random n-qubit Clifford circuit using the Bravyi-Maslov canonical form.
///
/// This function implements the O(n^2) algorithm described in the paper to sample a Clifford operator uniformly at random from the n-qubit Clifford group.
/// The resulting circuit is structured according to the canonical form U = F1 * H * S * F2. See the reference for details.
///
/// # Arguments
/// * `n` - The number of qubits. Must be greater than 0.
/// * `seed` - An optional seed for the random number generator for reproducibility.
///
/// # Returns
/// A [`QuantumCircuit`] object representing the random Clifford operator.
///
/// # References
/// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford group," arXiv:2003.09412v2 (2021).
pub(crate) fn _random_clifford(n: usize, seed: Option<u64>) -> QuantumCircuit {
    let clifford_circuit = CliffordCircuit::random_clifford(n, seed);
    let mut qc = QuantumCircuit::new(n);
    // Convert stabilizer_ch_form_rust::circuit::CliffordGate to core::circuit::QuantumGate
    let gates_iter = clifford_circuit.gates.into_iter().map(QuantumGate::from);
    qc.gates.extend(gates_iter);

    qc
}
