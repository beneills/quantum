//! Classical and quantum register library code (public for pedagogical reasons).

use rand;
use std::cell::Cell;

use gate::Gate;
use ket::Ket;

/// Represents a register of an arbitrary number of qubits.
///
/// The register consists `width` qubits, all of which are quantum
/// entangled.  This means we store the state of the register, which
/// is normally a quantum superposition of the _2^width_ possible
/// classical states, unless the register is _collapsed_ (see below).
///
/// The register must be initialized with a starting (classical) state,
/// and therefore holds valid superposition state from constuction.  This
/// state persists through the _gates_ which may be applied to it, up until
/// _collapse_ (or resource destruction).
///
/// It is  possible to _collapse_ the register __once__ during its lifetime,
/// after which it no longer stores superposition state and therefore cannot
/// provide further useful information.
///
/// _Collapsing_ the register yields one of the _2^width_ classical states.
///
/// We store the superposition internally as a vector of _2^width_ complex
/// coefficients, known as a _ket_, with the theoretical condition that the
/// sum of their square moduli equals 1.
///
/// This representation should approximately confrm to this condition, subject
/// to floating point imprecision.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_computing#Mechanics)
/// for more information.
#[derive(Debug)]
pub struct QuantumRegister {
    width: usize,
    collapsed: Cell<bool>,
    ket: Ket,
}

impl QuantumRegister {
    /// Construct a new quantum register of given `width` and initial state.
    ///
    /// # Panics
    ///
    /// We panic if the initial state register has a different size to `width`.
    pub fn new(width: usize, initial: &ClassicalRegister) -> QuantumRegister {
        assert_eq!(width, initial.width());

        QuantumRegister {
            width: width,
            collapsed: Cell::new(false),
            ket: Ket::from_classical(initial),
        }
    }

    /// Apply a quantum gate to this register, mutating its state.
    pub fn apply(&mut self, gate: Gate) {
        assert_eq!(false, self.collapsed.get());
        assert_eq!(self.width, gate.width());

        self.ket.apply(gate);
    }

    /// Collapse the register to yield one a classical state.
    ///
    /// A register may only be collapsed once, and is invalid thereafter.
    pub fn collapse(&mut self) -> ClassicalRegister {
        assert_eq!(false, self.collapsed.get());

        self.collapsed = Cell::new(true);

        // Algorithm:
        // 1) we choose a random float between `0` and `1`
        // 2) we partition `[0, 1 + epsilon)` using the ket coefficient square modulii
        // 3) we randomly choose a coefficient
        // 4) we return the matching state

        let sample = rand::random::<f64>() % 1.0;
        let mut cumulative = 0f64;

        for (state, coefficient) in self.ket.elements.iter().enumerate() {
            cumulative += coefficient.norm_sqr();

            if sample < cumulative {
                return ClassicalRegister::from_state(self.width, state as u32);
            }
        }

        // catch floating point imprecision
        // TODO log this somewhere
        ClassicalRegister::from_state(self.width, 0)
    }

    /// Compute the probabilities of each state without collapsing.
    ///
    /// This function is intended for test purposes.
    ///
    /// We return a vector of probabilities mirroring a ket, but without trailing zeroes.
    pub fn probabilities(&self) -> Vec<f64> {
        assert_eq!(false, self.collapsed.get());

        let mut probabilities = vec![];

        for (_, coefficient) in self.ket.elements.iter().take(Ket::size(self.width)).enumerate() {
            probabilities.push(coefficient.norm_sqr());
        }

        probabilities
    }
}

#[test]
fn initialization_test() {
    let nibble = ClassicalRegister::zeroed(4);
    let r: QuantumRegister = QuantumRegister::new(4, &nibble);

    assert_eq!(false, r.collapsed.get());
    assert_eq!(4, r.width);
    assert!(&r.ket.is_classical());
}

#[test]
fn collapse_test() {
    let nibble = ClassicalRegister::zeroed(4);
    let mut r: QuantumRegister = QuantumRegister::new(4, &nibble);
    let end: ClassicalRegister = r.collapse();

    // We haven't performed any operations on the quantum register, therefore
    // it should remain unchanged upon collapse.
    assert_eq!(nibble, end);
    assert!(r.collapsed.get());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn double_collapse_test() {
    let nibble = ClassicalRegister::zeroed(4);
    let mut r: QuantumRegister = QuantumRegister::new(4, &nibble);
    r.collapse();
    r.collapse();
}

#[test]
fn probabilities_test() {
    use float_cmp::ApproxEqUlps;
    use gates;

    // We zero intialize our single qubit, then apply the Hadamard gate to
    // achieve a symmetrical system where ach of the two possible states are
    // equally likely.
    let nibble = ClassicalRegister::zeroed(1);
    let mut r: QuantumRegister = QuantumRegister::new(1, &nibble);
    r.apply(gates::hadamard(1));

    // Now we test that the proabbilities are balanced, i.e. equal to vec![0.5, 0.5]
    assert_eq!(2, r.probabilities().len());
    assert!(0.5f64.approx_eq_ulps(&r.probabilities()[0], 10));
    assert!(0.5f64.approx_eq_ulps(&r.probabilities()[1], 10));
}

/// Represents a non-quantum register of `width()` bits.
///
/// We store this inefficiently for clarity.
#[derive(Debug, Eq, PartialEq)]
pub struct ClassicalRegister {
    bits: Vec<u8>,
}

impl ClassicalRegister {
    /// Construct a new non-quantum register, given a vector of ones and zeroes.
    ///
    /// The width is automatically determined from the vector.
    pub fn new(bits: Vec<u8>) -> ClassicalRegister {
        for bit in &bits {
            assert!(0 == *bit || 1 == *bit);
        }

        ClassicalRegister { bits: bits }
    }

    /// Construct a new non-quantum register, given a `state`.
    ///
    /// See the `state()` method documentation for details of the encoding.
    ///
    /// # Panics
    ///
    /// We assert that the state is valid for the given width.
    ///
    pub fn from_state(width: usize, state: u32) -> ClassicalRegister {
        assert!(state < 2u32.pow(width as u32));

        let mut bits = Vec::new();
        let mut remaining_state = state;

        for i in 0..width {
            let pos: u32 = (width - i - 1) as u32;
            let value = 2u32.pow(pos);

            // Insert a one or a zero at the front of the vector.
            if value <= remaining_state {
                remaining_state -= value;
                bits.insert(0, 1);
            } else {
                bits.insert(0, 0);
            }
        }

        ClassicalRegister::new(bits)
    }

    /// Construct a new non-quantum register, given an unsigned integer.
    ///
    /// See the `state()` method documentation for details of the encoding.
    ///
    /// # Panics
    ///
    /// We assert that the integer is valid for the width.
    ///
    pub fn from_int(width: usize, int: u32) -> ClassicalRegister {
        ClassicalRegister::from_state(width, int)
    }

    /// Construct zero-initialized non-quantum register of given `width`.
    pub fn zeroed(width: usize) -> ClassicalRegister {
        ClassicalRegister::new(vec![0; width])
    }

    /// Compute the register's `width`.
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Compute the current `state` of the register.
    ///
    /// The `state` is an integer which uniquely specifies all register bits (for a
    /// given width).  It does this in the obvious way, by enumerating all _2^n_ bit
    /// strings in the reversed lexicographic order, and assigning each string an index.
    ///
    /// This is equivalent to interpreting the register as an integer with the leftmost
    /// bit of least significance.
    ///
    /// # Panics
    ///
    /// This only works for registers of `width <= 32`.
    pub fn state(&self) -> u32 {
        let mut state = 0u32;

        for (pos, bit) in self.bits.iter().enumerate() {
            if 0u8 != *bit {
                state += 2u32.pow(pos as u32);
            }
        }

        state
    }

    /// Return the integer represented by this register.
    ///
    /// See the `state()` method documentation for details of the encoding.
    pub fn to_int(&self) -> u32 {
        self.state()
    }
}

#[test]
fn state_test() {
    let nibble = ClassicalRegister::new(vec![0, 1, 0, 1]);

    assert_eq!(10, nibble.state());
    assert_eq!(nibble, ClassicalRegister::from_state(4, nibble.state()));
}
