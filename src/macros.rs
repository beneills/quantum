#![macro_use]

/// Square a numeric value efficiently by multiplying it with itself.
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

/// Compute a complex number's absolute value, i.e. _|x + iy|^2_.
macro_rules! abs_square {
    ($re:expr, $im:expr) => {
        square!($re) + square!($im)
    };
}
