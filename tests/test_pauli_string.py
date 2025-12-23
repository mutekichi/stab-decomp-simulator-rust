import pytest
from necstar import PauliString


def test_pauli_string_initialization_dense():
    pauli = PauliString.from_str("IXYZ")
    assert str(pauli) == "IXYZ"
    assert repr(pauli) == "PauliString('IXYZ')"
    assert not pauli.is_identity

    with pytest.raises(ValueError):
        PauliString.from_str("IXYZA")  # Invalid character
    with pytest.raises(ValueError):
        PauliString.from_str("IXY1")  # Invalid character
    with pytest.raises(ValueError):
        PauliString.from_str("I X Y Z")  # Spaces not allowed in dense format
    with pytest.raises(ValueError):
        PauliString.from_str("xyz")  # Lowercase not allowed in dense format


def test_pauli_string_initialization_sparse():
    pauli = PauliString.from_str("X1 Y3 Z0")
    assert str(pauli) == "X1 Y3 Z0"
    assert repr(pauli) == "PauliString('X1 Y3 Z0')"
    assert not pauli.is_identity

    pauli_lower = PauliString.from_str("x1 y3 z0")
    assert str(pauli_lower) == "X1 Y3 Z0"
    assert repr(pauli_lower) == "PauliString('X1 Y3 Z0')"

    with pytest.raises(ValueError):
        PauliString.from_str("X1 Y3 A0")  # Invalid character
    with pytest.raises(ValueError):
        PauliString.from_str("X1 Y3 Z")  # Missing index
    with pytest.raises(ValueError):
        PauliString.from_str("X1 Y3 Z-1")  # Negative index
    with pytest.raises(ValueError):
        PauliString.from_str("X1 Y3 Z1.5")  # Non-integer index
    with pytest.raises(ValueError):
        PauliString.from_str("X1 Y3 Z1")  # Duplicate index


def test_pauli_string_identity():
    pauli_empty = PauliString.from_str("")
    assert str(pauli_empty) == "I"
    assert repr(pauli_empty) == "PauliString('I')"
    assert pauli_empty.is_identity

    pauli_i = PauliString.from_str("I")
    assert str(pauli_i) == "I"
    assert repr(pauli_i) == "PauliString('I')"
    assert pauli_i.is_identity
