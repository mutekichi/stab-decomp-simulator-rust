use stab_decomp_simulator_rust::prelude::{QuantumState, from_qasm_file};

use crate::common::{assert_eq_complex_array1, load_statevector_from_file};
use std::path::PathBuf;

pub mod common;

#[test]
fn test_assert_eq_precomputed_statevector() {
    let base_path = PathBuf::from("tests")
        .join("resources")
        .join("4q_100g_cx-cz-h-s-sdg-sx-sxdg-t-tdg-x-z");

    let circuit_path = base_path.join("circuit.qasm");
    let statevector_path = base_path.join("ref.sv");

    let circuit = from_qasm_file(circuit_path).unwrap();
    let ref_state = load_statevector_from_file(statevector_path).unwrap();

    let sim_state = QuantumState::from_circuit(&circuit).unwrap();
    let sim_state_vec = sim_state.to_statevector();

    println!("sim_state_vec: {:?}", sim_state_vec[0]);
    println!("ref_state: {:?}", ref_state[0]);

    assert_eq_complex_array1(&ref_state, &sim_state_vec);
}
