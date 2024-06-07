/*
 * Copyright (c) 2024.
 *
 * Copyright 2024 Trevor Campbell
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
 * associated documentation files (the “Software”), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
 * NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
 * OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 */

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
