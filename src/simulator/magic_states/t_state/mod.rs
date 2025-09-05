use stabilizer_ch_form_rust::{StabilizerCHForm, api::*};

use crate::prelude::{
    StabilizerDecomposedState,
    types::{phase_factor::PhaseFactor, scalar::Scalar},
};
mod cat_state;

/// Apply X then S on the target qubit
fn _apply_xs(state: &mut StabilizerCHForm, target: usize) {
    state.apply_x(target);
    state.apply_s(target);
}

pub(crate) fn _construct_t_tensor_state(num_qubits: usize) -> StabilizerDecomposedState<Scalar> {
    let cat_state = cat_state::_construct_cat_state(num_qubits);

    let new_stabs_original = cat_state.stabilizers.clone();
    let new_coeffs_original = cat_state
        .coefficients
        .iter()
        .copied()
        .map(|c| c * Scalar::ONE_OVER_SQRT_2)
        .collect::<Vec<_>>();

    let new_stabs_to_append = new_stabs_original
        .iter()
        .map(|stab| {
            let mut new_stab = stab.clone();
            _apply_xs(&mut new_stab, 0);
            new_stab
        })
        .collect::<Vec<_>>();
    let new_coeffs_to_append = new_coeffs_original
        .iter()
        .copied()
        .map(|c| {
            c * Scalar::NonZero {
                phase: PhaseFactor::EXP_I_7PI_4,
                r: 1,
            }
        })
        .collect::<Vec<_>>();

    let mut new_stabs = new_stabs_original;
    new_stabs.extend(new_stabs_to_append);
    let mut new_coeffs = new_coeffs_original;
    new_coeffs.extend(new_coeffs_to_append);

    StabilizerDecomposedState {
        num_qubits: cat_state.num_qubits,
        stabilizers: new_stabs,
        coefficients: new_coeffs,
    }
}
