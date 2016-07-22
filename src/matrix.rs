//! Matrix library code (public for pedagogical reasons).

use std::fmt;
use std::ops::Add;
use std::ops::Mul;

use complex::Complex;

/// Max size of matrix and therefore ket.
pub const MAX_SIZE: usize = 32;

const MAX_ELEMENTS: usize = MAX_SIZE * MAX_SIZE;

/// Efficient array of complex numbers.
pub type Vector = [Complex; MAX_SIZE];

/// Represents a square matrix over C of maximum size _MAX_SIZE_.
///
/// Each element is an instance of _Complex_, and we store the elements
/// internally in an array of size _MAX_SIZE^2 * sizeof(Complex)_.
///
/// In practice, this means each matrix occupies around 16KiB.
#[allow(missing_copy_implementations)]
pub struct Matrix {
    size: usize,
    elements: [Complex; MAX_ELEMENTS],
}

impl Matrix {
    /// Construct a new zero-initialized matrix of given size.
    ///
    /// # Panics
    ///
    /// We panic if the given size exceeds _MAX_SIZE_.
    pub fn new(size: usize) -> Matrix {
        assert!(size <= MAX_SIZE);

        Matrix {
            size: size,
            elements: [Complex::zero(); MAX_ELEMENTS],
        }
    }

    /// Construct a new identity matrix of given size.
    ///
    /// # Panics
    ///
    /// We panic if the given size exceeds _MAX_SIZE_.
    pub fn identity(size: usize) -> Matrix {
        assert!(size <= MAX_SIZE);

        let mut elements = [Complex::zero(); MAX_ELEMENTS];

        for i in 0..size {
            elements[i * MAX_SIZE + i] = Complex::one();
        }

        Matrix {
            size: size,
            elements: elements,
        }
    }

    /// Size of the matrix.
    pub fn size(self: &Matrix) -> usize {
        self.size
    }

    /// Get the element in position _(i, j)_.
    pub fn get(self: &Matrix, i: usize, j: usize) -> Complex {
        self.elements[i * MAX_SIZE + j]
    }

    /// Set the element in position _(i, j)_ to _value_.
    pub fn set(self: &mut Matrix, i: usize, j: usize, value: Complex) {
        self.elements[i * MAX_SIZE + j] = value
    }
}

/// Display the matrix for debug purposes.
impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(size={}, elements=...)", self.size)
    }
}

/// Implement matrix equality comparison.
impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        assert_eq!(self.size, other.size);

        for i in 0..MAX_ELEMENTS {
            if self.elements[i] != other.elements[i] {
                return false;
            }
        }

        true
    }
}

/// Implement standard matrix addition.
impl<'a> Add<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn add(self, rhs: &'a Matrix) -> Matrix {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                m.set(i, j, self.get(i, j) + rhs.get(i, j));
            }
        }

        m
    }
}

/// Implement standard matrix multiplication.
impl<'a> Mul<&'a Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.size, rhs.size);

        let mut m = Matrix::new(self.size);

        for i in 0..self.size {
            for j in 0..self.size {
                let mut val = Complex::zero();

                for k in 0..self.size {
                    val += self.get(i, k) * rhs.get(k, j)
                }

                m.set(i, j, val);
            }
        }

        m
    }
}

/// Implement standard matrix vector multiplication.
///
/// # Panics
///
/// We panic if the vector contains non-zero elements in
/// positions _self.size_ or beyond.
impl<'a> Mul<&'a Vector> for &'a Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        let mut output = [Complex::zero(); MAX_SIZE];

        // Check that vector tail is zero
        for i in self.size..MAX_SIZE {
            assert_eq!(Complex::zero(), rhs[i])
        }

        for i in 0..self.size {
            let mut val = Complex::zero();

            for k in 0..self.size {
                val += self.get(i, k) * rhs[k]
            }

            output[i] = val;
        }

        output
    }
}

#[test]
fn matrix_test() {
    let mut m = Matrix::new(2);
    m.set(0, 0, Complex::new(1f64, 0f64));
    m.set(0, 1, Complex::new(2f64, 0f64));
    m.set(1, 0, Complex::new(3f64, 0f64));
    m.set(1, 1, Complex::new(4f64, 0f64));

    let mut v: Vector = [Complex::zero(); MAX_SIZE];
    v[0] = Complex::new(10f64, 0f64);
    v[1] = Complex::new(20f64, 0f64);

    let mut expected: Vector = [Complex::zero(); MAX_SIZE];
    expected[0] = Complex::new(50f64, 0f64);
    expected[1] = Complex::new(110f64, 0f64);

    let mut added = Matrix::new(2);
    added.set(0, 0, Complex::new(2f64, 0f64));
    added.set(0, 1, Complex::new(4f64, 0f64));
    added.set(1, 0, Complex::new(6f64, 0f64));
    added.set(1, 1, Complex::new(8f64, 0f64));

    let mut squared = Matrix::new(2);
    squared.set(0, 0, Complex::new(7f64, 0f64));
    squared.set(0, 1, Complex::new(10f64, 0f64));
    squared.set(1, 0, Complex::new(15f64, 0f64));
    squared.set(1, 1, Complex::new(22f64, 0f64));

    assert_eq!(added, &m + &m);
    assert_eq!(squared, &m * &m);
    assert_eq!(expected, &m * &v);
}
