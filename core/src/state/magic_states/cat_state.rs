//! Module for constructing "cat states" as [`StabilizerDecomposedState`]s.
//! These implementations are based on the construction described in reference:
//! - H. Qassim, et al., "Improved upper bounds on the stabilizer rank of magic states,"
//!   Quantum 5, 604 (2021). <https://doi.org/10.22331/q-2021-12-20-606>

use stabilizer_ch_form_rust::prelude::*;

use crate::state::{
    StabilizerDecomposedState,
    types::{phase_factor::PhaseFactor, scalar::Scalar},
};

use crate::error::{Error, Result};

/// Returns (|0^n> - i|1^n>) / sqrt(2) as a StabilizerCHForm
fn zero_minus_i_one_state(num_qubits: usize) -> Result<StabilizerCHForm> {
    if num_qubits == 0 {
        return Err(crate::error::Error::InvalidNumQubits(num_qubits));
    }

    let mut state = StabilizerCHForm::new(num_qubits)?;
    state.apply_h(0)?;
    for i in 1..num_qubits {
        state.apply_cx(0, i)?;
    }
    state.apply_sdg(0)?;

    Ok(state)
}

/// Returns |E_m> = (Σ_{x: x is even} |x>) / sqrt(2^(n-1)) as a StabilizerCHForm
fn even_parity_state(num_qubits: usize) -> Result<StabilizerCHForm> {
    let mut state = StabilizerCHForm::new(num_qubits)?;
    for i in 0..num_qubits - 1 {
        state.apply_h(i)?;
    }
    for i in 0..num_qubits - 1 {
        state.apply_cx(i, num_qubits - 1)?;
    }

    Ok(state)
}

/// Returns |K_m> = Π_{1<=i<j<=m} CZ_{i,j} |E_m> as a StabilizerCHForm
fn even_parity_phase_flipped_state(num_qubits: usize) -> Result<StabilizerCHForm> {
    let mut state = even_parity_state(num_qubits)?;
    for i in 0..num_qubits {
        for j in (i + 1)..num_qubits {
            state.apply_cz(i, j)?;
        }
    }
    Ok(state)
}

/// Construct |cat_1> = |0> state as a StabilizerDecomposedState
fn construct_cat_1_state() -> Result<StabilizerDecomposedState<Scalar>> {
    let stab = StabilizerCHForm::new(1)?;
    let coeffs = vec![Scalar::ONE];

    Ok(StabilizerDecomposedState::new(1, vec![stab], coeffs))
}

/// Construct |cat_2> = (|00> + i|11>) / sqrt(2) state as a StabilizerDecomposedState
fn construct_cat_2_state() -> Result<StabilizerDecomposedState<Scalar>> {
    let mut stab = StabilizerCHForm::new(2)?;
    stab.apply_h(0)?;
    stab.apply_cx(0, 1)?;
    stab.apply_s(1)?;
    let coeffs = vec![Scalar::ONE];

    Ok(StabilizerDecomposedState::new(2, vec![stab], coeffs))
}

/// Construct |cat_4> state as a superposition of 2 StabilizerCHForm states
fn construct_cat_4_state() -> Result<StabilizerDecomposedState<Scalar>> {
    let stab1 = zero_minus_i_one_state(4)?;
    let stab2 = even_parity_state(4)?;
    let coeffs = vec![
        Scalar::NonZero {
            phase: PhaseFactor::EXP_I_7PI_4,
            r: 1,
        },
        Scalar::NonZero {
            phase: PhaseFactor::PLUS_I,
            r: 0,
        },
    ];

    Ok(StabilizerDecomposedState::new(
        4,
        vec![stab1, stab2],
        coeffs,
    ))
}

/// Construct |cat_6> state as a superposition of 3 StabilizeCHForm states
fn construct_cat_6_state() -> Result<StabilizerDecomposedState<Scalar>> {
    let stab1 = zero_minus_i_one_state(6)?;
    let stab2 = even_parity_state(6)?;
    let stab3 = even_parity_phase_flipped_state(6)?;
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

    Ok(StabilizerDecomposedState::new(
        6,
        vec![stab1, stab2, stab3],
        coeffs,
    ))
}

fn project_ch_form_onto_cat_state(state: &mut StabilizerCHForm, qubits: &[usize]) -> Result<()> {
    // Make sure qubits has length 2 and qubits[0] < qubits[1]
    // We do not check this here for performance reasons
    state.apply_sdg(qubits[0])?;
    state.apply_cx(qubits[0], qubits[1])?;
    state.apply_h(qubits[0])?;
    state.project(qubits[0], false)?;
    state.project(qubits[1], false)?;
    state.discard(qubits[1])?;
    state.discard(qubits[0])?;

    Ok(())
}

fn project_stab_decomp_state_onto_cat_state(
    state: &mut StabilizerDecomposedState<Scalar>,
    qubits: &[usize],
) -> Result<()> {
    for stab in &mut state.stabilizers {
        project_ch_form_onto_cat_state(stab, qubits)?;
    }
    state.num_qubits -= 2;

    Ok(())
}

/// Make |cat_{m-1}> from |cat_m> by tracing out the last qubits
fn reduce_cat_state(state: &mut StabilizerDecomposedState<Scalar>) -> Result<()> {
    let num_qubits = state.num_qubits;
    for stab in &mut state.stabilizers {
        stab.project(num_qubits - 1, false)?;
        stab.discard(num_qubits - 1)?;
    }
    state.num_qubits -= 1;

    Ok(())
}

/// Constructs the "cat state" |cat_n> as a [`StabilizerDecomposedState`]
pub(crate) fn construct_cat_state(num_qubits: usize) -> Result<StabilizerDecomposedState<Scalar>> {
    match num_qubits {
        0 => Err(Error::InvalidNumQubits(num_qubits)),
        1 => construct_cat_1_state(),
        2 => construct_cat_2_state(),
        3 => {
            let mut state = construct_cat_4_state()?;
            reduce_cat_state(&mut state)?;
            Ok(state)
        }
        4 => construct_cat_4_state(),
        5 => {
            let mut state = construct_cat_6_state()?;
            reduce_cat_state(&mut state)?;
            Ok(state)
        }
        6 => construct_cat_6_state(),
        _ => {
            let mut cat_pair =
                construct_cat_state(num_qubits - 4)?.kron(&construct_cat_state(6)?)?;
            project_stab_decomp_state_onto_cat_state(
                &mut cat_pair,
                &[num_qubits - 5, num_qubits - 4],
            )?;
            Ok(cat_pair)
        }
    }
}
