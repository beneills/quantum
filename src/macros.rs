//! All library macros.

#![macro_use]

/// Convenience macro for complex number construction.
#[macro_export]
macro_rules! c {
    ($re:expr, $im:expr) => {
        Complex::new($re, $im)
    };
}

/// Internal helper macro for matrix constuction.
macro_rules! m_one {
  ( $item:tt ) => ( 1 )
}

/// Internal helper macro for matrix constuction.
macro_rules! m_rec(
  ( [ $($row:tt),* ] [$($i:expr),*]) => ({
     let _rows = 0 $(+ m_one!($row) )*;
     let _cols = (0 $(+ m_one!($i))*) / _rows;

     assert_eq!(_rows, _cols);

     Matrix::new_from_elements(_rows, vec![$($i),*])
  })
);

/// Convenience macro for matrix constuction.
///
/// We could make this more efficient by pattern matching on the five sizes and passing
/// arrays to a constructor.
///
/// Adapted from [rust-la](https://github.com/xasmx/rust-la/blob/master/src/mac$
#[macro_export]
macro_rules! m {
  ( $( $( $i:expr ),* );* ) => ( m_rec!([$([$($i),*]),*] [$($($i),*),*]) )
}

/// Convenience macro for constucting a matrix of reals.
///
/// Adapted from [rust-la](https://github.com/xasmx/rust-la/blob/master/src/mac$
#[macro_export]
macro_rules! m_real {
  ( $( $( $i:expr ),* );* ) => ( m_rec!([$([$(Complex::new($i as f64, 0f64)),*]),*]
                                        [$($(Complex::new($i as f64, 0f64)),*),*]) )
}
