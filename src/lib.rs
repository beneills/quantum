//! Advanced Rust quantum computer simulator.
//!
//! See the code [here](https://github.com/beneills/quantum).

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate float_cmp;
extern crate rand;

pub mod complex;
pub mod computer;
pub mod gate;
pub mod gates;
pub mod ket;
pub mod matrix;
pub mod other;
pub mod registers;
