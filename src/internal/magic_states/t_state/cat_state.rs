use core::num;

use crate::prelude::{
    StabilizerDecomposedState,
    types::{phase_factor::PhaseFactor, scalar::Scalar},
};
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

/// Construct |cat_1> = |0> state as a StabilizerDecomposedState
fn _construct_cat_1_state() -> StabilizerDecomposedState<Scalar> {
    let stab = StabilizerCHForm::new(1);
    let coeffs = vec![Scalar::ONE];

    StabilizerDecomposedState {
        num_qubits: 1,
        stabilizers: vec![stab],
        coefficients: coeffs,
    }
}

/// Construct |cat_2> = (|00> + i|11>) / sqrt(2) state as a StabilizerDecomposedState
fn _construct_cat_2_state() -> StabilizerDecomposedState<Scalar> {
    let mut stab = StabilizerCHForm::new(2);
    stab.apply_h(0);
    stab.apply_cx(0, 1);
    stab.apply_s(1);
    let coeffs = vec![Scalar::ONE];

    StabilizerDecomposedState {
        num_qubits: 2,
        stabilizers: vec![stab],
        coefficients: coeffs,
    }
}

/// Construct |cat_4> state as a superposition of 2 StabilizerCHForm states
fn construct_cat_4_state() -> StabilizerDecomposedState<Scalar> {
    let stab1 = _zero_minus_i_one_state(4);
    let stab2 = _even_parity_state(4);
    let coeffs = vec![Scalar::ONE_OVER_SQRT_2, Scalar::ONE_OVER_SQRT_2];

    StabilizerDecomposedState {
        num_qubits: 4,
        stabilizers: vec![stab1, stab2],
        coefficients: coeffs,
    }
}

/// Construct |cat_6> state as a superposition of 3 StabilizerCHForm states
fn construct_cat_6_state() -> StabilizerDecomposedState<Scalar> {
    let stab1 = _zero_minus_i_one_state(6);
    let stab2 = _even_parity_state(6);
    let stab3 = _even_parity_phase_flipped_state(6);
    let coeffs = vec![
        Scalar::NonZero {
            phase: PhaseFactor::PLUS_ONE,
            r: 2,
        }, // 0.5
        Scalar::NonZero {
            phase: PhaseFactor::EXP_I_3PI_4,
            r: 1,
        }, // (-1+i)/sqrt(2)
        Scalar::NonZero {
            phase: PhaseFactor::EXP_I_5PI_4,
            r: 1,
        }, // (1+i)/sqrt(2)
    ];

    StabilizerDecomposedState {
        num_qubits: 6,
        stabilizers: vec![stab1, stab2, stab3],
        coefficients: coeffs,
    }
}

fn _project_ch_form_onto_cat_state(
    state: &mut StabilizerCHForm,
    qubits: &[usize],
) {
    // Make sure qubits has length 2 and qubits[0] < qubits[1]
    // We do not check this here for performance reasons
    state.apply_sdg(qubits[0]);
    state.apply_cx(qubits[0], qubits[1]);
    state.apply_h(qubits[0]);
    state.project(qubits[0], false).unwrap();
    state.project(qubits[1], false).unwrap();
    state.discard(qubits[1]).unwrap();
    state.discard(qubits[0]).unwrap();
}

fn _project_stab_decomp_state_onto_cat_state(
    state: &mut StabilizerDecomposedState<Scalar>,
    qubits: &[usize],
) {
    for stab in &mut state.stabilizers {
        _project_ch_form_onto_cat_state(stab, qubits);
    }
    state.num_qubits -= 2;
}

/// Make |cat_{m-1}> from |cat_m> by tracing out the last qubits
fn _reduce_cat_state(
    state: &mut StabilizerDecomposedState<Scalar>,
) {
    let num_qubits = state.num_qubits;
    for stab in &mut state.stabilizers {
        stab.project(num_qubits - 1, false).unwrap();
        stab.discard(num_qubits - 1).unwrap();
    }
    state.num_qubits -= 1;
}

pub(crate) fn construct_cat_state(
    num_qubits: usize,
) -> StabilizerDecomposedState<Scalar> {
    match num_qubits {
        1 => _construct_cat_1_state(),
        2 => _construct_cat_2_state(),
        4 => construct_cat_4_state(),
        5 => {
            let mut state = construct_cat_6_state();
            _reduce_cat_state(&mut state);
            state
        }
        6 => construct_cat_6_state(),
        _ => {
            let mut cat_pair = construct_cat_state(num_qubits - 4).kron(
                &construct_cat_state(6)
            );
            _project_stab_decomp_state_onto_cat_state(&mut cat_pair, &[num_qubits - 5, num_qubits - 4]);
            cat_pair
        }
    }
}