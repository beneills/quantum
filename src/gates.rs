//! Implementations of quantum gates, intended for consumer use.

use gate::Gate;
use ket::Ket;
use matrix::Matrix;

/// The identity gate, not mutating the state at all.
#[allow(unused)]
pub fn identity(width: usize) -> Gate {
    let m: Matrix = Matrix::identity(Ket::size(width));

    Gate::new(width, m)
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
