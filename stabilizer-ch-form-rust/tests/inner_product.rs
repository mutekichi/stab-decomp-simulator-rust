mod common;
use common::{assert_eq_complex, random_circuit};
use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::*;

#[test]
fn test_inner_product_random_circuits() {
    let n_qubits = 5;
    let gate_count = 50;
    let iterations = 30; // Number of test iterations

    for i in 0..iterations {
        // Generate two different random circuits for each iteration
        let circuit1 = random_circuit(n_qubits, gate_count, None);
        let circuit2 = random_circuit(n_qubits, gate_count, None);

        let ch_form1 = StabilizerCHForm::from_clifford_circuit(&circuit1).unwrap();
        let ch_form2 = StabilizerCHForm::from_clifford_circuit(&circuit2).unwrap();

        // Method 1: Naively compute statevectors and then their inner product
        let statevector1 = ch_form1.to_statevector();
        let statevector2 = ch_form2.to_statevector();
        let expected_inner_product: Complex64 = statevector1
            .iter()
            .zip(statevector2.iter())
            .map(|(a, b)| a.conj() * b)
            .sum();

        // Method 2: Use the inner_product method of StabilizerCHForm
        let actual_inner_product = ch_form1.inner_product(&ch_form2);

        // Check that both methods give the same result
        assert_eq_complex(expected_inner_product, actual_inner_product);

        // If the assertion passes, print the agreed-upon value
        println!(
            "Test {}: Passed. Inner Product = {}",
            i + 1,
            actual_inner_product
        );
    }
}
