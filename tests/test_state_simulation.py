import numpy as np
import pytest
from necstar import QuantumCircuit, QuantumState


def make_toffoli_state():
    """Creates a "Toffoli state":
    |Toffoli> = (|000> + |100> + |010> + |111>) / 2
    """
    circuit = QuantumCircuit(3)
    # Initialize control qubits to uniform superposition
    circuit.apply_h(0)
    circuit.apply_h(1)
    # Apply Toffoli gate using 7 t-type gates
    circuit.apply_h(2)
    circuit.apply_cx(1, 2)
    circuit.apply_tdg(2)
    circuit.apply_cx(0, 2)
    circuit.apply_t(2)
    circuit.apply_cx(1, 2)
    circuit.apply_tdg(2)
    circuit.apply_cx(0, 2)

    circuit.apply_t(1)
    circuit.apply_t(2)
    circuit.apply_cx(0, 1)
    circuit.apply_h(2)

    circuit.apply_t(0)
    circuit.apply_tdg(1)
    circuit.apply_cx(0, 1)

    state = QuantumState.from_circuit(circuit)
    return state


def test_state_sampling():
    toffoli_state = make_toffoli_state()
    shots = 10000
    counts = toffoli_state.sample([i for i in range(3)], shots, seed=42)
    expected_outcomes = {"000", "100", "010", "111"}
    for outcome in expected_outcomes:
        assert outcome in counts, f"Expected outcome {outcome} not found in counts."
        frequency = counts[outcome] / shots
        assert (
            sum(counts.values()) == shots
        ), "Total counts do not match number of shots."
        assert (
            abs(frequency - 0.25) < 0.05
        ), f"Frequency of outcome {outcome} deviates from expected 0.25."

    counts_reverse = toffoli_state.sample([i for i in range(2, -1, -1)], shots, seed=40)
    expected_outcomes_reverse = {"000", "001", "010", "111"}
    for outcome in expected_outcomes_reverse:
        assert (
            outcome in counts_reverse
        ), f"Expected outcome {outcome} not found in counts."

    with pytest.raises(ValueError):
        toffoli_state.sample([0, 1, 2, 3], shots)  # Invalid qubit index
    with pytest.raises(ValueError):
        toffoli_state.sample([0, 0], shots)  # Duplicate qubit index
    with pytest.raises(ValueError):
        toffoli_state.sample([], shots)  # Empty qubit list


def test_state_measurement():
    trials = 20
    for i in range(trials):
        toffoli_state = make_toffoli_state()
        measured_bits = toffoli_state.measure([0, 1, 2], seed=i)
        outcome_str = "".join(map(str, map(int, measured_bits)))
        expected_outcomes = {"000", "100", "010", "111"}
        assert (
            outcome_str in expected_outcomes
        ), f"Unexpected measurement result: {outcome_str}"

        statevector = toffoli_state.to_statevector()
        non_zero_indices = np.where(np.abs(statevector) > 1e-10)[0]
        assert (
            len(non_zero_indices) == 1
        ), "Statevector is not one-hot after measurement."
        assert np.isclose(
            np.abs(statevector[non_zero_indices[0]]), 1.0
        ), "Amplitude is not 1.0."

        remeasured_outcome = toffoli_state.measure([0, 1, 2])
        assert list(measured_bits) == list(
            remeasured_outcome
        ), "Re-measurement of collapsed state failed."

    with pytest.raises(ValueError):
        toffoli_state = make_toffoli_state()
        toffoli_state.measure([0, 0])  # Duplicate qubit index
    with pytest.raises(ValueError):
        toffoli_state = make_toffoli_state()
        toffoli_state.measure([0, 1, 10])  # Out of range
