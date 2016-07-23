//! All library macros.

#![macro_use]

/// Convenience macro for complex number construction.
#[macro_export]
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}

/// Convenience macro for 2x2 matrix construction.
#[macro_export]
macro_rules! m {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            let mut m = Matrix::new(2);
            m.set(0, 0, $a);
            m.set(0, 1, $b);
            m.set(1, 0, $c);
            m.set(1, 1, $d);

            m
        }
    };
}
