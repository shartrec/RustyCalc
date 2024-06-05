#[cfg(test)]
mod evaluator_tests;

#[macro_export]
macro_rules! assert_near {
    ($left:expr, $right:expr) => {
        $crate::assert_near!($left, $right, 0.000001f64);
    };
    ($left:expr, $right:expr, $tol:expr) => {
        match (&$left, &$right, &$tol) {
            (left_val, right_val, tol_val) => {
                if (*left_val - *right_val).abs() > *tol_val {
                    panic!(
                        "assertion failed: `(left ≈ right)` \
                (left: `{:?}`, right: `{:?}`, tol: `{:?}`)",
                        &*left_val, &*right_val, &*tol_val
                    )
                }
            }
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => match left_val {
                Err(e) => assert_eq!(e, *right_val, "unexpected error message"),
                _ => panic!("Expected error {}, but it didn't happen", *right_val),
            },
        }
    };
}
