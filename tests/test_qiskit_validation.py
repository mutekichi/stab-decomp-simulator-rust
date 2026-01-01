import itertools
import random
from typing import Optional

import pytest
from necstar import PauliString as NcPauliString
from necstar import QuantumCircuit as NcQuantumCircuit
from necstar import QuantumState as NcQuantumState
from qiskit import qasm2
from qiskit.quantum_info import Pauli as QiskitPauli
from qiskit.quantum_info import Statevector

SINGLE_QUBIT_CLIFFORDS = ["h", "x", "y", "z", "s", "sdg", "sx", "sxdg"]
TWO_QUBIT_CLIFFORDS = ["cx", "cz", "swap"]
T_TYPE_GATES = ["t", "tdg"]


def generate_random_circuit_qasm(
    num_qubits: int,
    num_single_clifford: int,
    num_two_clifford: int,
    num_t: int,
    seed: Optional[int] = None,
) -> str:
    if seed is not None:
        random.seed(seed)

    gates_to_apply = []
    # Add single-qubit Cliffords
    for _ in range(num_single_clifford):
        gate = random.choice(SINGLE_QUBIT_CLIFFORDS)
        target = random.randrange(num_qubits)
        gates_to_apply.append((gate, [target]))

    # Add two-qubit Cliffords
    if num_qubits >= 2:
        for _ in range(num_two_clifford):
            gate = random.choice(TWO_QUBIT_CLIFFORDS)
            q1, q2 = random.sample(range(num_qubits), 2)
            gates_to_apply.append((gate, [q1, q2]))

    # Add T-type gates
    for _ in range(num_t):
        gate = random.choice(T_TYPE_GATES)
        target = random.randrange(num_qubits)
        gates_to_apply.append((gate, [target]))

    random.shuffle(gates_to_apply)

    qasm_lines = ["OPENQASM 2.0;", 'include "qelib1.inc";', f"qreg q[{num_qubits}];"]
    for gate_name, qubits in gates_to_apply:
        qubit_args = ", ".join([f"q[{i}]" for i in qubits])
        qasm_lines.append(f"{gate_name} {qubit_args};")

    return "\n".join(qasm_lines)


def assert_statevector_match(
    necstar_state: NcQuantumState,
    qiskit_sv: Statevector,
    tolerance: float = 1e-6,
) -> None:
    necstar_sv = necstar_state.to_statevector()
    qiskit_data = qiskit_sv.data
    assert len(necstar_sv) == len(qiskit_data)
    for i in range(len(necstar_sv)):
        assert abs(necstar_sv[i] - qiskit_data[i]) < tolerance


def assert_exp_value_match(
    num_qubits: int,
    necstar_state: NcQuantumState,
    qiskit_sv: Statevector,
    tolerance: float = 1e-6,
) -> None:
    paulis = ["I", "X", "Y", "Z"]
    # Check all possible Pauli strings for n qubits
    for pauli_tuple in itertools.product(paulis, repeat=num_qubits):
        pauli_str = "".join(pauli_tuple)
        nc_pauli = NcPauliString.from_str(pauli_str)
        qk_pauli = QiskitPauli(pauli_str)

        nc_val = necstar_state.exp_value(nc_pauli)
        qk_val = qiskit_sv.expectation_value(qk_pauli).real
        assert abs(nc_val - qk_val) < tolerance


def assert_inner_product_match(
    necstar_state1: NcQuantumState,
    necstar_state2: NcQuantumState,
    qiskit_sv1: Statevector,
    qiskit_sv2: Statevector,
    tolerance: float = 1e-6,
) -> None:
    # necstar.inner_product calculates <state2 | state1>
    nc_inner = necstar_state2.inner_product(necstar_state1)
    qk_inner = qiskit_sv2.inner(qiskit_sv1)
    assert abs(nc_inner - qk_inner) < tolerance


@pytest.mark.parametrize("num_qubits", [2, 3, 4])
@pytest.mark.parametrize("trial", range(5))
def test_comprehensive_validation(num_qubits: int, trial: int) -> None:
    seed = 42 + trial

    # Generate random state 1
    qasm1 = generate_random_circuit_qasm(num_qubits, 20, 30, 6, seed=seed)
    nc_circuit1 = NcQuantumCircuit.from_qasm_str(qasm1)
    qk_circuit1 = qasm2.loads(
        qasm1, custom_instructions=qasm2.LEGACY_CUSTOM_INSTRUCTIONS
    )

    nc_state1 = NcQuantumState.from_circuit(nc_circuit1)
    qk_state1 = Statevector(qk_circuit1)

    # Statevector verification
    assert_statevector_match(nc_state1, qk_state1)

    # Expectation value verification
    assert_exp_value_match(num_qubits, nc_state1, qk_state1)

    # Measurement and Projection verification
    # Pick a random qubit to measure
    measure_target = random.randrange(num_qubits)
    outcome, qk_state1_after = qk_state1.measure([measure_target])
    nc_state1.project_normalized(measure_target, outcome == "1")

    assert_statevector_match(nc_state1, qk_state1_after)

    # Inner product verification
    # Generate another random state 2
    qasm2_str = generate_random_circuit_qasm(num_qubits, 5, 5, 2, seed=seed + 100)
    nc_circuit2 = NcQuantumCircuit.from_qasm_str(qasm2_str)
    qk_circuit2 = qasm2.loads(
        qasm2_str, custom_instructions=qasm2.LEGACY_CUSTOM_INSTRUCTIONS
    )

    nc_state2 = NcQuantumState.from_circuit(nc_circuit2)
    qk_state2 = Statevector(qk_circuit2)

    assert_inner_product_match(nc_state1, nc_state2, qk_state1_after, qk_state2)
