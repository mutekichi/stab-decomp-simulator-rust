from __future__ import annotations

from typing import List, Optional

from .gate import QuantumGate

class QuantumCircuit:
    """
    Represents a quantum circuit as a sequence of quantum gates.
    """

    def __init__(self, num_qubits: int) -> None:
        """Initializes a QuantumCircuit with the specified number of qubits.

        Args:
            num_qubits (int): The number of qubits in the circuit.
        """
        ...

    @property
    def num_qubits(self) -> int:
        """The number of qubits in the circuit."""
        ...

    @property
    def num_gates(self) -> int:
        """The total number of gates in the circuit."""
        ...

    @property
    def gates(self) -> List[QuantumGate]:
        """A list of :class:`~necstar.QuantumGate` objects representing the gates in
        the circuit."""
        ...

    @staticmethod
    def from_qasm_file(path: str) -> QuantumCircuit:
        """Parses an OpenQASM 2.0 file into a :class:`~necstar.QuantumCircuit` object.

        Args:
            path (str): The file path to the OpenQASM 2.0 file.

        Returns:
            QuantumCircuit: A :class:`~necstar.QuantumCircuit` object representing the
                parsed circuit.

        Raises:
            FileNotFoundError: If the specified file cannot be read.
            ValueError: If the QASM content is invalid or unsupported.
        """
        ...

    @staticmethod
    def from_qasm_str(qasm: str) -> QuantumCircuit:
        """Parses an OpenQASM 2.0 string into a :class:`~necstar.QuantumCircuit` object.

        Args:
            qasm (str): A string containing the OpenQASM 2.0 circuit description.

        Returns:
            QuantumCircuit: A :class:`~necstar.QuantumCircuit` object representing the
                parsed circuit.

        Raises:
            ValueError: If the QASM string is invalid or unsupported.
        """
        ...

    def to_qasm_str(self, reg_name: str) -> str:
        """Generates the OpenQASM 2.0 string representation of the circuit.

        Args:
            reg_name (str): The name of the quantum register (e.g., "q").

        Returns:
            str: A string containing the OpenQASM 2.0 representation of the circuit.
        """
        ...

    def to_qasm_file(self, path: str, reg_name: str) -> None:
        """Writes the circuit to an OpenQASM 2.0 file.

        Args:
            path (str): The path to the output file.
            reg_name (str): The name of the quantum register (e.g., "q").
        """
        ...

    @staticmethod
    def random_clifford(n: int, seed: Optional[int] = None) -> QuantumCircuit:
        """Generates a random n-qubit Clifford circuit using the canonical form
        decomposition presented in Ref. [1].

        This function implements the O(n^2) algorithm described in the paper
        to sample a Clifford operator uniformly at random from the n-qubit
        Clifford group. The resulting circuit is structured according to the
        canonical form U = F1 * H * S * F2, where F1 and F2 are Hadamard-free
        Clifford circuits, H is a layer of Hadamard gates, and S is a permutation
        of qubits.

        ### References:
            [1] S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure\
                of the Clifford   group," arXiv:2003.09412v2 (2021).

        Args:
            n (int): The number of qubits. Must be greater than 0.
            seed (Optional[int]): An optional seed for the random number generator
                for reproducibility. Defaults to None (uses system entropy).

        Returns:
            QuantumCircuit: A :class:`~necstar.QuantumCircuit` object representing the
            random Clifford operator.
        """
        ...

    def append(self, other: QuantumCircuit) -> None:
        """Appends the gates from another :class:`~necstar.QuantumCircuit` to this one.

        Args:
            other (QuantumCircuit): The circuit whose gates will be appended.
                The number of qubits must match.
        """
        ...

    def tensor(self, other: QuantumCircuit) -> QuantumCircuit:
        """Creates a new circuit by taking the tensor product of this circuit and
        another.

        The new circuit will have `self.num_qubits + other.num_qubits` qubits.
        Gates from `self` are applied to the first qubits (indices 0 to
        `self.num_qubits` - 1), and gates from `other` are applied to the subsequent
        qubits (indices `self.num_qubits` onwards).

        Args:
            other (QuantumCircuit): The circuit to tensor with this one.

        Returns:
            QuantumCircuit: A new :class:`~necstar.QuantumCircuit` representing the
            tensor product.
        """
        ...

    def apply_gate(self, gate_name: str, qubits: List[int]) -> None:
        """Applies a quantum gate to the circuit by name.

        Args:
            gate_name (str): The name of the gate (case-insensitive).
                Supported names include: `'h'`, `'x'`, `'y'`, `'z'`, `'s'`, `'sdg'`,
                `'sqrtx'`, `'sqrtxdg'`, `'t'`, `'tdg'`, `'cx'`, `'cz'`, `'swap'`,
                `'ccx'`.
            qubits (List[int]): A list of qubit indices the gate acts upon.
                The number of indices must match the gate's arity
                (e.g., 1 for `'h'`, 2 for `'cx'`, 3 for `'ccx'`).
                For multi-qubit gates, the order is generally control qubits
                followed by target qubits.

        Raises:
            ValueError: If the gate name is unknown or the number of qubit
                indices is incorrect for the gate.
        """
        ...

    def apply_h(self, target: int) -> None:
        """Applies a Hadamard gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_x(self, target: int) -> None:
        """Applies a Pauli-X gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_y(self, target: int) -> None:
        """Applies a Pauli-Y gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_z(self, target: int) -> None:
        """Applies a Pauli-Z gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_s(self, target: int) -> None:
        """Applies an S gate (Phase gate) to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_sdg(self, target: int) -> None:
        """Applies an S-dagger gate (conjugate Phase gate) to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_sqrt_x(self, target: int) -> None:
        """Applies a square root of X gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_sqrt_xdg(self, target: int) -> None:
        """Applies a square root of X-dagger gate to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_t(self, target: int) -> None:
        """Applies a T gate (π/8 gate) to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_tdg(self, target: int) -> None:
        """Applies a T-dagger gate (conjugate π/8 gate) to the target qubit.

        Args:
            target (int): The target qubit index.
        """
        ...

    def apply_cx(self, control: int, target: int) -> None:
        """Applies a CNOT (Controlled-X) gate.

        Args:
            control (int): The control qubit index.
            target (int): The target qubit index.
        """
        ...

    def apply_cz(self, qarg1: int, qarg2: int) -> None:
        """Applies a CZ (Controlled-Z) gate.

        Args:
            qarg1 (int): The index of the first qubit.
            qarg2 (int): The index of the second qubit.
        """
        ...

    def apply_swap(self, qarg1: int, qarg2: int) -> None:
        """Applies a SWAP gate.

        Args:
            qarg1 (int): The index of the first qubit.
            qarg2 (int): The index of the second qubit.
        """
        ...

    def apply_ccx(self, control1: int, control2: int, target: int) -> None:
        """Applies a Toffoli (CCX) gate.

        Args:
            control1 (int): The index of the first control qubit.
            control2 (int): The index of the second control qubit.
            target (int): The index of the target qubit.
        """
        ...

    def __str__(self) -> str:
        """Returns a string representation of the circuit summary."""
        ...
