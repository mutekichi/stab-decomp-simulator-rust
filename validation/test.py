import itertools
import random
from typing import Optional

from necstar import PauliString as NcPauliString
from necstar import QuantumCircuit as NcQuantumCircuit
from necstar import QuantumGate as NcQuantumGate
from necstar import QuantumState as NcQuantumState
from qiskit import qasm2
from qiskit.quantum_info import Pauli as QiskitPauli
from qiskit.quantum_info import Statevector

SINGLE_QUBIT_CLIFFORDS = ["h", "x", "y", "z", "s", "sdg", "sx", "sxdg"]
TWO_QUBIT_CLIFFORDS = ["cx", "cz", "swap"]
T_TYPE_GATES = ["t", "tdg"]


def generate_random_circuit_qasm(
    n_qubits: int,
    num_single_clifford_each: int,
    num_two_clifford_each: int,
    num_t_each: int,
    seed: Optional[int] = None
) -> str:
    """
    Generates a random quantum circuit in OpenQASM format.

    Args:
        n_qubits (int): Number of qubits in the circuit.
        num_single_clifford_each (int): Number of each single-qubit Clifford gate to include.
        num_two_clifford_each (int): Number of each two-qubit Clifford gate to include.
        num_t_each (int): Number of each T-type gate (T and T†) to include.
        seed (Optional[int]): Seed for random number generator for reproducibility.
    """
    if seed is not None:
        random.seed(seed)

    gates_to_apply_info = []  # List of tuples (gate_name: str, target_qubits: List[int])

    # Append specified number of single-qubit Clifford gates
    for gate_name in SINGLE_QUBIT_CLIFFORDS:
        for _ in range(num_single_clifford_each):
            target = random.randrange(n_qubits)
            gates_to_apply_info.append((gate_name, [target]))

    # Append specified number of two-qubit Clifford gates
    if n_qubits >= 2:
        for gate_name in TWO_QUBIT_CLIFFORDS:
            for _ in range(num_two_clifford_each):
                q1, q2 = random.sample(range(n_qubits), 2)
                gates_to_apply_info.append((gate_name, [q1, q2]))

    # Append specified number of T-type gates
    for gate_name in T_TYPE_GATES:
        for _ in range(num_t_each):
            target = random.randrange(n_qubits)
            gates_to_apply_info.append((gate_name, [target]))

    # Shuffle the gates to randomize their order
    random.shuffle(gates_to_apply_info)

    qasm_lines = []
    qasm_lines.append("OPENQASM 2.0;")
    qasm_lines.append('include "qelib1.inc";')
    qasm_lines.append(f"qreg q[{n_qubits}];")

    for gate_name, qubits in gates_to_apply_info:
        qubit_args = ", ".join([f"q[{i}]" for i in qubits])
        qasm_lines.append(f"{gate_name} {qubit_args};")

    return "\n".join(qasm_lines)


def generate_pauli_strings(n: int) -> list[str]:
    """
    Generates all 4^n Pauli strings for n qubits.
    """
    if n < 1:
        return []

    paulis = ['I', 'X', 'Y', 'Z']

    # Calculate the Cartesian product
    # This yields tuples like ('I', 'I'), ('I', 'X'), ...
    product_iter = itertools.product(paulis, repeat=n)

    # Convert tuples to joined strings (e.g., "II", "IX")
    string_list = ["".join(pauli_tuple) for pauli_tuple in product_iter]

    return string_list


def run_validation(
    n_qubits: int,
    num_single_clifford_each: int,
    num_two_clifford_each: int,
    num_t_each: int,
    seed: Optional[int] = None
) -> None:
    qasm = generate_random_circuit_qasm(
        n_qubits,
        num_single_clifford_each,
        num_two_clifford_each,
        num_t_each,
        seed
    )

    necstar_circuit = NcQuantumCircuit.from_qasm_str(qasm)
    # Custom instructions are needed to properly handle SWAP gates in Qiskit
    qiskit_circuit = qasm2.loads(qasm, custom_instructions=qasm2.LEGACY_CUSTOM_INSTRUCTIONS)

    necstar_state = NcQuantumState.from_circuit(necstar_circuit)
    qiskit_state = Statevector(qiskit_circuit)

    pauli_strings = generate_pauli_strings(n_qubits)
    for pauli_str in pauli_strings:
        necstar_pauli = NcPauliString.from_str(pauli_str)
        qiskit_pauli = QiskitPauli(pauli_str)

        necstar_exp_value = necstar_state.exp_value(necstar_pauli)
        qiskit_exp_value = qiskit_state.expectation_value(qiskit_pauli).real

        assert abs(necstar_exp_value - qiskit_exp_value) < 1e-6, (
            f"Mismatch for Pauli string {pauli_str}: "
            f"necstar={necstar_exp_value}, qiskit={qiskit_exp_value}"
        )


if __name__ == "__main__":
    print(dir(NcQuantumGate))

    run_validation(
        n_qubits=3,
        num_single_clifford_each=2,
        num_two_clifford_each=2,
        num_t_each=2,
        seed=42
    )
