from necstar import QuantumGate


def test_gate_initialization():
    gate_x = QuantumGate.x(0)
    assert gate_x.name == "X"
    assert gate_x.qubits == [0]
    assert gate_x.name == "X"
    assert str(gate_x) == "X(0)"
    assert repr(gate_x) == "<QuantumGate: X(0)>"

    gate_cx = QuantumGate.cx(1, 2)
    assert gate_cx.name == "CX"
    assert gate_cx.qubits == [1, 2]
    assert str(gate_cx) == "CX(1, 2)"
    assert repr(gate_cx) == "<QuantumGate: CX(1, 2)>"


def test_clifford_gates():
    clifford_gates = [
        QuantumGate.h(0),
        QuantumGate.x(1),
        QuantumGate.y(2),
        QuantumGate.z(3),
        QuantumGate.s(4),
        QuantumGate.sdg(5),
        QuantumGate.sqrt_x(6),
        QuantumGate.sqrt_xdg(7),
        QuantumGate.cx(8, 9),
        QuantumGate.cz(10, 11),
        QuantumGate.swap(12, 13),
    ]
    assert all(gate.is_clifford() for gate in clifford_gates)
    assert not any(gate.is_t_type() for gate in clifford_gates)


def test_t_type_gates():
    t_gates = [
        QuantumGate.t(0),
        QuantumGate.tdg(1),
    ]
    assert all(gate.is_t_type() for gate in t_gates)
    assert not any(gate.is_clifford() for gate in t_gates)
