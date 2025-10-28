from __future__ import annotations

from typing import List

class QuantumGate:
    """
    Represents a single quantum gate operation within a QuantumCircuit.

    This class is primarily used internally by QuantumCircuit and QuantumState.
    It provides information about the gate's name, the qubits it acts upon,
    and its properties (e.g., whether it's a Clifford gate).
    """
    @staticmethod
    def h(qubit: int) -> QuantumGate:
        """Creates an H gate acting on the specified qubit."""
        ...

    @staticmethod
    def x(qubit: int) -> QuantumGate:
        """Creates an X gate acting on the specified qubit."""
        ...

    @staticmethod
    def y(qubit: int) -> QuantumGate:
        """Creates a Y gate acting on the specified qubit."""
        ...

    @staticmethod
    def z(qubit: int) -> QuantumGate:
        """Creates a Z gate acting on the specified qubit."""
        ...

    @staticmethod
    def s(qubit: int) -> QuantumGate:
        """Creates an S gate acting on the specified qubit."""
        ...

    @staticmethod
    def sdg(qubit: int) -> QuantumGate:
        """Creates an S-dagger gate acting on the specified qubit."""
        ...

    @staticmethod
    def sqrt_x(qubit: int) -> QuantumGate:
        """Creates a sqrt-X gate acting on the specified qubit."""
        ...

    @staticmethod
    def sqrt_xdg(qubit: int) -> QuantumGate:
        """Creates a sqrt-X-dagger gate acting on the specified qubit."""
        ...

    @staticmethod
    def cx(control: int, target: int) -> QuantumGate:
        """Creates a CX (CNOT) gate acting on the specified control and target qubits."""
        ...

    @staticmethod
    def cz(qarg1: int, qarg2: int) -> QuantumGate:
        """Creates a CZ gate acting on the specified control and target qubits."""
        ...

    @staticmethod
    def swap(qarg1: int, qarg2: int) -> QuantumGate:
        """Creates a SWAP gate acting on the specified qubits."""
        ...

    @staticmethod
    def t(qubit: int) -> QuantumGate:
        """Creates a T gate acting on the specified qubit."""
        ...

    @staticmethod
    def tdg(qubit: int) -> QuantumGate:
        """Creates a T-dagger gate acting on the specified qubit."""
        ...

    @staticmethod
    def ccx(control1: int, control2: int, target: int) -> QuantumGate:
        """Creates a Toffoli (CCX) gate acting on the specified control and target qubits."""
        ...

    @property
    def name(self) -> str:
        """The name of the quantum gate (e.g., 'H', 'CX', 'T')."""
        ...

    @property
    def qubits(self) -> List[int]:
        """A list of qubit indices that the gate acts upon."""
        ...

    @property
    def is_clifford(self) -> bool:
        """Returns True if the gate is a Clifford gate, False otherwise."""
        ...

    @property
    def is_t_type(self) -> bool:
        """Returns True if the gate is a T or Tdg gate, False otherwise."""
        ...

    def __str__(self) -> str:
        """Returns a string representation of the gate (e.g., 'H(0)', 'CX(1, 2)')."""
        ...

    def __repr__(self) -> str:
        """Returns a developer-friendly representation of the QuantumGate object."""
        ...
