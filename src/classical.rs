/// Represents a non-quantum register of _width()_ bits.
///
/// We store this inefficiently for clarity.
pub struct ClassicalRegister {
    bits: Vec<u8>
}

impl ClassicalRegister {

    /// Construct a new non-quantum register, given a vector of ones and zeroes.
    ///
    /// The width is automatically determined from the vector.
    pub fn new(bits: Vec<u8>) -> ClassicalRegister {
        for bit in &bits {
            assert!(0 == *bit || 1 == *bit);
        }

        ClassicalRegister {
            bits: bits
        }
    }

    /// Compute the register's width.
    fn width(&self) -> usize {
        self.bits.len()
    }
}
