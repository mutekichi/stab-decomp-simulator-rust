use num_complex::Complex64;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use stab_decomp_simulator_rust::prelude::{QuantumGate, QuantumState as RustQuantumState};

use crate::gate::PyQuantumGate;
use crate::pauli_string::PyPauliString;
use crate::utils::parse_py_seed;

#[pyclass(name = "QuantumState")]
pub struct PyQuantumState {
    pub(crate) inner: RustQuantumState,
}

#[pymethods]
impl PyQuantumState {
    #[staticmethod]
    fn from_circuit(circuit: &crate::circuit::PyQuantumCircuit) -> PyResult<Self> {
        let state = RustQuantumState::from_circuit(&circuit.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyQuantumState { inner: state })
    }

    fn to_statevector(&self) -> PyResult<Vec<Complex64>> {
        let sv = self
            .inner
            .to_statevector()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(sv.into_iter().map(|c| Complex64::new(c.re, c.im)).collect())
    }

    fn inner_product(&self, other: &PyQuantumState) -> PyResult<Complex64> {
        let ip = self
            .inner
            .inner_product(&other.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(ip)
    }

    fn measure(
        &mut self,
        qargs: Vec<usize>,
        seed: Option<Bound<'_, PyAny>>,
    ) -> PyResult<Vec<bool>> {
        let rust_seed = parse_py_seed(seed)?;
        let results = self
            .inner
            .measure(&qargs, rust_seed)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(results)
    }

    fn measure_all(&mut self, seed: Option<Bound<'_, PyAny>>) -> PyResult<Vec<bool>> {
        let rust_seed = parse_py_seed(seed)?;
        let results = self
            .inner
            .measure_all(rust_seed)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(results)
    }

    fn sample(
        &self,
        qargs: Vec<usize>,
        shots: usize,
        seed: Option<Bound<'_, PyAny>>,
    ) -> PyResult<HashMap<String, usize>> {
        let rust_seed = parse_py_seed(seed)?;

        let shot_count = self
            .inner
            .sample(&qargs, shots, rust_seed)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        let py_shot_count: HashMap<String, usize> = shot_count
            .into_iter()
            .map(|(outcome_vec, count)| {
                // Convert Vec<bool> to String like "010"
                let outcome_str: String = outcome_vec
                    .into_iter()
                    .map(|b| if b { '1' } else { '0' })
                    .collect();
                (outcome_str, count)
            })
            .collect();

        Ok(py_shot_count)
    }

    fn exp_value(&self, pauli_op: &PyPauliString) -> PyResult<f64> {
        let exp_val = self
            .inner
            .exp_value(&pauli_op.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(exp_val)
    }

    fn project_normalized(&mut self, qubit: usize, outcome: bool) -> PyResult<()> {
        self.inner
            .project_normalized(qubit, outcome)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn project_unnormalized(&mut self, qubit: usize, outcome: bool) -> PyResult<()> {
        self.inner
            .project_unnormalized(qubit, outcome)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn discard(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .discard(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_gate(&mut self, gate: &PyQuantumGate) -> PyResult<()> {
        self.inner
            .apply_gate(&gate.internal)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_gates(&mut self, gates: Vec<PyQuantumGate>) -> PyResult<()> {
        let rust_gates: Vec<QuantumGate> = gates.into_iter().map(|g| g.internal).collect();
        self.inner
            .apply_gates(&rust_gates)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_x(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_x(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_y(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_y(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_z(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_z(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_h(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_h(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_s(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_s(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_sdg(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_sdg(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_sqrt_x(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_sqrt_x(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_sqrt_xdg(&mut self, qubit: usize) -> PyResult<()> {
        self.inner
            .apply_sqrt_xdg(qubit)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_cx(&mut self, control: usize, target: usize) -> PyResult<()> {
        self.inner
            .apply_cx(control, target)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> PyResult<()> {
        self.inner
            .apply_cz(qarg1, qarg2)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> PyResult<()> {
        self.inner
            .apply_swap(qarg1, qarg2)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn norm(&self) -> PyResult<f64> {
        let norm = self
            .inner
            .norm()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(norm)
    }

    #[getter]
    fn stabilizer_rank(&self) -> usize {
        self.inner.stabilizer_rank()
    }

    #[getter]
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits()
    }

    fn __str__(&self) -> String {
        format!(
            "QuantumState(num_qubits={}, stabilizer_rank={})",
            self.num_qubits(),
            self.stabilizer_rank()
        )
    }
}
