use pyo3::prelude::*;

mod circuit;
mod gate;
mod pauli_string;
mod state;

use circuit::PyQuantumCircuit;
use gate::PyQuantumGate;
use pauli_string::PyPauliString;
use state::PyQuantumState;

#[pymodule]
fn necstar(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyQuantumGate>()?;
    m.add_class::<PyQuantumCircuit>()?;
    m.add_class::<PyQuantumState>()?;
    m.add_class::<PyPauliString>()?;

    Ok(())
}
