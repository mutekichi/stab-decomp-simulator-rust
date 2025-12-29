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

def test_random_clifford_circuit():
    n_qubits = 3
    seed = 42
    circuit = QuantumCircuit.random_clifford(n_qubits, seed=seed)

    assert circuit.num_qubits == n_qubits
    assert len(circuit.gates) > 0  # Ensure that some gates were added

def test_random_circuit_determinism():
    n_qubits = 4
    seed = 12345678901234567890
    circuit1 = QuantumCircuit.random_clifford(n_qubits, seed=seed)
    circuit2 = QuantumCircuit.random_clifford(n_qubits, seed=seed)

    assert len(circuit1.gates) == len(circuit2.gates)
    for gate1, gate2 in zip(circuit1.gates, circuit2.gates):
        assert gate1.name == gate2.name
        assert gate1.qubits == gate2.qubits

def test_random_circuit_large_seed():
    n_qubits = 4
    large_seed = (1 << 256) - 1
    circuit = QuantumCircuit.random_clifford(n_qubits, seed=large_seed)
    assert circuit.num_qubits is not None

def test_invalid_seed():
    n_qubits = 4
    # > 256 bits should raise an error
    too_large_seed = 1 << 300
    with pytest.raises(OverflowError):
        QuantumCircuit.random_clifford(n_qubits, seed=too_large_seed)
    # negative seed should raise an error
    negative_seed = -1
    with pytest.raises(OverflowError):
        QuantumCircuit.random_clifford(n_qubits, seed=negative_seed)

def test_random_circuit_no_seed():
    n_qubits = 5
    circuit_set = set()
    for _ in range(10):
        circuit = QuantumCircuit.random_clifford(n_qubits)
        circuit_str = str(circuit)
        print(circuit_str)
        circuit_set.add(circuit_str)
