use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;
use std::str::FromStr;

use stab_decomp_simulator_rust::prelude::{
    QuantumState as RustQuantumState,
    PauliString,
};

#[pyclass(name = "QuantumState")]
pub struct PyQuantumState {
    pub(crate) inner: RustQuantumState,
}

#[pymethods]
impl PyQuantumState {
    fn exp_value(&self, pauli_string: String) -> PyResult<f64> {
        let pauli_op = PauliString::from_str(&pauli_string)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let exp_val = self.inner.exp_value(&pauli_op)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(exp_val.re)
    }

    fn sample(&self, qargs: Vec<usize>, shots: usize) -> PyResult<HashMap<String, usize>> {
        let shot_count = self.inner.sample(&qargs, shots, None)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(shot_count.into_iter().map(|(outcome, count)| {
            (outcome.into_iter().map(|b| if b { '1' } else { '0' }).collect(), count)
        }).collect())
    }

    fn apply_h(&mut self, target: usize) -> PyResult<()> {
        self.inner.apply_h(target).map_err(|e| PyValueError::new_err(e.to_string()))
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
            self.num_qubits(), self.stabilizer_rank()
        )
    }
}