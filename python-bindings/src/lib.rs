use pyo3::prelude::*;

mod circuit;
mod pauli_string;
mod state;

use circuit::PyQuantumCircuit;
use pauli_string::PyPauliString;
use state::PyQuantumState;

#[pymodule]
fn necstar(m: &PyModule) -> PyResult<()> {
    m.add_class::<PyQuantumCircuit>()?;
    m.add_class::<PyQuantumState>()?;
    m.add_class::<PyPauliString>()?;

    Ok(())
}
