NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust (Python bindings) 
=====================

A high-performance quantum circuit simulator designed for the strong simulation of near-Clifford circuits.
NECSTAR is particularly effective for circuits dominated by Clifford gates but also contain a small number of non-Clifford gates, such as the T-gate.
It provides an intuitive API for building and simulating quantum circuits.

Features
--------

* **Stabilizer Decomposition Core**: The simulator's engine is built on the stabilizer decomposition method. Instead of representing the state vector in a memory-intensive dense vector, it maintains the quantum state as a linear combination of stabilizer states.
* **Magic State Teleportation**: Non-Clifford gates are handled using the gate teleportation protocol. The required magic states (e.g., T-states) are represented using stabilizer decompositions.
* **Intuitive Declarative API**: Users can define quantum computations by building a :class:`~necstar.QuantumCircuit`. This is compiled into a :class:`~necstar.QuantumState`, which abstracts away the complex internal state representation.
* **Strong, Exact Simulation**: Necstar performs strong simulation, calculating the full final quantum state with exact amplitudes without approximations.

Typical Workflow
----------------

1.  Construct a quantum circuit using :class:`~necstar.QuantumCircuit`.
2.  Compile the circuit into a :class:`~necstar.QuantumState` using :meth:`~necstar.QuantumState.from_circuit`.
3.  Perform operations such as :meth:`~necstar.QuantumState.measure`, :meth:`~necstar.QuantumState.sample`, or :meth:`~necstar.QuantumState.exp_value`.

Example
-------

.. code-block:: python

    import necstar

    # 1. Build a quantum circuit
    qc = necstar.QuantumCircuit(2)
    qc.apply_h(0)
    qc.apply_cx(0, 1)
    qc.apply_t(1)

    # 2. Compile into a state
    state = necstar.QuantumState.from_circuit(qc)

    # 3. Perform operations
    samples = state.sample(qargs=[0, 1], shots=1024)
    print(f"Samples: {samples}")

    pauli_z0 = necstar.PauliString.from_str("ZI")
    exp_val = state.exp_value(pauli_z0)
    print(f"Expectation value of Z0: {exp_val}")

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   api/index

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`