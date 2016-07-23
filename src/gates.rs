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
                   sqrt2inv,
                   sqrt2inv,
                   -sqrt2inv];

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
