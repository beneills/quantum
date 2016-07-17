extern crate float_cmp;

use self::float_cmp::ApproxEqUlps;

/// Represents a single (pure) qubit state of the form _a|0> + b|1>_.
///
/// The qubit is the linear superposition of the computational basis of _|0>_ and _|1>_
///
/// We encode the complex coeffients as tuples of their real and imaginary parts,
/// each represented as a 64-bit floating points.  This gives high accuracy, while
/// allowing word-size arithmetic on 64-bit systems.
///
/// The theoretical state should always satisfy the equations:
///  * _a = a_re + i * a_im_
///  * _b = b_re + i * b_im_
///  * _1 = |a|^2+ |b|^2_
///
/// This representaion of that state should approximately satisfy them, subject to floating
/// point imprecision.
struct Qubit {
    a_re: f64,
    a_im: f64,
    b_re: f64,
    b_im: f64
}


impl Qubit {

    /// Safely construct a qubit, given the real and imaginary parts of both coefficients.
    ///
    /// This function validates that the given state is possible.
    fn new(a_re: f64, a_im: f64, b_re: f64, b_im: f64) -> Qubit {
        let candidate = Qubit {
            a_re: a_re,
            a_im: a_im,
            b_re: b_re,
            b_im: b_im,
        };

        assert!(candidate.validate());

        candidate
    }

    /// Validate that this qubit's state is possible.
    ///
    /// In our imperfect floating point model, this means computing _|a|^2+ |b|^2_ and
    /// comparing it to _1_ with some leeway.
    ///
    /// That leeway is arbitrarily chosen as 10 units of least precision.
    #[cfg(not(feature = "optimize"))]
    fn validate(&self) -> bool {
        let sample_space_sum: f64 = abs_square!(self.a_re, self.a_im) + abs_square!(self.b_re, self.b_im);

        sample_space_sum.approx_eq_ulps(&1.0f64, 10)
    }

    /// Skip state validation for speed.
    #[cfg(feature = "optimize")]
    #[inline(always)]
    fn validate(&self) -> bool {
        true
    }

}

#[test]
fn initialization_test() {
    let sqrt2inv = 2.0f64.sqrt().recip();

    let q1: Qubit = Qubit::new(0.5,      0.5,      0.5,      0.5);
    let q2: Qubit = Qubit::new(sqrt2inv, sqrt2inv, 0.0,      0.0);
    let q3: Qubit = Qubit::new(0.0,      0.0,      sqrt2inv, sqrt2inv);


    assert!(q1.validate());
    assert!(q2.validate());
    assert!(q3.validate());
}

#[test]
#[should_panic(expected = "assertion failed")]
#[cfg(not(feature = "optimize"))]
fn bad_initialization_test() {
    Qubit::new(0.0,      0.0,      0.0,      0.0);
}
