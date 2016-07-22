//! Ket library code (public for pedagogical reasons).

use float_cmp::ApproxEqUlps;

use complex::Complex;
use gate::Gate;
use matrix::MAX_SIZE;
use registers::ClassicalRegister;

/// A ket describes the state of a quantum register.
///
/// We choose to always use kets as coefficients for linear combinations of
/// the _computational basis_.  A register of width `n` corresponds to a ket
/// of size _2^n_.  Theoretically, the sum of the square coefficient moduli
/// must equal `1`.
///
/// We store the elements (left-aligned) in an array of size `MAX_SIZE`, with
/// the unused slots set to zero.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Bra%E2%80%93ket_notation) for
/// more information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ket {
    size: usize,

    /// The ket's elements, w.r.t. the computational basis.
    pub elements: [Complex; MAX_SIZE],
}

impl Ket {
    /// Construct a new, zero-initialized ket of given size.
    pub fn new(size: usize) -> Ket {
        Ket {
            size: size,
            elements: [Complex::zero(); MAX_SIZE],
        }
    }

    /// Generate a ket from a classical register.
    ///
    /// This ket encodes a single basis vector, and is used for initializing a
    /// quantum register to an initial (classical) state.
    pub fn from_classical(register: &ClassicalRegister) -> Ket {
        let mut ket = Ket::new(Ket::size(register.width()));

        ket.elements[register.state() as usize] = Complex::one();

        ket
    }

    /// Is this structure a valid ket?
    #[allow(unused)]
    pub fn is_valid(&self) -> bool {
        let mut sample_space_sum = 0f64;

        for coefficient in self.elements.iter() {
            sample_space_sum += coefficient.norm_sqr()
        }

        sample_space_sum.approx_eq_ulps(&1.0f64, 10)
    }

    /// Determine whether this ket represents a classically possible state.
    ///
    /// This is true iff. the ket encodes a single basis vector, meaning that
    /// precisely one slot will have value `1`, and all others `0`.
    #[allow(unused)]
    pub fn is_classical(&self) -> bool {
        assert!(self.is_valid());

        let mut zeroes = 0;
        let mut ones = 0;
        let mut others = 0;

        for coefficient in self.elements.iter() {
            if Complex::zero() == *coefficient {
                zeroes += 1;
            } else if Complex::one() == *coefficient {
                ones += 1;
            } else {
                others += 1;
            }
        }

        return 1 == ones && 0 == others;
    }

    /// Compute a ket's size from the register width.
    pub fn size(register_width: usize) -> usize {
        2usize.pow(register_width as u32)
    }

    /// Apply a quantum gate to this ket, mutating its state.
    pub fn apply(&mut self, gate: Gate) {
        self.elements = gate.matrix() * &self.elements;
    }
}

#[test]
fn valid_test() {
    let mut valid = Ket::new(3);
    valid.elements[0] = Complex::zero();
    valid.elements[1] = Complex::zero();
    valid.elements[2] = Complex::one();

    let mut invalid = Ket::new(3);
    invalid.elements[0] = Complex::new(0.5, 0.0);
    invalid.elements[1] = Complex::new(0.0, 0.5);

    assert!(valid.is_valid());
    assert_eq!(false, invalid.is_valid());
}

#[test]
fn classical_test() {
    let sqrt2inv = 2.0f64.sqrt().recip();

    let mut classical = Ket::new(3);
    classical.elements[0] = Complex::zero();
    classical.elements[1] = Complex::zero();
    classical.elements[2] = Complex::one();

    let mut nonclassical1 = Ket::new(2);
    nonclassical1.elements[0] = Complex::new(sqrt2inv, 0.0);
    nonclassical1.elements[1] = Complex::new(0.0, sqrt2inv);

    let mut nonclassical2 = Ket::new(2);
    nonclassical2.elements[0] = Complex::new(0.5, 0.5);
    nonclassical2.elements[1] = Complex::new(0.5, 0.5);

    assert!(classical.is_classical());
    assert_eq!(false, nonclassical1.is_classical());
    assert_eq!(false, nonclassical2.is_classical());
}

#[test]
fn from_classical_test() {
    let r: ClassicalRegister = ClassicalRegister::new(vec![0, 1]);

    let mut expected = Ket::new(4);
    expected.elements[0] = Complex::zero();
    expected.elements[1] = Complex::zero();
    expected.elements[2] = Complex::one();
    expected.elements[3] = Complex::zero();

    assert!(Ket::from_classical(&r).is_classical());
    assert_eq!(expected, Ket::from_classical(&r));
}
