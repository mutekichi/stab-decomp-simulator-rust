use stabilizer_ch_form_rust::StabilizerCHForm;

use crate::error::Result;
use crate::state::magic_states::cat_state::construct_cat_state;
use crate::state::{
    StabilizerDecomposedState,
    types::{phase_factor::PhaseFactor, scalar::Scalar},
};

/// Apply X then S on the target qubit
fn apply_xs(state: &mut StabilizerCHForm, target: usize) -> Result<()> {
    state.apply_x(target)?;
    state.apply_s(target)?;
    Ok(())
}

/// Create a T-tensor state: $|T\rangle^{\otimes n}$ as a stabilizer decomposed state.
/// The decomposition is based on the work by Qassim et al. (2021).
///
/// ## Arguments
/// * `num_qubits` - The number of T states ($n$) to include in the tensor product.
///
/// ## Returns
/// A [`Result`] containing the resulting [`StabilizerDecomposedState`].
///
/// ## Reference
/// - H. Qassim, et al., "Improved upper bounds on the stabilizer rank of magic states," Quantum 5,
///   604 (2021). https://doi.org/10.22331/q-2021-12-20-606
pub(crate) fn construct_t_tensor_state(
    num_qubits: usize,
) -> Result<StabilizerDecomposedState<Scalar>> {
    let cat_state = construct_cat_state(num_qubits)?;

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
            apply_xs(&mut new_stab, 0).unwrap();
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

    Ok(StabilizerDecomposedState::new(
        cat_state.num_qubits,
        new_stabs,
        new_coeffs,
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        state::magic_states::t_state::construct_t_tensor_state,
        test_utils::assert_eq_complex_array1,
    };
    use ndarray::Array1;
    use num_complex::Complex64;

    // Define t_state_vector_1 T|+> = (|0> + e^{iπ/4}|1>)/√2
    fn construct_t_state_vector_1() -> Array1<Complex64> {
        let mut state = Array1::<Complex64>::zeros(2);
        state[0] = Complex64::new(1.0 / 2f64.sqrt(), 0.0);
        state[1] = Complex64::new(1.0 / 2f64, 1.0 / 2f64);
        state
    }

    fn kron_complex_vectors(a: &Array1<Complex64>, b: &Array1<Complex64>) -> Array1<Complex64> {
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

    fn construct_t_tensor_vector(num_qubits: usize) -> Array1<Complex64> {
        match num_qubits {
            0 => panic!("Number of T states must be at least 1"),
            1 => construct_t_state_vector_1(),
            _ => kron_complex_vectors(
                &construct_t_tensor_vector(num_qubits - 1),
                &construct_t_state_vector_1(),
            ),
        }
    }

    #[test]
    fn test_construct_t_tensor_state() {
        for num_qubits in 1..=9 {
            let state = construct_t_tensor_state(num_qubits).unwrap();
            let expected_vector = construct_t_tensor_vector(num_qubits);
            let state_vector = state.to_statevector().unwrap();
            assert_eq_complex_array1(&expected_vector, &state_vector);
            println!("Test passed for {} qubits.", num_qubits);
        }
    }
}
