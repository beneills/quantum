extern crate float_cmp;
extern crate rand;

mod macros;

pub mod classical;
#[macro_use] mod complex;
mod gate;
mod ket;
mod matrix;
mod qubit;
mod register;
