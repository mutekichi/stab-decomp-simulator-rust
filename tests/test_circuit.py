import pytest
from necstar import QuantumCircuit


def test_circuit_initialization():
    circuit = QuantumCircuit(2)
    assert circuit.num_qubits == 2
    assert len(circuit.gates) == 0


def test_gate_addition():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_cx(0, 1)

    assert len(circuit.gates) == 2
    assert circuit.gates[0].name == "H"
    assert circuit.gates[0].qubits == [0]
    assert circuit.gates[1].name == "CX"
    assert circuit.gates[1].qubits == [0, 1]


def test_gate_application_by_name():
    circuit = QuantumCircuit(2)
    circuit.apply_gate("x", [1])
    circuit.apply_gate("cz", [0, 1])

    assert len(circuit.gates) == 2
    assert circuit.gates[0].name == "X"
    assert circuit.gates[0].qubits == [1]
    assert circuit.gates[1].name == "CZ"
    assert circuit.gates[1].qubits == [0, 1]


def test_gate_application_invalid_name():
    circuit = QuantumCircuit(2)
    with pytest.raises(ValueError):
        circuit.apply_gate("invalid_gate", [0])


def test_gate_application_invalid_qubit_count():
    circuit = QuantumCircuit(2)
    with pytest.raises(ValueError):
        circuit.apply_gate("cx", [0])
    with pytest.raises(ValueError):
        circuit.apply_gate("h", [0, 1])
