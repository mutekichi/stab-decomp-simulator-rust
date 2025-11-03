use ndarray::{Array1, Array2};
use num_complex::Complex64;

use crate::{
    circuit::{CliffordCircuit, CliffordGate},
    error::{Error, Result},
};

mod types;
use crate::types::pauli::PauliString;
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
mod measure;
mod permute;
mod project;
mod resolve_superposition;
mod right_multiplication;
mod statevector;

impl StabilizerCHForm {
    /// Computes the tensor product of this state with another.
    ///
    /// Returns: |self> ⊗ |other>
    pub fn kron(&self, other: &StabilizerCHForm) -> Result<StabilizerCHForm> {
        self._kron(other)
    }

    /// Discards the specified qubit from the state.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state.
    ///
    /// # Errors
    //// Returns an `ChFormError` if the qubit index is out of bounds. Note that
    /// this function does not check if the qubit is properly projected onto |0>.
    pub fn discard(&mut self, qarg: usize) -> Result<()> {
        self._discard(qarg)
    }

    /// Returns a new StabilizerCHForm with the qubits permuted.
    ///
    /// # Arguments
    ///
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    pub fn permuted(&self, axes: &[usize]) -> Result<Self> {
        self._permuted(axes)
    }

    /// Permutes the qubits of the state in-place.
    ///
    /// # Arguments
    ///
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    ///
    /// # Panics
    ///
    /// Panics if the length of `axes` is not equal to the number of qubits.
    pub fn permute(&mut self, axes: &[usize]) -> Result<()> {
        self._permute(axes)
    }

    /// Returns a new StabilizerCHForm with the specified qubit discarded.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state.
    pub fn discarded(&self, qarg: usize) -> Result<StabilizerCHForm> {
        let mut self_clone = self.clone();
        self_clone.discard(qarg)?;
        Ok(self_clone)
    }

    /// Projects a qubit onto a computational basis state (`|0>` or `|1>`).
    ///
    /// This operation modifies the stabilizer state in place.
    ///
    /// In a stabilizer state, measuring a qubit in the computational basis yields either a
    /// deterministic outcome (`|0>` or `|1>`) or a perfectly random one (50% probability for each).
    /// This function attempts to force the qubit into the specified `outcome`, succeeding if the projection
    /// is physically possible.
    ///
    /// # Arguments
    ///
    /// * `qarg`: The index of the qubit to project.
    /// * `outcome`: The desired basis state to project onto (`false` for `|0>`, `true` for `|1>`).
    ///
    /// # Returns
    ///
    /// A `Result` indicating the outcome of the projection:
    ///
    /// * `Ok(true)` if the projection was **deterministic**. This means the qubit was already
    ///   in the desired state. The stabilizer state is unchanged.
    /// * `Ok(false)` if the projection was **non-deterministic** (probabilistic). This means the
    ///   qubit was in a superposition and has now been collapsed to the desired state. The
    ///   stabilizer state has been updated.
    ///
    /// # Errors
    ///
    /// Returns an `ChFormError` if the projection is impossible. This occurs when the qubit has a
    /// deterministic value that is orthogonal to the desired `outcome` (e.g., attempting to
    /// project a qubit in state `|0>` onto `|1>`).
    pub fn project(&mut self, qarg: usize, outcome: bool) -> Result<bool> {
        self._project(qarg, outcome)
    }

    /// Computes the inner product 〈self|other〉.
    ///
    /// This method works by finding a sequence of Clifford operations that
    /// transforms the state |self> into the |0...0> state, and then applying
    /// the same sequence of operations to |other>. The inner product is then
    /// derived from the resulting state's amplitude at the |0...0> basis state.
    pub fn inner_product(&self, other: &StabilizerCHForm) -> Result<num_complex::Complex64> {
        self._inner_product(other)
    }

    pub fn measure(&mut self, qarg: usize) -> Result<bool> {
        self._measure(qarg)
    }

    pub fn to_statevector(&self) -> Result<ndarray::Array1<Complex64>> {
        self._to_statevector()
    }

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

    pub fn from_clifford_circuit(circuit: &CliffordCircuit) -> Result<Self> {
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

    /// Applies the Hadamard gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n^2)
    ///
    /// See around Proposition 4. of arXiv:1808.00128 for details.
    pub fn apply_h(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_h(qarg)
    }

    /// Applies the Pauli-X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(48) of arXiv:1808.00128 for details.
    pub fn apply_x(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_x(qarg)
    }

    /// Applies the Pauli-Y gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    pub fn apply_y(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_y(qarg)
    }

    /// Applies the Pauli-Z gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(1)
    pub fn apply_z(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_z(qarg)
    }

    /// Applies the Phase (S) gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n)
    ///
    /// See around the end of Proposition 4 of arXiv:1808.00128 for details.
    pub fn apply_s(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_s(qarg)
    }

    /// Applies the adjoint Phase (S†) gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    pub fn apply_sdg(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_sdg(qarg)
    }

    /// Applies the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    pub fn apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_sqrt_x(qarg)
    }

    /// Applies the adjoint of the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    pub fn apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_sqrt_xdg(qarg)
    }

    /// Applies the CNOT (CX) gate with control qubit at index `control` and target qubit at index `target`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(49) of arXiv:1808.00128 for details.
    pub fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        self._left_multiply_cx(control, target)
    }

    /// Applies the CZ gate between qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(50) of arXiv:1808.00128 for details.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self._left_multiply_cz(qarg1, qarg2)
    }

    /// Applies the SWAP gate between the qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self._left_multiply_swap(qarg1, qarg2)
    }

    /// Applies a Pauli string to the stabilizer state.
    pub fn apply_pauli(&mut self, pauli_string: &PauliString) -> Result<()> {
        self._apply_pauli(pauli_string)
    }

    /// Applies a CliffordGate to the stabilizer state.
    pub fn apply_gate(&mut self, gate: &CliffordGate) -> Result<()> {
        self._apply_gate(gate)
    }

    /// Applies a CliffordCircuit to the stabilizer state.
    pub fn apply_circuit(&mut self, circuit: &CliffordCircuit) -> Result<()> {
        self._apply_circuit(circuit)
    }
}
