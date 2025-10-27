use pyo3::prelude::*;
use stab_decomp_simulator_rust::circuit::QuantumGate as RustQuantumGate;

#[pyclass(name = "QuantumGate")]
#[derive(Debug, Clone)]
pub struct PyQuantumGate {
    internal: RustQuantumGate,
}

#[pymethods]
impl PyQuantumGate {
    #[getter]
    fn name(&self) -> &'static str {
        self.internal.name()
    }

    #[getter]
    fn qubits(&self) -> Vec<usize> {
        self.internal.qubits()
    }

    fn __str__(&self) -> String {
        self.internal.to_string()
    }

    fn __repr__(&self) -> String {
        format!("<QuantumGate: {}>", self.internal.to_string())
    }

    #[getter]
    pub fn is_clifford(&self) -> bool {
        self.internal.is_clifford()
    }

    #[getter]
    pub fn is_t_type(&self) -> bool {
        self.internal.is_t_type_gate()
    }
}
