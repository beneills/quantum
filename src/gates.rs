//! Implementations of quantum gates, intended for consumer use.

use complex::Complex;

use gate::Gate;
use ket::Ket;
use matrix::Matrix;

/// The identity gate, not mutating the state at all.
#[allow(unused)]
pub fn identity(width: usize) -> Gate {
    let m = Matrix::identity(Ket::size(width));

    Gate::new(width, m)
}

/// The Hadamard gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
/// for more information.
#[allow(unused)]
pub fn hadamard() -> Gate {
    let sqrt2inv = c![2.0f64.sqrt().recip(), 0f64];

    let mut m = m![sqrt2inv,
                   sqrt2inv;
                   sqrt2inv,
                   -sqrt2inv];

    Gate::new(1, m)
}

/// The Pauli-X gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-X_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_x() -> Gate {
    let m = m![Complex::zero(),
               Complex::one();
               Complex::one(),
               Complex::zero()];

    Gate::new(1, m)
}

/// The Pauli-Y gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Y_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_y() -> Gate {
    let m = m![Complex::zero(),
               -Complex::i();
               Complex::i(),
               Complex::zero()];

    Gate::new(1, m)
}

/// The Pauli-Z gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Z_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_z() -> Gate {
    let m = m![Complex::one(),
               Complex::zero();
               Complex::zero(),
               -Complex::one()];

    Gate::new(1, m)
}

/// A single qubit phase-shift gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Phase_shift_gates)
/// for more information.
#[allow(unused)]
pub fn phase_shift(phi: f64) -> Gate {
    let m = m![Complex::one(),
               Complex::zero();
               Complex::zero(),
               Complex::new_euler(1f64, phi)];

    Gate::new(1, m)
}

#[test]
fn identity_test() {
    use complex::Complex;

    let id_gate = identity(3);
    let mut ket = Ket::new(8);
    ket.elements[5] = c![99f64, 0f64];

    let expected = ket.clone();

    ket.apply(id_gate);

    assert_eq!(expected, ket);
}

#[test]
fn hadamard_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    let mut apply_hadamard = || {
        c.initialize(0);
        c.apply(hadamard());
        c.collapse();
        let v = c.value();
        c.reset();

        v
    };

    let mut ones = 0;

    for _ in 0..1000 {
        if 1 == apply_hadamard() {
            ones += 1;
        }
    }

    assert!( ones <= 600 && 400 <= ones)
}

#[test]
fn pauli_x_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to |1>
    c.initialize(0);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(1, c.value());
    c.reset();

    // |1> goes to |0>
    c.initialize(1);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(0, c.value());
}

#[test]
fn pauli_y_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to i|1>
    c.initialize(0);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(1, c.value());
    c.reset();

    // |1> goes to -i|0>
    c.initialize(1);
    c.apply(pauli_y());
    c.collapse();
    assert_eq!(0, c.value());
}

#[test]
fn pauli_z_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to |0>
    c.initialize(0);
    c.apply(pauli_z());
    c.collapse();
    assert_eq!(0, c.value());
    c.reset();

    // |1> goes to -|1>
    c.initialize(1);
    c.apply(pauli_z());
    c.collapse();
    assert_eq!(1, c.value());
}

#[test]
fn phase_shift_test() {
    use computer::QuantumComputer;

    let phi = 0.3f64;
    let mut c = QuantumComputer::new(1);

    // |0> goes to |0>
    c.initialize(0);
    c.apply(phase_shift(phi));
    c.collapse();
    assert_eq!(0, c.value());
    c.reset();

    // |1> goes to exp(i * phi)|1>
    c.initialize(1);
    c.apply(phase_shift(phi));
    c.collapse();
    assert_eq!(1, c.value());
}
