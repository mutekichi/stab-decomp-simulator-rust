use crate::circuit::{CliffordCircuit, CliffordGate};
use ndarray::{Array1, Array2};
use rand::{Rng, SeedableRng};

/// Samples H (Hadamard layer) and S (Permutation layer) from the quantum Mallows distribution.
/// This implementation is based on Algorithm 1 and Lemma 6 from arXiv:2003.09412v2.
fn sample_quantum_mallows<R: Rng>(n: usize, rng: &mut R) -> (Array1<u8>, Array1<usize>) {
    let mut h = Array1::zeros(n);
    let mut s_perm = Array1::from_elem(n, 0);
    let mut available_indices: Vec<usize> = (0..n).collect();

    for i in 0..n {
        let m = n - i;
        if m == 0 {
            continue;
        }

        let r: f64 = rng.r#gen(); // A random number in [0, 1)

        // Inverse transform sampling for the distribution P(a) ~ 2^(-a).
        let val_to_log = r * (4.0f64.powi(m as i32) - 1.0) + 1.0;
        let ceil_log = val_to_log.log2().ceil();

        // `a` is sampled from [1, 2m] according to the distribution.
        let a = (2 * m + 1) as isize - ceil_log as isize;
        let a = if a <= 0 { 1 } else { a as usize }; // Handle edge case where r=1.0

        // Decode 'a' into h_i and the permutation choice 'k' (1-based).
        let k_1_based = if a <= m {
            h[i] = 1;
            a
        } else {
            h[i] = 0;
            2 * m - a + 1
        };

        let k_0_based = k_1_based - 1;

        if k_0_based < available_indices.len() {
            s_perm[i] = available_indices.remove(k_0_based);
        } else {
            // This safeguard handles potential floating point inaccuracies at the boundaries.
            s_perm[i] = available_indices.pop().unwrap();
        }
    }
    (h, s_perm)
}

/// A struct to hold all parameters defining a Clifford operator in its canonical form.
struct CliffordParams {
    h: Array1<u8>,
    s: Array1<usize>,
    pauli2_z: Array1<u8>,
    pauli2_x: Array1<u8>,
    gamma1: Array2<u8>,
    delta1: Array2<u8>,
    gamma2: Array2<u8>,
    delta2: Array2<u8>,
}

/// Generates the parameter matrices for the Hadamard-free layers F1 and F2.
/// This implementation follows Algorithm 2 and the rules C1-C5 from Theorem 1 in
/// arXiv:2003.09412v2.
fn generate_clifford_params<R: Rng>(n: usize, rng: &mut R) -> CliffordParams {
    let (h, s) = sample_quantum_mallows(n, rng);

    let mut gamma1 = Array2::zeros((n, n));
    let mut delta1 = Array2::eye(n);
    let mut gamma2 = Array2::zeros((n, n));
    let mut delta2 = Array2::eye(n);

    let pauli2_z = Array1::from_shape_fn(n, |_| rng.gen_range(0..=1));
    let pauli2_x = Array1::from_shape_fn(n, |_| rng.gen_range(0..=1));

    // Fill diagonal elements (P gates).
    for i in 0..n {
        gamma2[[i, i]] = rng.gen_range(0..=1);
        if h[i] == 1 {
            gamma1[[i, i]] = rng.gen_range(0..=1);
        }
    }

    // Fill off-diagonal elements (CZ and CNOT gates) for i > j.
    for i in 1..n {
        for j in 0..i {
            // F2 matrices are always filled randomly.
            let b_gamma2 = rng.gen_range(0..=1);
            gamma2[[i, j]] = b_gamma2;
            gamma2[[j, i]] = b_gamma2;
            delta2[[i, j]] = rng.gen_range(0..=1);

            // F1 matrices are filled based on rules C1-C5 from the paper.
            let (h_i, h_j) = (h[i] == 1, h[j] == 1);
            let (s_i, s_j) = (s[i], s[j]);

            // Gamma1 (CZ gates) based on C1 & C2
            if (s_i < s_j || h_j) && h_i || h_j && s_j < s_i {
                let b = rng.gen_range(0..=1);
                gamma1[[i, j]] = b;
                gamma1[[j, i]] = b;
            }

            // Delta1 (CNOT gates) based on C3, C4, C5
            if (s_i < s_j || h_j) && (s_i > s_j || !h_i) && (h_j || !h_i) {
                delta1[[i, j]] = rng.gen_range(0..=1);
            }
        }
    }

    CliffordParams {
        h,
        s,
        pauli2_z,
        pauli2_x,
        gamma1,
        delta1,
        gamma2,
        delta2,
    }
}

/// Applies a Hadamard-free layer (F) to the quantum circuit.
fn apply_hadamard_free_layer(
    qc: &mut CliffordCircuit,
    n: usize,
    gamma: &Array2<u8>,
    delta: &Array2<u8>,
    pauli_z: Option<&Array1<u8>>,
    pauli_x: Option<&Array1<u8>>,
) {
    // Apply gates in reverse order of the canonical form: CNOT -> CZ/S -> Pauli.

    // CNOT layer from Delta matrix.
    for j in 0..n {
        for i in (j + 1)..n {
            if delta[[i, j]] == 1 {
                qc.add_gate(CliffordGate::CX(j, i));
            }
        }
    }

    // CZ and S (Phase) layer from Gamma matrix.
    for i in 0..n {
        if gamma[[i, i]] == 1 {
            qc.add_gate(CliffordGate::S(i));
        }
        for j in 0..i {
            if gamma[[i, j]] == 1 {
                qc.add_gate(CliffordGate::CZ(i, j));
            }
        }
    }

    // Pauli layer (only for F2).
    if let (Some(z), Some(x)) = (pauli_z, pauli_x) {
        for i in 0..n {
            if z[i] == 1 && x[i] == 1 {
                qc.add_gate(CliffordGate::Y(i));
            } else if z[i] == 1 {
                qc.add_gate(CliffordGate::Z(i));
            } else if x[i] == 1 {
                qc.add_gate(CliffordGate::X(i));
            }
        }
    }
}

/// Applies a permutation layer (S) using a sequence of SWAP gates.
fn apply_permutation_layer(qc: &mut CliffordCircuit, s_perm: &Array1<usize>) {
    let n = s_perm.len();
    let mut p: Vec<usize> = (0..n).collect();

    for i in 0..n {
        let target_pos = s_perm[i];
        if p[i] != target_pos {
            // Find the current location of the qubit that should be at `i`.
            let j = p.iter().position(|&x| x == target_pos).unwrap();
            qc.add_gate(CliffordGate::Swap(i, j));
            p.swap(i, j);
        }
    }
}

/// Generates a uniformly random Clifford circuit on `n` qubits.
///
/// ## Reference
/// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford
///   group," IEEE Trans. Inf. Theory 67, 5800 (2021). https://doi.org/10.1109/TIT.2021.3081415
pub(crate) fn random_clifford(n: usize, seed: Option<[u8; 32]>) -> CliffordCircuit {
    if n == 0 {
        return CliffordCircuit::new(0);
    }
    let mut rng = match seed {
        Some(s) => rand::rngs::StdRng::from_seed(s),
        None => rand::rngs::StdRng::from_entropy(),
    };
    let params = generate_clifford_params(n, &mut rng);
    let mut qc = CliffordCircuit::new(n);

    // Build the circuit U = F1 * H * S * F2 by applying gates in reverse order.
    apply_hadamard_free_layer(
        &mut qc,
        n,
        &params.gamma2,
        &params.delta2,
        Some(&params.pauli2_z),
        Some(&params.pauli2_x),
    );
    apply_permutation_layer(&mut qc, &params.s);
    for i in 0..n {
        if params.h[i] == 1 {
            qc.add_gate(CliffordGate::H(i));
        }
    }
    apply_hadamard_free_layer(&mut qc, n, &params.gamma1, &params.delta1, None, None);

    qc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_clifford_generation() {
        let num_qubits = 3;
        let circuit = random_clifford(num_qubits, Some([42; 32]));
        assert_eq!(circuit.num_qubits, num_qubits);
        assert!(circuit.gates.len() > 0);
    }

    #[test]
    fn test_random_clifford_determinism() {
        let num_qubits = 4;
        let seed = [123; 32];
        let circuit1 = random_clifford(num_qubits, Some(seed));
        let circuit2 = random_clifford(num_qubits, Some(seed));
        assert_eq!(circuit1.gates, circuit2.gates);
    }

    #[test]
    fn test_random_clifford_validity() {
        let num_qubits = 4;
        let circuit = random_clifford(num_qubits, Some([56; 32]));
        for gate in circuit.gates {
            match gate {
                CliffordGate::H(q)
                | CliffordGate::X(q)
                | CliffordGate::Y(q)
                | CliffordGate::Z(q)
                | CliffordGate::S(q)
                | CliffordGate::Sdg(q)
                | CliffordGate::SqrtX(q)
                | CliffordGate::SqrtXdg(q) => {
                    assert!(q < num_qubits);
                }
                CliffordGate::CX(c, t) | CliffordGate::CZ(c, t) | CliffordGate::Swap(c, t) => {
                    assert!(c < num_qubits && t < num_qubits && c != t);
                }
            }
        }
    }

    // The uniformity of the distribution is tested in `tests` directory.
}

// DONE
