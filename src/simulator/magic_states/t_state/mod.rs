use stabilizer_ch_form_rust::{StabilizerCHForm, api::*};

use crate::simulator::{types::{phase_factor::PhaseFactor, scalar::Scalar}, StabilizerDecomposedState};

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
                r: 0,
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

#[cfg(test)]
mod tests {
    use crate::{simulator::magic_states::t_state::_construct_t_tensor_state, test_utils::assert_eq_complex_array1};
    use ndarray::Array1;
    use num_complex::Complex64;

    // Define t_state_vector_1 T|+> = (|0> + e^{iπ/4}|1>)/√2
    fn _construct_t_state_vector_1() -> Array1<Complex64> {
        let mut state = Array1::<Complex64>::zeros(2);
        state[0] = Complex64::new(1.0 / 2f64.sqrt(), 0.0);
        state[1] = Complex64::new(1.0 / 2f64, 1.0 / 2f64);
        state
    }

    fn _kron_complex_vectors(a: &Array1<Complex64>, b: &Array1<Complex64>) -> Array1<Complex64> {
        let len_a = a.len();
        let len_b = b.len();
        let mut result = Array1::<Complex64>::zeros(len_a * len_b);
        for i in 0..len_a {
            for j in 0..len_b {
                result[i * len_b + j] = a[i] * b[j];
            }
        }
        result
    }

    fn _construct_t_tensor_vector(num_qubits: usize) -> Array1<Complex64> {
        match num_qubits {
            0 => panic!("Number of T states must be at least 1"),
            1 => _construct_t_state_vector_1(),
            _ => _kron_complex_vectors(
                &_construct_t_tensor_vector(num_qubits - 1),
                &_construct_t_state_vector_1(),
            ),
        }
    }

    #[test]
    fn test_construct_t_tensor_state() {
        for num_qubits in 1..=9 {
            let state = _construct_t_tensor_state(num_qubits);
            let expected_vector = _construct_t_tensor_vector(num_qubits);
            let state_vector = state._to_statevector();
            assert_eq_complex_array1(&expected_vector, &state_vector);
            println!("Test passed for {} qubits.", num_qubits);
        }
    }
}
