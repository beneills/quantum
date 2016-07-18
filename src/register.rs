use std::cell::Cell;

use classical::ClassicalRegister;

/// Represents a register of an arbitrary number of qubits.
///
/// The register consists _width_ qubits, all of which are quantum
/// entangled.  This means we store the state of the register, which
/// is normally a quantum superposition of the 2^_width_ possible
/// classical states, unless the register is _collapsed_ (see below_).
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
/// _Collapsing_ the register yields one of the 2^_width_ classical states.
///
/// We store the superposition internally as a vector of 2^_width_ complex
/// coefficients, known as a _ket_, with the theoretical condition that the
/// sum of their square moduli equals 1.
///
/// This representation should approximately confrm to this condition, subject
/// to floating point imprecision.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_computing#Mechanics)
/// for more information.
struct QuantumRegister {
    width: usize,
    collapsed: Cell<bool>,
}

impl QuantumRegister {

    /// Construct a new quantum register of given _width_.
    ///
    /// TODO accept intial state
    fn new(width: usize) -> QuantumRegister {
        QuantumRegister {
            width: width,
            collapsed: Cell::new(false)
        }
    }

    /// Collapse the register to yield one a classical state.
    ///
    /// A register may only be collapsed once, and is invalid thereafter.
    fn collapse(&mut self) -> ClassicalRegister {
        assert_eq!(false, self.collapsed.get());

        self.collapsed = Cell::new(true);

        ClassicalRegister::new(vec![1, 0])
    }
}

#[test]
fn initialization_test() {
    let r: QuantumRegister = QuantumRegister::new(5);

    assert_eq!(false, r.collapsed.get());
    assert_eq!(5, r.width);
}

#[test]
fn collapse_test() {
    let mut r: QuantumRegister = QuantumRegister::new(5);
    r.collapse();

    assert!(r.collapsed.get());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn double_collapse_test() {
    let mut r: QuantumRegister = QuantumRegister::new(5);
    r.collapse();
    r.collapse();
}
