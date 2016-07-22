use ket::Ket;
use matrix::Matrix;

/// Represents a _quantum gate_: a quantum regster transformation.
///
/// This gate is tagged with a width, and contains a unitary matrix
/// representing the numerical transformation in the computational
/// basis.
///
/// This gate may be _applied_ to a ket to update the ket's state.
///
/// Currently we do not check whether the matrix is unitary.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_computing#Operation)
/// for more information.
pub struct Gate {
    pub width: usize,
    pub matrix: Matrix
}

impl Gate {

    /// Construct a new quantum gate, given _width_ and computational basis matrix.
    ///
    /// Currently we do not check whether the matrix is unitary.
    ///
    /// # Panics
    ///
    /// We panic if the supplied matrix is non-square or not of dimension _width_.
    pub fn new(width: usize, matrix: Matrix) -> Gate {
        assert_eq!(Ket::size(width), matrix.size);

        // TODO check that det(matrix) == 1

        Gate {
            width: width,
            matrix: matrix
        }
    }
}
