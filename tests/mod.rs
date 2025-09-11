use stab_decomp_simulator_rust::prelude::{SimulatorState, from_qasm_file};

use crate::common::{
    assert_eq_complex_array1, load_statevector_from_file, pretty_print_complex_vec,
};
use std::path::PathBuf;

pub mod common;

#[test]
fn test_assert_eq_precomputed_inner_product() {
    let base_path = PathBuf::from("tests")
        .join("resources")
        .join("4q_50g_cx-h-s-t");

    let circuit_path = base_path.join("circuit.qasm");
    let statevector_path = base_path.join("ref.sv");

    let circuit = from_qasm_file(circuit_path).unwrap();
    let ref_state = load_statevector_from_file(statevector_path).unwrap();

    let sim_state = SimulatorState::from_circuit(&circuit).unwrap();
    let sim_state_vec = sim_state.to_statevector();

    pretty_print_complex_vec("sim", &sim_state_vec);
    pretty_print_complex_vec("ref", &ref_state);

    assert_eq_complex_array1(&ref_state, &sim_state_vec);
}
