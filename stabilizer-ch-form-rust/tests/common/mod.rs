use ndarray::Array1;
use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::{CliffordCircuit, CliffordGate};
use std::f64::consts::FRAC_1_SQRT_2;

#[allow(dead_code)]
/// Asserts that two complex numbers are approximately equal using both
/// relative and absolute tolerances.
pub fn assert_eq_complex_robust(a: Complex64, b: Complex64) {
    let diff = (a - b).norm();
    let max_abs = a.norm().max(b.norm());

    // Common default tolerances
    let abs_tol = 1e-8f64;
    let rel_tol = 1e-5f64;

    let tolerance = abs_tol.max(rel_tol * max_abs);

    assert!(
        diff <= tolerance,
        "Complex numbers differ significantly:\n  left: {}\n right: {}\n  diff: {}\n   tol: {}",
        a,
        b,
        diff,
        tolerance
    );
}

#[allow(dead_code)]
/// Asserts that two Array1<Complex64> are approximately equal element-wise
/// using both relative and absolute tolerances.
pub fn assert_eq_complex_array1(a: &Array1<Complex64>, b: &Array1<Complex64>) {
    assert_eq!(
        a.len(),
        b.len(),
        "Arrays have different lengths: {} vs {}",
        a.len(),
        b.len()
    );
    for (i, (val_a, val_b)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (val_a - val_b).norm();
        let max_abs = val_a.norm().max(val_b.norm());

        // Common default tolerances
        let abs_tol = 1e-8f64;
        let rel_tol = 1e-5f64;

        let tolerance = abs_tol.max(rel_tol * max_abs);

        assert!(
            diff <= tolerance,
            "Arrays differ at index {}: \n  left: {}\n right: {}\n  diff: {}\n   tol: {}",
            i,
            val_a,
            val_b,
            diff,
            tolerance
        );
    }
}

// --- Naive StateVector Simulator for Reference ---

type Matrix2x2 = [[Complex64; 2]; 2];

const X_MATRIX: Matrix2x2 = [
    [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
];
const Y_MATRIX: Matrix2x2 = [
    [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
    [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
];
const Z_MATRIX: Matrix2x2 = [
    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
];
const H_MATRIX: Matrix2x2 = [
    [
        Complex64::new(FRAC_1_SQRT_2, 0.0),
        Complex64::new(FRAC_1_SQRT_2, 0.0),
    ],
    [
        Complex64::new(FRAC_1_SQRT_2, 0.0),
        Complex64::new(-FRAC_1_SQRT_2, 0.0),
    ],
];
const S_MATRIX: Matrix2x2 = [
    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)],
];
const SDG_MATRIX: Matrix2x2 = [
    [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
    [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
];
const SQRT_X_MATRIX: Matrix2x2 = [
    [Complex64::new(0.5, 0.5), Complex64::new(0.5, -0.5)],
    [Complex64::new(0.5, -0.5), Complex64::new(0.5, 0.5)],
];
const SQRT_XDG_MATRIX: Matrix2x2 = [
    [Complex64::new(0.5, -0.5), Complex64::new(0.5, 0.5)],
    [Complex64::new(0.5, 0.5), Complex64::new(0.5, -0.5)],
];

#[allow(dead_code)]
/// Apply a single-qubit gate represented by a 2x2 matrix to the target qubit
/// in the given statevector.
fn apply_single_qubit_gate(sv: &mut Array1<Complex64>, target_qubit: usize, matrix: &Matrix2x2) {
    let n_qubits = (sv.len() as f64).log2() as usize;
    let dim = 1 << n_qubits;
    let mut sv_copy = sv.clone();

    let m00 = matrix[0][0];
    let m01 = matrix[0][1];
    let m10 = matrix[1][0];
    let m11 = matrix[1][1];

    for i in 0..dim {
        if (i >> target_qubit) & 1 == 0 {
            let j = i | (1 << target_qubit);

            let sv_i = sv[i];
            let sv_j = sv[j];

            sv_copy[i] = m00 * sv_i + m01 * sv_j;
            sv_copy[j] = m10 * sv_i + m11 * sv_j;
        }
    }
    *sv = sv_copy;
}

#[allow(dead_code)]
fn apply_cx(sv: &mut Array1<Complex64>, control: usize, target: usize) {
    let n_qubits = (sv.len() as f64).log2() as usize;
    let dim = 1 << n_qubits;

    for i in 0..dim {
        if ((i >> control) & 1 == 1) && ((i >> target) & 1 == 0) {
            let j = i | (1 << target);
            sv.swap(i, j);
        }
    }
}

#[allow(dead_code)]
fn apply_cz(sv: &mut Array1<Complex64>, q1: usize, q2: usize) {
    let n_qubits = (sv.len() as f64).log2() as usize;
    let dim = 1 << n_qubits;

    for i in 0..dim {
        if ((i >> q1) & 1 == 1) && ((i >> q2) & 1 == 1) {
            sv[i] *= -1.0;
        }
    }
}

#[allow(dead_code)]
fn apply_swap(sv: &mut Array1<Complex64>, q1: usize, q2: usize) {
    let n_qubits = (sv.len() as f64).log2() as usize;
    let dim = 1 << n_qubits;

    for i in 0..dim {
        let bit1 = (i >> q1) & 1;
        let bit2 = (i >> q2) & 1;

        if bit1 != bit2 {
            let j = i ^ (1 << q1) ^ (1 << q2);
            if i < j {
                sv.swap(i, j);
            }
        }
    }
}

#[allow(dead_code)]
/// Simulate the given `CliffordCircuit` U and return the resulting reference
/// statevector: |ψ⟩ = U |0...0⟩, using a naive statevector simulator.
pub fn get_reference_statevector(circuit: &CliffordCircuit) -> Array1<Complex64> {
    let n = circuit.n_qubits;
    let dim = 1 << n;
    let mut sv = Array1::<Complex64>::zeros(dim);
    sv[0] = Complex64::ONE;

    for gate in &circuit.gates {
        match gate {
            CliffordGate::H(q) => apply_single_qubit_gate(&mut sv, *q, &H_MATRIX),
            CliffordGate::X(q) => apply_single_qubit_gate(&mut sv, *q, &X_MATRIX),
            CliffordGate::Y(q) => apply_single_qubit_gate(&mut sv, *q, &Y_MATRIX),
            CliffordGate::Z(q) => apply_single_qubit_gate(&mut sv, *q, &Z_MATRIX),
            CliffordGate::S(q) => apply_single_qubit_gate(&mut sv, *q, &S_MATRIX),
            CliffordGate::Sdg(q) => apply_single_qubit_gate(&mut sv, *q, &SDG_MATRIX),
            CliffordGate::SqrtX(q) => apply_single_qubit_gate(&mut sv, *q, &SQRT_X_MATRIX),
            CliffordGate::SqrtXdg(q) => apply_single_qubit_gate(&mut sv, *q, &SQRT_XDG_MATRIX),
            CliffordGate::CX(c, t) => apply_cx(&mut sv, *c, *t),
            CliffordGate::CZ(q1, q2) => apply_cz(&mut sv, *q1, *q2),
            CliffordGate::Swap(q1, q2) => apply_swap(&mut sv, *q1, *q2),
        }
    }
    sv
}
