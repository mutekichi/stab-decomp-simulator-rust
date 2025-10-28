from __future__ import annotations

class PauliString:
    """
    Represents a multi-qubit Pauli operator.

    Can be initialized from a string in either dense ("IXYZ") or sparse ("X1 Y3") format.
    The identity operator can be represented by an empty string "" or "I".
    """

    @property
    def is_identity(self) -> bool:
        """Checks if the Pauli string represents the identity operator."""
        ...

    @staticmethod
    def from_str(s: str) -> PauliString:
        """Creates a :class:`~necstar.PauliString` object from a string representation. Supports:

        1. Dense format: e.g., "IXYZ" (case-sensitive).
        2. Sparse format: e.g., "X1 Y3 Z0" (case-insensitive for Pauli chars, space-separated).
        3. Identity: "" or "I" (case-insensitive).

        Args:
            s (str): The string representation.

        Returns:
            PauliString: The corresponding PauliString object.

        Raises:
            ValueError: If the string format is invalid.
        """
        ...

    def __str__(self) -> str:
        """Returns the string representation of the Pauli operator."""
        ...

    def __repr__(self) -> str:
        """Returns a printable representation of the :class:`~necstar.PauliString` object."""
        ...
