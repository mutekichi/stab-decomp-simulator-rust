use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

mod circuit;
mod state;

use circuit::PyQuantumCircuit;
use state::PyQuantumState;

use stab_decomp_simulator_rust::prelude::QuantumState as RustQuantumState;

#[pyfunction]
fn compile(circuit: &PyQuantumCircuit) -> PyResult<PyQuantumState> {
    let state = RustQuantumState::from_circuit(&circuit.inner)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(PyQuantumState { inner: state })
}


#[pymodule]
fn necstar(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyQuantumCircuit>()?;
    m.add_class::<PyQuantumState>()?;

    m.add_function(wrap_pyfunction!(compile, m)?)?;
    
    Ok(())
}