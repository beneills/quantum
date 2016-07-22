use classical::ClassicalRegister;
use gate::Gate;
use register::QuantumRegister;

#[derive(Debug, Eq, PartialEq)]
enum State {
    /// The computer has been set up, but the qubits could be anything.
    Initializing,

    /// The comuter is running, with qubits in arbitrary superposition.
    Running,

    /// The system is collapsed/decomposed into a classical state.
    Collapsed
}

/// Represents a quantum computer of one register.
///
/// This is essentially a wrapping around a quantum register
/// with convenience methods to run algorithms, log and read
/// results.
pub struct QuantumComputer {
    state: State,
    width: usize,

    /// Only makes sense if State::Running == state
    register: QuantumRegister,

    /// Only makes sense if State::Collapsed == state
    classical: ClassicalRegister
}

impl QuantumComputer {
    /// Construct a new quantum computer with register of given width.
    pub fn new(width: usize) -> QuantumComputer {
        QuantumComputer {
            state: State::Initializing,
            width: width,
            register: QuantumRegister::new(width, &ClassicalRegister::zeroed(width)),
            classical: ClassicalRegister::zeroed(width)
        }
    }

    /// Initialize the quantum register qubits to a certian classical integer state.
    ///
    /// # Panics
    ///
    /// We panic if the state is anything other than state::Initializing.
    pub fn initialize(&mut self, value: u32) {
        assert_eq!(State::Initializing, self.state);

        let classical = ClassicalRegister::from_int(self.width, value);
        self.register = QuantumRegister::new(self.width, &classical);

        self.state = State::Running;
    }

    /// Apply a quantum gate to the quantum register qubits.
    ///
    /// # Panics
    ///
    /// We panic if the state is anything other than state::Running.
    pub fn apply(&mut self, gate: Gate) {
        assert_eq!(State::Running, self.state);

        self.register.apply(gate);
    }

    /// Collapse the quantum register to a classical state.
    ///
    /// # Panics
    ///
    /// We panic if the state is anything other than state::Running.
    pub fn collapse(&mut self) {
        assert_eq!(State::Running, self.state);

        self.classical = self.register.collapse();

        self.state = State::Collapsed;
    }

    /// Reset the quantum register, ready to be initialized again.
    ///
    /// # Panics
    ///
    /// We panic if the state is anything other than state::Collapsed.
    pub fn reset(&mut self) {
        self.state = State::Initializing;
    }

    /// Read the collapsed register qubits as an integer.
    ///
    /// # Panics
    ///
    /// We panic if the state is anything other than state::Collapsed.
    pub fn value(&self) -> u32 {
        assert_eq!(State::Collapsed, self.state);

        self.classical.to_int()
    }
}

#[test]
fn state_test() {
    let mut c = QuantumComputer::new(3);
    assert_eq!(State::Initializing, c.state);

    c.initialize(5);
    assert_eq!(State::Running, c.state);

    c.collapse();
    assert_eq!(State::Collapsed, c.state);

    c.value();

    c.reset();
    assert_eq!(State::Initializing, c.state);
}

#[test]
fn compute_test() {
    use gates;

    let mut c = QuantumComputer::new(3);

    c.initialize(5);

    c.apply(gates::identity(3));

    c.collapse();

    assert_eq!(5, c.value());
}
