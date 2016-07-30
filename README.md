# quantum [![Build Status](https://travis-ci.org/beneills/quantum.svg?branch=master)](https://travis-ci.org/beneills/quantum) [![License](http://img.shields.io/:license-mit-blue.svg)](http://doge.mit-license.org)

_Advanced Rust quantum computer simulator._

## Motivation

Quantum is a quantum computer simulator written with the following design goals in mind:

1) __Utility:__ we can simulate a 5-qubit register, enough to run interesting algorithms.

2) __Efficiency:__ we implement all important operations from scratch using primitives

3) __Educational Value:__ the [documentation](http://beneills.github.io/quantum/) is a prose description of how to implement a quantum computer in Rust.

4) __Correctness:__ it's entirely written in safe Rust with well-defined states.

A good place to start reading the theory behind this is the [Wikipedia article](https://en.wikipedia.org/wiki/Quantum_computing#Mechanics), and you can follow the documentation for our implementations of [gates](https://beneills.github.io/quantum/quantum/gate/struct.Gate.html), [kets](https://beneills.github.io/quantum/quantum/ket/struct.Ket.html), [common operations](https://beneills.github.io/quantum/quantum/gates/index.html), and [quantum registers](https://beneills.github.io/quantum/quantum/registers/struct.QuantumRegister.html).

## Usage

```toml
# Cargo.toml

[dependencies]
quantum = "0.1.3"
```

```rust
// main.rs

use computer::QuantumComputer;
use algorithms::deutsch;
use gates;

// Let's do something simple of a 3-qubit system.
let mut c1 = QuantumComputer::new(3);
c1.initialize(5);
c1.apply(gates::identity(3));
c1.collapse();
assert_eq!(5, c1.value());

// Now let's perform a coin flip using the Hadamard transform.
let mut c2 = QuantumComputer::new(1);
c2.initialize(0);
c2.apply(gates::hadamard(1));
c2.collapse();
let result = if 1 == c2.value() { "heads" } else { "tails" };
println!("coin flip: {}", result);

// Finally let's determine whether f: {0, 1} -> {0, 1} is constant
// or balanced using Deutsch's algorithm.
// (see http://physics.stackexchange.com/q/3400)
let mut c3 = QuantumComputer::new(2);
c3.initialize(1);
c3.apply(gates::hadamard(2));
c3.apply(deutsch::deutsch_gate(f));
c3.apply(gates::hadamard(2));
c3.collapse();
let result = if 1 == c3.value() { "constant" } else { "balanced" };
println!("f is: {}", result);
```

## Gates

We provide the following quantum gates:

+ _Identity_
+ _Hadamard_
+ _Pauli-X_
+ _Pauli-Y_
+	_Pauli-Z_
+	_Phase Shifts_
+	_Swap_
+	_Sqrt(Swap)_
+	_Controlled Not_
+ _General Controlled-U_
+ _Controlled-X_
+ _Controlled-Y_
+ _Controlled-Z_
+	_Toffoli_
+	_Fredkin_

## Contributing

 - Create or take ownership of an issue
 - Fork _development_ branch
 - Write code and tests
 - `rust test`
 - Commit [with a reasonable message](http://chris.beams.io/posts/git-commit/) and push
 - Submit a pull request
