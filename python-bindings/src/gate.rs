use necstar_core::circuit::QuantumGate as RustQuantumGate;
use pyo3::prelude::*;

#[pyclass(name = "QuantumGate")]
#[derive(Debug, Clone)]
pub struct PyQuantumGate {
    pub(crate) internal: RustQuantumGate,
}

#[pymethods]
impl PyQuantumGate {
    #[staticmethod]
    fn h(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::H(target),
        }
    }
    #[staticmethod]
    fn x(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::X(target),
        }
    }
    #[staticmethod]
    fn y(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::Y(target),
        }
    }
    #[staticmethod]
    fn z(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::Z(target),
        }
    }
    #[staticmethod]
    fn s(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::S(target),
        }
    }
    #[staticmethod]
    fn sdg(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::Sdg(target),
        }
    }
    #[staticmethod]
    fn sqrt_x(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::SqrtX(target),
        }
    }
    #[staticmethod]
    fn sqrt_xdg(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::SqrtXdg(target),
        }
    }
    #[staticmethod]
    fn cx(control: usize, target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::CX(control, target),
        }
    }
    #[staticmethod]
    fn cz(qarg1: usize, qarg2: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::CZ(qarg1, qarg2),
        }
    }
    #[staticmethod]
    fn swap(qarg1: usize, qarg2: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::Swap(qarg1, qarg2),
        }
    }
    #[staticmethod]
    fn t(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::T(target),
        }
    }
    #[staticmethod]
    fn tdg(target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::Tdg(target),
        }
    }
    #[staticmethod]
    fn ccx(control1: usize, control2: usize, target: usize) -> Self {
        PyQuantumGate {
            internal: RustQuantumGate::CCX(control1, control2, target),
        }
    }

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
        format!("<QuantumGate: {}>", self.internal)
    }

    pub fn is_clifford(&self) -> bool {
        self.internal.is_clifford()
    }

    pub fn is_t_type(&self) -> bool {
        self.internal.is_t_type_gate()
    }
}
