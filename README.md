# quantum [![Build Status](https://travis-ci.org/beneills/quantum.svg?branch=master)](https://travis-ci.org/beneills/quantum) [![License](http://img.shields.io/:license-mit-blue.svg)](http://doge.mit-license.org)

_Advanced Rust quantum computer simulator._

## Motivation

Quantum is a quantum computer simulator written with the following design goals in mind:

1) __Utility:__ we can simulate a 5-qubit register, enough to run interesting algorithms.

2) __Efficiency:__ we implement all important operations from scratch using primitives

3) __Educational Value:__ the [documentation](http://beneills.github.io/quantum/) is a prose description of how to implement a quantum computer in Rust.

4) __Correctness:__ it's entirely written in safe Rust with well-defined states.

A good place to start reading the theory behind this is the [Wikipedia article](https://en.wikipedia.org/wiki/Quantum_computing#Mechanics), and you can follow the documentation for our implementations of

[gates](https://beneills.github.io/quantum/quantum/gate/struct.Gate.html)
[kets](https://beneills.github.io/quantum/quantum/ket/struct.Ket.html)

[](https://beneills.github.io/quantum/quantum/other/qubit/index.html)
.

## Usage

Add `quantum` dependency to `Cargo.toml`.

```rust
use computer::QuantumComputer;
use gates;

let mut c = QuantumComputer::new(3);
c.initialize(5);
c.apply(gates::identity(3));
c.collapse();

assert_eq!(5, c.value());
```

## Contributing

 - Create or take ownership of an issue
 - Fork _development_ branch
 - Write code and tests
 - `rust test`
 - Commit [with a reasonable message](http://chris.beams.io/posts/git-commit/) and push
 - Submit a pull request
