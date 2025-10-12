use ndarray::{Array1, Array2};
use num_complex::Complex64;

mod internal;
use crate::{
    api::representation::{CliffordCircuit, CliffordGate},
    error::ChFormError,
};
use internal::types::phase_factor::PhaseFactor;

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

impl StabilizerCHForm {
    pub fn new(n: usize) -> Result<Self, ChFormError> {
        if n == 0 {
            return Err(ChFormError::InvalidNumQubits(n));
        }

        Ok(Self {
            n: n,
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

    pub fn n_qubits(&self) -> usize {
        self.n
    }

    pub fn set_global_phase(&mut self, phase: Complex64) {
        if (phase.norm_sqr() - 1.0).abs() > 1e-8 {
            panic!("Global phase must be a unit complex number.");
        }
        self.omega = phase;
    }

    pub fn global_phase(&self) -> Complex64 {
        self.omega
    }

    pub fn from_clifford_circuit(circuit: &CliffordCircuit) -> Result<Self, ChFormError> {
        let mut ch_form = StabilizerCHForm::new(circuit.n_qubits)?;

        for gate in &circuit.gates {
            match gate {
                CliffordGate::H(q) => ch_form._left_multiply_h(*q)?,
                CliffordGate::S(q) => ch_form._left_multiply_s(*q)?,
                CliffordGate::Sdg(q) => ch_form._left_multiply_sdg(*q)?,
                CliffordGate::X(q) => ch_form._left_multiply_x(*q)?,
                CliffordGate::Y(q) => ch_form._left_multiply_y(*q)?,
                CliffordGate::Z(q) => ch_form._left_multiply_z(*q)?,
                CliffordGate::SqrtX(q) => ch_form._left_multiply_sqrt_x(*q)?,
                CliffordGate::SqrtXdg(q) => ch_form._left_multiply_sqrt_xdg(*q)?,
                CliffordGate::CX(control, target) => {
                    ch_form._left_multiply_cx(*control, *target)?
                }
                CliffordGate::CZ(control, target) => {
                    ch_form._left_multiply_cz(*control, *target)?
                }
                CliffordGate::Swap(q1, q2) => ch_form._left_multiply_swap(*q1, *q2)?,
            }
        }
        Ok(ch_form)
    }
}
