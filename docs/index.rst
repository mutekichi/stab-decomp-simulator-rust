NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust (Python bindings) 
=====================

A high-performance quantum circuit simulator designed for the strong simulation of
near-Clifford circuits based on the stabilizer decomposition method [1].

NECSTAR is particularly effective for circuits dominated by Clifford gates
but also containing a small number of non-Clifford gates. Currently, NECSTAR supports
only T-gates as non-Clifford operations, but future versions may include additional
non-Clifford gates.

Features
--------

* **Stabilizer Decomposition Core**: The simulator represents the quantum state as a linear combination of stabilizer states [1]. This approach avoids the memory overhead of dense state vectors and is efficient for circuits with low non-Clifford gate counts.
* **Magic State Teleportation**: Non-Clifford gates are applied via the gate teleportation protocol using magic states. A T-gate is implemented by consuming a T-state, and the tensor product of T-states is automatically decomposed into stabilizer states using optimized techniques [2].
* **Intuitive Declarative API**: Users can define quantum computations by building a :class:`~necstar.QuantumCircuit`. This is compiled into a :class:`~necstar.QuantumState`, which manages the internal stabilizer decomposition and provides a clean interface for simulation.

References
----------

* [1] S. Bravyi, D. Browne, P. Calpin, E. Campbell, D. Gosset, and M. Howard, "Simulation of quantum circuits by low-rank stabilizer decompositions", Quantum 3, 181 (2019). `<https://doi.org/10.22331/q-2019-09-02-181>`_
* [2] H. Qassim, H. Pashayan, and D. Gosset, "Improved upper bounds on the stabilizer rank of magic states", Quantum 5, 604 (2021). `<https://doi.org/10.22331/q-2021-12-20-606>`_

Typical Workflow
----------------

1. Construct a quantum circuit using :class:`~necstar.QuantumCircuit`.
2. Compile the circuit into a :class:`~necstar.QuantumState` using :meth:`~necstar.QuantumState.from_circuit`.
3. Perform operations such as :meth:`~necstar.QuantumState.measure`, :meth:`~necstar.QuantumState.sample`, or :meth:`~necstar.QuantumState.exp_value`.

Example
-------

.. code-block:: python

    import necstar

    # 1. Build a quantum circuit
    qc = necstar.QuantumCircuit(2)
    qc.apply_h(0)
    qc.apply_cx(0, 1)
    qc.apply_t(1) # Non-Clifford T-gate

    # 2. Compile into a QuantumState (internally decomposes T-states)
    state = necstar.QuantumState.from_circuit(qc)

    # 3. Perform operations
    shots = 1024
    samples = state.sample(qargs=[0, 1], shots=shots)
    print(f"Samples: {samples}")

    pauli_z0 = necstar.PauliString.from_str("ZI")
    exp_val = state.exp_value(pauli_z0)
    print(f"Expectation value: {exp_val}")

    # (Optional) Check the stabilizer rank
    print(f"Stabilizer rank: {state.stabilizer_rank}")

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   api/index

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`