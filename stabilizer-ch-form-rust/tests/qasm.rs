use stabilizer_ch_form_rust::prelude::*;
use std::fs;

mod common;

#[test]
fn test_qasm_parser_roundtrip_str() {
    let n_qubits = 4;
    // Generate a random Clifford circuit
    let circuit1 = CliffordCircuit::random_clifford(n_qubits, Some(42));
    assert!(
        circuit1.gates.len() > 0,
        "Random circuit should not be empty"
    );

    // Convert to QASM string
    let qasm_str = circuit1.to_qasm_str("q");

    // Parse back from QASM string
    let circuit2 = CliffordCircuit::from_qasm_str(&qasm_str)
        .expect("QASM parsing from generated string failed");

    // Check that the circuits match
    assert_eq!(circuit1.n_qubits, circuit2.n_qubits);
    assert_eq!(
        circuit1.gates, circuit2.gates,
        "Gate sequences must match after roundtrip"
    );
}

#[test]
fn test_qasm_parser_roundtrip_file() {
    // Create a temporary file path
    let mut temp_path = std::env::temp_dir();
    let file_name = format!("test_circuit_{}.qasm", std::process::id());
    temp_path.push(file_name);
    let temp_path_str = temp_path
        .to_str()
        .expect("Failed to create temp path string");

    // Generate a random circuit and write to QASM file
    let n_qubits = 3;
    let circuit1 = CliffordCircuit::random_clifford(n_qubits, Some(123));
    circuit1
        .to_qasm_file(temp_path_str, "qr")
        .expect("Failed to write QASM to file");

    // Read the circuit back from the QASM file
    let circuit2 =
        CliffordCircuit::from_qasm_file(temp_path_str).expect("Failed to read QASM from file");

    // Check that the circuits match
    assert_eq!(circuit1.n_qubits, circuit2.n_qubits);
    assert_eq!(circuit1.gates, circuit2.gates);

    // Clean up the temporary file
    fs::remove_file(temp_path).expect("Failed to remove temporary test file");
}

#[test]
fn test_qasm_parser_errors() {
    // The parser should reject non-Clifford gates like T
    let qasm_t_gate = r#"
        OPENQASM 2.0;
        include "qelib1.inc";
        qreg q[1];
        t q[0];
    "#;

    let result = CliffordCircuit::from_qasm_str(qasm_t_gate);
    assert!(
        result.is_err(),
        "Parser should fail for non-Clifford T gate"
    );
    if let Err(e) = result {
        println!("Received expected error for T gate: {}", e);
        assert!(matches!(e, Error::QasmParsingError(_)));
    }

    // The parser should reject malformed QASM syntax (e.g., missing semicolon)
    let qasm_bad_syntax = r#"
        OPENQASM 2.0;
        qreg q[1]
        h q[0];
    "#;
    assert!(
        CliffordCircuit::from_qasm_str(qasm_bad_syntax).is_err(),
        "Parser should fail on syntax error"
    );
}
