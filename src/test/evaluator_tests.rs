#[cfg(test)]
mod tests {
    use crate::{assert_err, assert_near};
    use crate::evaluator::{AngleMode, Evaluator};

    #[test]
    fn test_add_i() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("2 + 2");
        assert_near!(result.unwrap(), 2.0 + 2.0);
    }

    #[test]
    fn test_add_f() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("2.0 + 2.0");
        assert_near!(result.unwrap(), 2.0 + 2.0);
    }

    #[test]
    fn test_subtract() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("5 - 3");
        assert_near!(result.unwrap(), 5.0 - 3.0);
    }

    #[test]
    fn test_multiply() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("4 * 3");
        assert_near!(result.unwrap(), 4.0 * 3.0);
    }

    #[test]
    fn test_divide() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("10 / 2");
        assert_near!(result.unwrap(), 10.0 / 2.0);
    }

    #[test]
    fn test_exponent() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("2 ^ 3");
        assert_near!(result.unwrap(), 2.0f64.powf(3.0));
    }

    #[test]
    fn test_complex_expression() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("3 + 5 * (2 - 8) ^ 2");
        assert_near!(result.unwrap(), 3.0 + 5.0 * (2.0 - 8.0f64).powf(2.0));
    }

    #[test]
    fn test_sine() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("sin(0)");
        assert_near!(result.unwrap(), 0.0f64.sin());
        let result = Evaluator::with_mode(&AngleMode::Degrees).evaluate("sin(30)");
        assert_near!(result.unwrap(), 0.5);
        let result = Evaluator::with_mode(&AngleMode::Gradians).evaluate("sin(33.3333333)");
        assert_near!(result.unwrap(), 0.5);
    }

    #[test]
    fn test_cosine() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("cos(0)");
        assert_near!(result.unwrap(), 0.0f64.cos());
    }

    #[test]
    fn test_cosine_2() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("cos(45) * 7");
        assert_near!(result.unwrap(), 45.0f64.cos() * 7f64);
    }

    #[test]
    fn test_log() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("10 ^ log(7)");
        assert_near!(result.unwrap(), 7.0);
    }

    #[test]
    fn test_log2() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("2 ^ log2(7)");
        assert_near!(result.unwrap(), 7.0);
    }

    #[test]
    fn test_ln() {
        let result = Evaluator::with_mode(&AngleMode::Radians).evaluate("exp(ln(7))");
        assert_near!(result.unwrap(), 7.0);
    }

    #[test]
    fn test_abs() {
        let evaluator = Evaluator::with_mode(&AngleMode::Radians);
        assert_near!(evaluator.evaluate("abs(7)").unwrap(), 7.0);
        assert_near!(evaluator.evaluate("abs(-7.456)").unwrap(), 7.456);
    }

    #[test]
    fn test_sqrt() {
        let evaluator = Evaluator::with_mode(&AngleMode::Radians);
        assert_near!(evaluator.evaluate("sqrt(144)").unwrap(), 12.0f64);
        assert_near!(evaluator.evaluate("sqrt(94)").unwrap(), 94.0f64.sqrt());
        // Was pythagoras right?
        assert_near!(evaluator.evaluate("sqrt(3*3 + 4^2)").unwrap(), 5.0);
        assert!(evaluator.evaluate("sqrt(-7.456)").unwrap().is_nan());
    }

    #[test]
    fn test_factorial() {
        let evaluator = Evaluator::with_mode(&AngleMode::Radians);
        assert_near!(evaluator.evaluate("factorial(5)").unwrap(), 120.0f64);
        assert_near!(evaluator.evaluate("factorial(5.5)").unwrap(), f64::NAN);
        assert_near!(evaluator.evaluate("factorial(1)").unwrap(), 1.0f64);
        assert_near!(evaluator.evaluate("factorial(50)").unwrap(), 3.041409e64, 1.0e59);
        assert_near!(evaluator.evaluate("factorial(169)").unwrap(), 4.269068e304, 1.0e300);
        assert_near!(evaluator.evaluate("factorial(300)").unwrap(), f64::INFINITY);
    }

    #[test]
    fn test_invalid() {
        let evaluator = Evaluator::with_mode(&AngleMode::Radians);
        assert_err!(
            evaluator.evaluate(""),
            "Please supply an expression to evaluate"
        );
        assert_err!(evaluator.evaluate("3*4*"), "Unexpected end of token stream");
        assert_err!(evaluator.evaluate("sqrt()"), "Unexpected token: CloseParen");
        assert_err!(evaluator.evaluate("sqrt(/"), "Unexpected token: Divide");
        assert_err!(
            evaluator.evaluate("sqrt"),
            "Function must be followed by opening parenthesis"
        );
        assert_err!(evaluator.evaluate("3*/2"), "Unexpected token: Divide");
        assert_err!(
            evaluator.evaluate("poop(3)"),
            "Invalid token 'p' at position: 0"
        );
    }
}
