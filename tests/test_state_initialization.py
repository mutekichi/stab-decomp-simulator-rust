import pytest
from necstar import QuantumCircuit, QuantumState


def test_state_initialization():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_t(0)
    circuit.apply_cx(0, 1)
    state = QuantumState.from_circuit(circuit)
    assert state.num_qubits == 2
    assert state.stabilizer_rank == 2
    ref_statevector = [
        1 / 2**0.5,
        0.0,
        0.0,
        0.5 + 0.5j,
    ]
    for amp, ref_amp in zip(state.to_statevector(), ref_statevector):
        assert abs(amp - ref_amp) < 1e-10
    assert abs(state.norm() - 1.0) < 1e-10
    assert str(state) == "QuantumState(num_qubits=2, stabilizer_rank=2)"


def test_state_initialization_invalid_circuit():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_t(3)  # Invalid qubit index
    circuit.apply_cx(0, 1)
    with pytest.raises(ValueError):
        QuantumState.from_circuit(circuit)
