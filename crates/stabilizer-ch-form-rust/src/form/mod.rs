use ndarray::{Array1, Array2};
use num_complex::Complex64;

use crate::{
    circuit::{CliffordCircuit, CliffordGate},
    error::{Error, Result},
};

use types::PhaseFactor;

#[derive(Debug, Clone)]
pub struct StabilizerCHForm {
    pub(crate) n: usize,
    pub(crate) mat_g: Array2<bool>,
    pub(crate) mat_f: Array2<bool>,
    pub(crate) mat_m: Array2<bool>,
    pub(crate) gamma: Array1<PhaseFactor>,
    pub(crate) vec_v: Array1<bool>,
    pub(crate) vec_s: Array1<bool>,
    pub(crate) omega: Complex64,
    pub(crate) phase_factor: PhaseFactor,
}

mod amplitude;
mod discard;
mod gate_application;
mod get_qubit_state;
mod inner_product;
mod kron;
mod left_multiplication;
mod matrix_operations;
mod measure;
mod permute;
mod project;
mod resolve_superposition;
mod right_multiplication;
mod statevector;
mod types;

impl StabilizerCHForm {
    /// Creates a new StabilizerCHForm representing the |0...0> state for `n` qubits.
    /// ## Arguments
    /// * `n` - The number of qubits.
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return Err(Error::InvalidNumQubits(n));
        }

        Ok(Self {
            n,
            // Initialize G, F as identity matrices, M as zero matrix
            mat_g: Array2::from_shape_fn((n, n), |(i, j)| i == j),
            mat_f: Array2::from_shape_fn((n, n), |(i, j)| i == j),
            mat_m: Array2::from_elem((n, n), false),
            // Initialize gamma as [+1, +1, ..., +1]
            gamma: Array1::from_elem(n, PhaseFactor::PLUS_ONE),
            // Initialize v, s as zero vectors
            vec_v: Array1::from_elem(n, false),
            vec_s: Array1::from_elem(n, false),
            // Initialize omega as 1 + 0i
            omega: Complex64::new(1.0, 0.0),
            // Initialize overall phase factor as +1
            phase_factor: PhaseFactor::PLUS_ONE,
        })
    }

    /// Returns the number of qubits in the stabilizer state.
    pub fn num_qubits(&self) -> usize {
        self.n
    }

    /// Sets the global phase of the stabilizer state.
    ///
    /// ## Arguments
    /// * `phase` - A unit complex number representing the desired global phase.
    pub fn set_global_phase(&mut self, phase: Complex64) {
        if (phase.norm_sqr() - 1.0).abs() > 1e-8 {
            panic!("Global phase must be a unit complex number.");
        }
        self.omega = phase;
    }

    /// Returns the global phase of the stabilizer state.
    ///
    /// ## Returns
    /// A unit complex number representing the global phase.
    pub fn global_phase(&self) -> Complex64 {
        self.omega
    }

    /// Constructs a [`StabilizerCHForm`] from a [`CliffordCircuit`].
    ///
    /// ## Arguments
    /// * `circuit` - The [`CliffordCircuit`] to convert.
    ///
    /// ## Returns
    /// A [`Result`] containing the resulting [`StabilizerCHForm`].
    pub fn from_clifford_circuit(circuit: &CliffordCircuit) -> Result<Self> {
        let mut ch_form = StabilizerCHForm::new(circuit.num_qubits)?;

        for gate in &circuit.gates {
            match gate {
                CliffordGate::H(q) => ch_form.left_multiply_h(*q)?,
                CliffordGate::S(q) => ch_form.left_multiply_s(*q)?,
                CliffordGate::Sdg(q) => ch_form.left_multiply_sdg(*q)?,
                CliffordGate::X(q) => ch_form.left_multiply_x(*q)?,
                CliffordGate::Y(q) => ch_form.left_multiply_y(*q)?,
                CliffordGate::Z(q) => ch_form.left_multiply_z(*q)?,
                CliffordGate::SqrtX(q) => ch_form.left_multiply_sqrt_x(*q)?,
                CliffordGate::SqrtXdg(q) => ch_form.left_multiply_sqrt_xdg(*q)?,
                CliffordGate::CX(control, target) => ch_form.left_multiply_cx(*control, *target)?,
                CliffordGate::CZ(control, target) => ch_form.left_multiply_cz(*control, *target)?,
                CliffordGate::Swap(q1, q2) => ch_form.left_multiply_swap(*q1, *q2)?,
            }
        }
        Ok(ch_form)
    }
}
