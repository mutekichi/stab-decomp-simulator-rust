use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::str::FromStr;

// Import the Rust PauliString type
use stab_decomp_simulator_rust::types::PauliString as RustPauliString;

#[pyclass(name = "PauliString", module = "necstar")]
#[derive(Clone, Debug)]
pub struct PyPauliString {
    pub(crate) inner: RustPauliString,
}

#[pymethods]
impl PyPauliString {
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        let rust_pauli = RustPauliString::from_str(s)
            .map_err(|e| PyValueError::new_err(format!("Invalid Pauli string format: {}", e)))?;
        Ok(PyPauliString { inner: rust_pauli })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("PauliString('{}')", self.inner)
    }

    #[getter]
    fn is_identity(&self) -> bool {
        self.inner.is_identity()
    }
}
