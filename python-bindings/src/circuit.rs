use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;

use stab_decomp_simulator_rust::circuit::{
    QuantumCircuit as RustQuantumCircuit, QuantumGate as RustQuantumGate,
};

use crate::gate::PyQuantumGate;
use crate::utils::parse_py_seed;

#[pyclass(name = "QuantumCircuit")]
pub struct PyQuantumCircuit {
    pub(crate) inner: RustQuantumCircuit,
}

#[pymethods]
impl PyQuantumCircuit {
    #[new]
    fn new(num_qubits: usize) -> Self {
        PyQuantumCircuit {
            inner: RustQuantumCircuit::new(num_qubits),
        }
    }

    #[getter]
    fn num_qubits(&self) -> usize {
        self.inner.n_qubits
    }

    #[getter]
    fn num_gates(&self) -> usize {
        self.inner.gates.len()
    }

    #[getter]
    fn gates(&self) -> Vec<PyQuantumGate> {
        self.inner
            .gates
            .iter()
            .cloned()
            .map(|g| PyQuantumGate { internal: g })
            .collect()
    }

    #[staticmethod]
    fn from_qasm_file(path: String) -> PyResult<Self> {
        let rust_circuit = RustQuantumCircuit::from_qasm_file(&path).map_err(|e| {
            PyFileNotFoundError::new_err(format!("Failed to read QASM file: {}", e))
        })?;
        Ok(PyQuantumCircuit {
            inner: rust_circuit,
        })
    }

    #[staticmethod]
    fn from_qasm_str(qasm: String) -> PyResult<Self> {
        let rust_circuit = RustQuantumCircuit::from_qasm_str(&qasm)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse QASM string: {}", e)))?;
        Ok(PyQuantumCircuit {
            inner: rust_circuit,
        })
    }

    fn to_qasm_str(&self, reg_name: String) -> String {
        self.inner.to_qasm_str(&reg_name)
    }

    fn to_qasm_file(&self, path: String, reg_name: String) -> PyResult<()> {
        self.inner
            .to_qasm_file(&path, &reg_name)
            .map_err(|e| PyValueError::new_err(format!("Failed to write QASM file: {}", e)))
    }

    #[staticmethod]
    fn random_clifford(n: usize, seed: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
        let rust_seed = parse_py_seed(seed)?;
        let rust_circuit = RustQuantumCircuit::random_clifford(n, rust_seed);
        Ok(PyQuantumCircuit {
            inner: rust_circuit,
        })
    }

    fn append(&mut self, other: &PyQuantumCircuit) {
        self.inner.append(&other.inner);
    }

    fn tensor(&self, other: &PyQuantumCircuit) -> PyQuantumCircuit {
        let new_circuit = self.inner.tensor(&other.inner);
        PyQuantumCircuit { inner: new_circuit }
    }

    fn apply_gate(&mut self, gate_name: String, qubits: Vec<usize>) -> PyResult<()> {
        let gate = match gate_name.to_lowercase().as_str() {
            // --- Single-qubit gates ---
            "h" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("H gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::H(qubits[0]))
            }
            "x" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("X gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::X(qubits[0]))
            }
            "y" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("Y gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::Y(qubits[0]))
            }
            "z" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("Z gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::Z(qubits[0]))
            }
            "s" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("S gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::S(qubits[0]))
            }
            "sdg" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("Sdg gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::Sdg(qubits[0]))
            }
            "sqrtx" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("SqrtX gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::SqrtX(qubits[0]))
            }
            "sqrtxdg" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err(
                        "SqrtXdg gate requires 1 qubit index.",
                    ));
                }
                Ok(RustQuantumGate::SqrtXdg(qubits[0]))
            }
            "t" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("T gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::T(qubits[0]))
            }
            "tdg" => {
                if qubits.len() != 1 {
                    return Err(PyValueError::new_err("Tdg gate requires 1 qubit index."));
                }
                Ok(RustQuantumGate::Tdg(qubits[0]))
            }

            // --- Two-qubit gates ---
            "cx" | "cnot" => {
                if qubits.len() != 2 {
                    return Err(PyValueError::new_err(
                        "CX gate requires 2 qubit indices (control, target).",
                    ));
                }
                Ok(RustQuantumGate::CX(qubits[0], qubits[1]))
            }
            "cz" => {
                if qubits.len() != 2 {
                    return Err(PyValueError::new_err("CZ gate requires 2 qubit indices."));
                }
                Ok(RustQuantumGate::CZ(qubits[0], qubits[1]))
            }
            "swap" => {
                if qubits.len() != 2 {
                    return Err(PyValueError::new_err("SWAP gate requires 2 qubit indices."));
                }
                Ok(RustQuantumGate::Swap(qubits[0], qubits[1]))
            }

            // --- Unrecognized gate ---
            _ => Err(PyValueError::new_err(format!(
                "Unknown gate name: '{}'",
                gate_name
            ))),
        }?;

        self.inner.apply_gate(gate);
        Ok(())
    }

    fn apply_h(&mut self, target: usize) {
        self.inner.apply_h(target);
    }
    fn apply_x(&mut self, target: usize) {
        self.inner.apply_x(target);
    }
    fn apply_y(&mut self, target: usize) {
        self.inner.apply_y(target);
    }
    fn apply_z(&mut self, target: usize) {
        self.inner.apply_z(target);
    }
    fn apply_s(&mut self, target: usize) {
        self.inner.apply_s(target);
    }
    fn apply_sdg(&mut self, target: usize) {
        self.inner.apply_sdg(target);
    }
    fn apply_sqrt_x(&mut self, target: usize) {
        self.inner.apply_sqrt_x(target);
    }
    fn apply_sqrt_xdg(&mut self, target: usize) {
        self.inner.apply_sqrt_xdg(target);
    }
    fn apply_t(&mut self, target: usize) {
        self.inner.apply_t(target);
    }
    fn apply_tdg(&mut self, target: usize) {
        self.inner.apply_tdg(target);
    }
    fn apply_cx(&mut self, control: usize, target: usize) {
        self.inner.apply_cx(control, target);
    }
    fn apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        self.inner.apply_cz(qarg1, qarg2);
    }
    fn apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        self.inner.apply_swap(qarg1, qarg2);
    }
    fn apply_ccx(&mut self, control1: usize, control2: usize, target: usize) {
        self.inner.apply_ccx(control1, control2, target);
    }

    fn __str__(&self) -> String {
        format!(
            "QuantumCircuit(num_qubits={}, num_gates={})",
            self.inner.n_qubits,
            self.num_gates()
        )
    }
}
