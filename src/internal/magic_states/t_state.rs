use crate::prelude::StabilizerDecomposedState;
use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::*;

/// Returns (|0^n> - i|1^n>) / sqrt(2) as a StabilizerCHForm
fn _zero_minus_i_one_state(num_qubits: usize) -> StabilizerCHForm {
    let mut state = StabilizerCHForm::new(num_qubits);
    state.apply_h(0);
    for i in 1..num_qubits {
        state.apply_cx(0, i);
    }
    state.apply_sdg(0);
    state
}

/// Returns |E_m> = (Σ_{x: x is even} |x>) / sqrt(2^(n-1)) as a StabilizerCHForm
fn _even_parity_state(num_qubits: usize) -> StabilizerCHForm {
    let mut state = StabilizerCHForm::new(num_qubits);
    for i in 0..num_qubits - 1 {
        state.apply_h(i);
    }
    for i in 0..num_qubits - 1 {
        state.apply_cx(i, num_qubits - 1);
    }
    state
}

/// Returns |K_m> = Π_{1<=i<j<=m} CZ_{i,j} |E_m> as a StabilizerCHForm
fn _even_parity_phase_flipped_state(num_qubits: usize) -> StabilizerCHForm {
    let mut state = _even_parity_state(num_qubits);
    for i in 0..num_qubits {
        for j in (i + 1)..num_qubits {
            state.apply_cz(i, j);
        }
    }
    state
}

/// Apply X then S on the target qubit
fn _apply_xs(state: &mut StabilizerCHForm, target: usize) {
    state.apply_x(target);
    state.apply_s(target);
}

/// Construct |cat_1> = |0> state as a StabilizerDecomposedState
fn _construct_cat_1_state() -> StabilizerDecomposedState {
    let stab = StabilizerCHForm::new(1);
    let coeffs = vec![Complex64::new(1.0, 0.0)];

    StabilizerDecomposedState {
        num_qubits: 1,
        stabilizers: vec![stab],
        coefficients: coeffs,
    }
}

/// Construct |cat_2> = (|00> + i|11>) / sqrt(2) state as a StabilizerDecomposedState
fn _construct_cat_2_state() -> StabilizerDecomposedState {
    let mut stab = StabilizerCHForm::new(2);
    stab.apply_h(0);
    stab.apply_cx(0, 1);
    stab.apply_s(1);
    let coeffs = vec![Complex64::new(1.0, 0.0)];

    StabilizerDecomposedState {
        num_qubits: 2,
        stabilizers: vec![stab],
        coefficients: coeffs,
    }
}

// /// Construct |cat_4> state as a superposition of 2 StabilizerCHForm states
//
// fn construct_cat_4_state() -> StabilizerDecomposedState {
//     let mut stab1 = StabilizerCHForm::new(4);
//     stab1 = _zero_minus_i_one_state(4);
//     stab2 = _even_parity_state(4);
//     coeffs = vec![
//         Complex64::new(1.0 / 2.0, 0.0),
//         Complex64::new(1.0 / 2.0, 0.0),
//     ];

//     StabilizerDecomposedState {
//         num_qubits: 4,
//         stabilizers: vec![stab1, stab2],
//         coefficients: coeffs,
//     }
// }
