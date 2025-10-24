use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

use stab_decomp_simulator_rust::prelude::{PauliString, QuantumState as RustQuantumState};

// Helper function to convert Python seed (Option<u64>) to Rust seed (Option<[u8; 32]>)
fn convert_py_seed(py_seed: Option<u64>) -> Option<[u8; 32]> {
    py_seed.map(|s| {
        let mut seed_array = [0u8; 32];
        // Use the lower 8 bytes (64 bits) of the u64 for the seed
        let bytes = s.to_le_bytes(); // Little-endian representation
        seed_array[..8].copy_from_slice(&bytes);
        seed_array
    })
}

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

    fn to_statevector(&self) -> PyResult<Vec<(f64, f64)>> {
        let sv = self
            .inner
            .to_statevector()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(sv.iter().map(|c| (c.re, c.im)).collect())
    }

    fn inner_product(&self, other: &PyQuantumState) -> PyResult<(f64, f64)> {
        let ip = self
            .inner
            .inner_product(&other.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok((ip.re, ip.im))
    }

    fn measure(&mut self, qargs: Vec<usize>, seed: Option<u64>) -> PyResult<Vec<bool>> {
        let rust_seed = convert_py_seed(seed);
        let results = self
            .inner
            .measure(&qargs, rust_seed)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(results)
    }

    fn measure_all(&mut self, seed: Option<u64>) -> PyResult<Vec<bool>> {
        let rust_seed = convert_py_seed(seed);
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
        seed: Option<u64>,
    ) -> PyResult<HashMap<String, usize>> {
        let rust_seed = convert_py_seed(seed);

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

    fn exp_value(&self, pauli_string: String) -> PyResult<f64> {
        let pauli_op = PauliString::from_str(&pauli_string)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let exp_val = self
            .inner
            .exp_value(&pauli_op)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(exp_val.re)
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
