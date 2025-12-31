import os

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
    num_qubits = 3
    seed = 42
    circuit = QuantumCircuit.random_clifford(num_qubits, seed=seed)

    assert circuit.num_qubits == num_qubits
    assert len(circuit.gates) > 0  # Ensure that some gates were added


def test_random_circuit_determinism():
    num_qubits = 4
    seed = 12345678901234567890
    circuit1 = QuantumCircuit.random_clifford(num_qubits, seed=seed)
    circuit2 = QuantumCircuit.random_clifford(num_qubits, seed=seed)

    assert len(circuit1.gates) == len(circuit2.gates)
    for gate1, gate2 in zip(circuit1.gates, circuit2.gates):
        assert gate1.name == gate2.name
        assert gate1.qubits == gate2.qubits


def test_random_circuit_large_seed():
    num_qubits = 4
    large_seed = (1 << 256) - 1
    circuit = QuantumCircuit.random_clifford(num_qubits, seed=large_seed)
    assert circuit.num_qubits is not None


def test_invalid_seed():
    num_qubits = 4
    # > 256 bits should raise an error
    too_large_seed = 1 << 300
    with pytest.raises(OverflowError):
        QuantumCircuit.random_clifford(num_qubits, seed=too_large_seed)
    # negative seed should raise an error
    negative_seed = -1
    with pytest.raises(OverflowError):
        QuantumCircuit.random_clifford(num_qubits, seed=negative_seed)


def test_random_circuit_no_seed():
    num_qubits = 5
    circuit_set = set()
    for _ in range(10):
        circuit = QuantumCircuit.random_clifford(num_qubits)
        circuit_str = str(circuit)
        circuit_set.add(circuit_str)
    assert len(circuit_set) > 1


def test_from_qasm_str():
    qasm_str = """
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];
t q[1];
"""
    circuit = QuantumCircuit.from_qasm_str(qasm_str)
    assert circuit.num_qubits == 2
    assert len(circuit.gates) == 3
    assert circuit.gates[0].name == "H"
    assert circuit.gates[1].name == "CX"
    assert circuit.gates[2].name == "T"


def test_from_qasm_str_invalid():
    # Unsupported gate (e.g., RX)
    qasm_str_unsupported_gate = """
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
t q[1];
rx(pi/4) q[0];
"""
    with pytest.raises(ValueError):
        QuantumCircuit.from_qasm_str(qasm_str_unsupported_gate)

    # Malformed QASM syntax (e.g., missing semicolon)
    qasm_str_bad_syntax = """
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2]
h q[0]
"""
    with pytest.raises(ValueError):
        QuantumCircuit.from_qasm_str(qasm_str_bad_syntax)


def test_to_qasm_str():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_cx(0, 1)
    circuit.apply_t(1)

    qasm_str = circuit.to_qasm_str(reg_name="q")
    expected_qasm_str = """OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];
t q[1];
"""
    assert qasm_str.strip() == expected_qasm_str.strip()


def test_qasm_file_roundtrip():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_cx(0, 1)
    circuit.apply_t(1)

    qasm_file_path = "test_circuit.qasm"
    circuit.to_qasm_file(str(qasm_file_path), reg_name="q")

    loaded_circuit = QuantumCircuit.from_qasm_file(str(qasm_file_path))

    assert loaded_circuit.num_qubits == circuit.num_qubits
    assert len(loaded_circuit.gates) == len(circuit.gates)
    for gate_loaded, gate_original in zip(loaded_circuit.gates, circuit.gates):
        assert gate_loaded.name == gate_original.name
        assert gate_loaded.qubits == gate_original.qubits

    os.remove(qasm_file_path)


def test_append_circuit():
    circuit1 = QuantumCircuit(2)
    circuit1.apply_h(0)
    circuit2 = QuantumCircuit(2)
    circuit2.apply_cx(0, 1)
    circuit2.apply_t(0)

    circuit1.append(circuit2)

    assert circuit1.num_qubits == 2
    assert len(circuit1.gates) == 3
    assert circuit1.gates[0].name == "H"
    assert circuit1.gates[0].qubits == [0]
    assert circuit1.gates[1].name == "CX"
    assert circuit1.gates[1].qubits == [0, 1]
    assert circuit1.gates[2].name == "T"
    assert circuit1.gates[2].qubits == [0]


def test_tensor_circuit():
    circuit1 = QuantumCircuit(2)
    circuit1.apply_h(0)
    circuit2 = QuantumCircuit(3)
    circuit2.apply_cx(0, 1)
    circuit2.apply_t(2)

    tensor_circuit = circuit1.tensor(circuit2)

    assert tensor_circuit.num_qubits == 5
    assert len(tensor_circuit.gates) == 3
    assert tensor_circuit.gates[0].name == "H"
    assert tensor_circuit.gates[0].qubits == [0]
    assert tensor_circuit.gates[1].name == "CX"
    assert tensor_circuit.gates[1].qubits == [2, 3]
    assert tensor_circuit.gates[2].name == "T"
    assert tensor_circuit.gates[2].qubits == [4]


def test_circuit_str_representation():
    circuit = QuantumCircuit(2)
    circuit.apply_h(0)
    circuit.apply_cx(0, 1)
    circuit.apply_t(1)

    circuit_str = str(circuit)
    expected_str = "QuantumCircuit(num_qubits=2) [H(0), CX(0, 1), T(1)]"
    assert circuit_str == expected_str
