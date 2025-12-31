import numpy as np
import pytest
from necstar import PauliString, QuantumCircuit, QuantumState


def make_toffoli_state():
    """Creates a "Toffoli state":
    |Toffoli> = (|000> + |100> + |010> + |111>) / 2
    """
    circuit = QuantumCircuit(3)
    # Initialize control qubits to uniform superposition
    circuit.apply_h(0)
    circuit.apply_h(1)
    # Apply Toffoli gate using 7 t-type gates
    circuit.apply_h(2)
    circuit.apply_cx(1, 2)
    circuit.apply_tdg(2)
    circuit.apply_cx(0, 2)
    circuit.apply_t(2)
    circuit.apply_cx(1, 2)
    circuit.apply_tdg(2)
    circuit.apply_cx(0, 2)

    circuit.apply_t(1)
    circuit.apply_t(2)
    circuit.apply_cx(0, 1)
    circuit.apply_h(2)

    circuit.apply_t(0)
    circuit.apply_tdg(1)
    circuit.apply_cx(0, 1)

    state = QuantumState.from_circuit(circuit)
    return state


def test_state_expectation_value():
    toffoli_state = make_toffoli_state()
    observable = PauliString.from_str("ZII")
    exp_value = toffoli_state.exp_value(observable)
    assert np.isclose(exp_value, 0.5), f"Unexpected expectation value: {exp_value}"

    observable_large_dense = PauliString.from_str("ZZIIII")
    with pytest.raises(ValueError):
        toffoli_state.exp_value(observable_large_dense)
    observable_large_sparse = PauliString.from_str("Z0 Y2 X4")
    with pytest.raises(ValueError):
        toffoli_state.exp_value(observable_large_sparse)
    observable_small = PauliString.from_str("ZI")
    with pytest.raises(ValueError):
        toffoli_state.exp_value(observable_small)


def test_state_inner_product():
    state1 = make_toffoli_state()
    state2 = make_toffoli_state()
    inner_prod = state1.inner_product(state2)
    assert np.isclose(
        inner_prod, 1.0
    ), f"Inner product of identical states should be 1, got {inner_prod}"

    # Mismatched qubit counts should raise ValueError
    circuit_large = QuantumCircuit(4)
    circuit_large.apply_h(0)
    state_large = QuantumState.from_circuit(circuit_large)
    with pytest.raises(ValueError):
        state1.inner_product(state_large)

    circuit_small = QuantumCircuit(2)
    circuit_small.apply_h(0)
    state_small = QuantumState.from_circuit(circuit_small)
    with pytest.raises(ValueError):
        state1.inner_product(state_small)
