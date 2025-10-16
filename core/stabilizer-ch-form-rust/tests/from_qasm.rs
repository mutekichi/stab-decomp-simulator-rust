mod common;

use stabilizer_ch_form_rust::api::parser::from_qasm_file;
use stabilizer_ch_form_rust::prelude::*;
use std::fs;
use std::path::Path;

const TEST_DATA_DIR: &str = "tests/resources";

#[test]
fn compare_with_qiskit_references() {
    let resources = Path::new(TEST_DATA_DIR);
    if !resources.exists() {
        println!(
            "Skipping reference tests: '{}' directory not found.",
            TEST_DATA_DIR
        );
        return;
    }

    for entry in fs::read_dir(resources).expect("Failed to read test resources directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_dir() {
            let test_case_name = path.file_name().unwrap().to_str().unwrap();
            println!("--- Running test case: {} ---", test_case_name);

            let qasm_path = path.join("circuit.qasm");
            let sv_path = path.join("ref.sv");

            assert!(
                qasm_path.exists(),
                "circuit.qasm not found in {}",
                test_case_name
            );
            assert!(sv_path.exists(), "ref.sv not found in {}", test_case_name);

            let circuit = from_qasm_file(&qasm_path)
                .unwrap_or_else(|e| panic!("Failed to parse QASM for {}: {}", test_case_name, e));

            let ch_form = StabilizerCHForm::from_clifford_circuit(&circuit).unwrap_or_else(|e| {
                panic!("Failed to convert to CH-form for {}: {}", test_case_name, e)
            });

            let our_statevector = ch_form.to_statevector().unwrap_or_else(|e| {
                panic!(
                    "Failed to compute statevector for {}: {}",
                    test_case_name, e
                )
            });

            let reference_statevector = common::load_statevector_from_file(&sv_path)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to load reference statevector for {}: {}",
                        test_case_name, e
                    )
                });

            common::assert_eq_complex_array1(&our_statevector, &reference_statevector);

            println!("--- Test case '{}' passed! ---\n", test_case_name);
        }
    }
}
