from __future__ import annotations

from typing import Dict, List, Optional, Tuple

from .circuit import QuantumCircuit
from .gate import QuantumGate
from .pauli_string import PauliString

class QuantumState:
    """
    Represents a simulated quantum state using the stabilizer decomposition method.

    This class provides the primary interface for simulating quantum computations
    defined by a :class:`~necstar.QuantumCircuit`. It encapsulates the internal state representation
    and offers methods for performing measurements, sampling, calculating expectation
    values, and applying Clifford gates directly.
    """

    @property
    def num_qubits(self) -> int:
        """The number of qubits in the quantum state."""
        ...

    @property
    def stabilizer_rank(self) -> int:
        """The stabilizer rank χ (number of stabilizer states in the decomposition)."""
        ...

    @staticmethod
    def from_circuit(circuit: QuantumCircuit) -> QuantumState:
        """Creates a new :class:`~necstar.QuantumState` by compiling a :class:`~necstar.QuantumCircuit`.

        Args:
            circuit (QuantumCircuit): The quantum circuit to be simulated.

        Returns:
            QuantumState: The compiled quantum state ready for simulation.

        Raises:
            ValueError: If the circuit compilation fails (e.g., contains unsupported gates).
        """
        ...

    def to_statevector(self) -> List[Tuple[float, float]]:
        """Returns the statevector as a list of complex number tuples (real, imag).

        Note:
            This function computes the full, dense statevector of size :math:`2^n`, which
            can be computationally expensive and memory-intensive for a large
            number of qubits (:math:`n`). It is primarily intended for testing and debugging.
            The indexing follows the little-endian convention (like Qiskit).

        Returns:
            List[Tuple[float, float]]: The statevector, where each tuple represents
            the real and imaginary parts of an amplitude.

        Raises:
            ValueError: If the statevector calculation fails (e.g., too many qubits).
        """
        ...

    def inner_product(self, other: QuantumState) -> Tuple[float, float]:
        """Computes the inner product :math:`\\langle\\text{self}|\\text{other}\\rangle`
        between this state and another.

        Args:
            other (QuantumState): The other quantum state. Must have the same
                number of qubits.

        Returns:
            Tuple[float, float]: The inner product as a tuple (real, imag).

        Raises:
            ValueError: If the inner product calculation fails (e.g., qubit count mismatch).
        """
        ...

    def measure(self, qargs: List[int], seed: Optional[int] = None) -> List[bool]:
        """Measures the specified qubits in the computational basis. The state collapses according
        to the measurement results.

        Args:
            qargs (List[int]): A list of qubit indices to measure.
            seed (Optional[int]): An optional seed for the random number generator
                to ensure reproducibility. Defaults to None.

        Returns:
            List[bool]: A list of boolean measurement outcomes (False for :math:`|0\\rangle`,
            True for :math:`|1\\rangle`).

        Raises:
            ValueError: If measurement fails (e.g., invalid qubit index).
        """
        ...

    def measure_all(self, seed: Optional[int] = None) -> List[bool]:
        """Measures all qubits in the computational basis.

        The state collapses according to the measurement results.

        Args:
            seed (Optional[int]): An optional seed for the random number generator
            to ensure reproducibility. Defaults to None.

        Returns:
            List[bool]: A list of boolean measurement outcomes for all qubits.

        Raises:
            ValueError: If measurement fails.
        """
        ...

    def sample(self, qargs: List[int], shots: int, seed: Optional[int] = None) -> Dict[str, int]:
        """Samples measurement outcomes for the specified qubits without collapsing the state.

        This method efficiently gathers measurement statistics without modifying the
        internal state, making it suitable for repeated analysis. It uses a
        recursive approach with binomial sampling for efficiency with large shot counts.

        Args:
            qargs (List[int]): A list of qubit indices to sample.
            shots (int): The number of measurement samples to generate.
            seed (Optional[int]): An optional seed for the random number generator
                for reproducibility. Defaults to None.

        Returns:
            Dict[str, int]: A dictionary mapping outcome bitstrings (e.g., "010")
            to the number of times that outcome was observed.

        Raises:
            ValueError: If sampling fails (e.g., invalid qubit index).
        """
        ...

    def exp_value(self, pauli_string: PauliString) -> float:
        """Calculates the expectation value of a given Pauli observable.

        Args:
            pauli_string (PauliString): The Pauli string representing the observable.

        Returns:
            float: The expectation value :math:`\\langle\\psi|P|\\psi\\rangle`, where
            P is the Pauli operator.

        Raises:
            ValueError: If the Pauli string is invalid or the calculation fails.
        """
        ...

    def project_normalized(self, qubit: int, outcome: bool) -> None:
        """Projects the state onto a computational basis state for a specific qubit and normalizes.

        This is equivalent to a projective measurement in the Z-basis. The state
        is modified in place.

        Args:
            qubit (int): The index of the qubit to project.
            outcome (bool): The desired basis state (False for :math:`|0\\rangle`, True for :math:`|1\\rangle`).

        Raises:
            ValueError: If the projection is impossible (e.g., projecting :math:`|0\\rangle` onto :math:`|1\\rangle`).
        """
        ...

    def project_unnormalized(self, qubit: int, outcome: bool) -> None:
        """Projects the state onto a computational basis state without normalizing.

        The state is modified in place. The total norm after this operation may
        not be 1. Useful for intermediate steps in algorithms like sampling.

        Args:
            qubit (int): The index of the qubit to project.
            outcome (bool): The desired basis state (False for :math:`|0\\rangle`, True for :math:`|1\\rangle`).

        Raises:
            ValueError: If the projection operation encounters an internal error,
            though it won't raise an error for impossible projections resulting
            in a zero-norm state.
        """
        ...

    def discard(self, qubit: int) -> None:
        """Discards a qubit from the quantum state by tracing it out.

        Reduces the total number of qubits by one and modifies the state in place.

        Important:
            This function MUST only be called on a qubit that has been projected
            to the :math:`|0\\rangle` state and is disentangled from all others. Failure to meet
            this precondition will lead to incorrect results. No verification is performed
            for performance reasons. Use `project_normalized(qubit, False)` first if needed.

        Args:
            qubit (int): The index of the qubit to discard.

        Raises:
            ValueError: If the qubit index is out of bounds or an internal error occurs.
        """
        ...

    # --- Gate Applications ---

    def apply_gate(self, gate: QuantumGate) -> None:
        """Applies a :class:`~necstar.QuantumGate` directly to the quantum state.

        Note:
            Only Clifford gates are supported for direct application. Attempting
            to apply non-Clifford gates (e.g., T or T-dagger) will raise an error.

        Args:
            gate (QuantumGate): The quantum gate to apply.

        Raises:
            ValueError: If the gate application fails (e.g., unsupported gate).
        """
        ...

    def apply_gates(self, gates: List[QuantumGate]) -> None:
        """Applies a list of :class:`~necstar.QuantumGate` s directly to the quantum state.

        Note:
            Only Clifford gates are supported for direct application. Attempting
            to apply non-Clifford gates (e.g., T or T-dagger) will raise an error.

        Args:
            gates (List[QuantumGate]): The list of quantum gates to apply in sequence.

        Raises:
            ValueError: If any gate application fails (e.g., unsupported gate).
        """
        ...

    def apply_x(self, qubit: int) -> None:
        """Applies a Pauli-X gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails (e.g., invalid index).
        """
        ...

    def apply_y(self, qubit: int) -> None:
        """Applies a Pauli-Y gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_z(self, qubit: int) -> None:
        """Applies a Pauli-Z gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_h(self, qubit: int) -> None:
        """Applies a Hadamard gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_s(self, qubit: int) -> None:
        """Applies an S gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_sdg(self, qubit: int) -> None:
        """Applies an S-dagger gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_sqrt_x(self, qubit: int) -> None:
        """Applies a Sqrt(X) gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_sqrt_xdg(self, qubit: int) -> None:
        """Applies a Sqrt(X)-dagger gate directly to the state.

        Args:
            qubit (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_cx(self, control: int, target: int) -> None:
        """Applies a CNOT (CX) gate directly to the state.

        Args:
            control (int): The control qubit index.
            target (int): The target qubit index.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_cz(self, qarg1: int, qarg2: int) -> None:
        """Applies a CZ gate directly to the state.

        Args:
            qarg1 (int): The index of the first qubit.
            qarg2 (int): The index of the second qubit.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    def apply_swap(self, qarg1: int, qarg2: int) -> None:
        """Applies a SWAP gate directly to the state.

        Args:
            qarg1 (int): The index of the first qubit.
            qarg2 (int): The index of the second qubit.

        Raises:
            ValueError: If the gate application fails.
        """
        ...

    # --- Properties ---

    def norm(self) -> float:
        """Calculates the norm (magnitude) of the quantum state.

        For a valid, normalized state, this should be close to 1.0.

        Returns:
            float: The norm of the state.

        Raises:
            ValueError: If the norm calculation fails.
        """
        ...

    def __str__(self) -> str:
        """Returns a string representation of the quantum state summary."""
        ...
