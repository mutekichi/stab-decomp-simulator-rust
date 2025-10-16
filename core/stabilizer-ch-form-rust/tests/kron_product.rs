// tests/kron.rs
mod common;
use common::{assert_eq_complex_array1, random_circuit};
use ndarray::Array1;
use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::*;

/// Naive implementation of kronecker product for statevectors,
/// adjusted for the little-endian convention of `to_statevector`.
fn kron_statevectors(sv1: &Array1<Complex64>, sv2: &Array1<Complex64>) -> Array1<Complex64> {
    let n1 = sv1.len();
    let n2 = sv2.len();
    let mut result = Array1::zeros(n1 * n2);

    for j in 0..n2 {
        for i in 0..n1 {
            result[j * n1 + i] = sv1[i] * sv2[j];
        }
    }
    result
}

#[test]
fn test_kron_random_circuits() {
    let n_qubits1 = 2;
    let n_qubits2 = 3;
    let gate_count = 30;
    let iterations = 10;

    for i in 0..iterations {
        let circuit1 = random_circuit(n_qubits1, gate_count, Some(100 + i));
        let circuit2 = random_circuit(n_qubits2, gate_count, Some(200 + i));

        let ch_form1 = StabilizerCHForm::from_clifford_circuit(&circuit1).unwrap();
        let ch_form2 = StabilizerCHForm::from_clifford_circuit(&circuit2).unwrap();

        // Method 1: Use the kron method. The result is converted to a statevector
        // using the little-endian `to_statevector`.
        let kron_ch_form = ch_form1.kron(&ch_form2).unwrap();
        let actual_statevector = kron_ch_form.to_statevector().unwrap();

        // Method 2: Naively compute the kronecker product of statevectors
        // using a helper function that respects the little-endian ordering.
        let sv1 = ch_form1.to_statevector().unwrap();
        let sv2 = ch_form2.to_statevector().unwrap();
        let expected_statevector = kron_statevectors(&sv1, &sv2);

        dbg!(&expected_statevector);
        dbg!(&actual_statevector);

        // Compare the results
        assert_eq_complex_array1(&expected_statevector, &actual_statevector);

        println!(
            "Test {}: Passed. Kron results match for circuits of size {} and {}.",
            i + 1,
            n_qubits1,
            n_qubits2
        );
    }
}
