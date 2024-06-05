use std::fmt::Debug;
use std::rc::Rc;

use crate::evaluator::Token;

#[derive(Clone, Debug)]
pub(crate) struct Function {
    name: &'static str,
    function: fn(f64) -> f64,
}

impl Function {
    pub(crate) fn evaluate(&self, val: f64) -> f64 {
        (self.function)(val)
    }

    pub(crate) fn name(&self) -> &'static str {
        &self.name
    }

    pub(crate) fn is_token(
        &self,
        chars: &Vec<char>,
        exp_len: &usize,
        i: &usize,
    ) -> Option<(Token, usize)> {
        let name = self.name();
        if i + name.len() - 1 < *exp_len
            && chars[*i..*i + name.len()] == name.chars().collect::<Vec<char>>()
        {
            //Need to check the next character is not alphanumeric, otherwise it is the name of a different function
            if (i + name.len() < *exp_len) && chars[*i + name.len()].is_alphanumeric() {
                None
            } else {
                Some((Token::UnaryFunction(Rc::new(self.clone())), name.len()))
            }
        } else {
            None
        }
    }
}

pub(crate) fn get_all() -> Vec<Function> {
    vec![
        Function {
            name: "sin",
            function: f64::sin,
        },
        Function {
            name: "cos",
            function: f64::cos,
        },
        Function {
            name: "tan",
            function: f64::tan,
        },
        Function {
            name: "asin",
            function: f64::asin,
        },
        Function {
            name: "acos",
            function: f64::acos,
        },
        Function {
            name: "atan",
            function: f64::atan,
        },
        Function {
            name: "exp",
            function: f64::exp,
        },
        Function {
            name: "ln",
            function: f64::ln,
        },
        Function {
            name: "log",
            function: f64::log10,
        },
        Function {
            name: "log2",
            function: f64::log2,
        },
        Function {
            name: "sqrt",
            function: f64::sqrt,
        },
        Function {
            name: "abs",
            function: f64::abs,
        },
        Function {
            name: "ceil",
            function: f64::ceil,
        },
        Function {
            name: "floor",
            function: f64::floor,
        },
        Function {
            name: "factorial",
            function: |v| -> f64 {
                if v > 170.0 {
                    f64::INFINITY
                } else if v.fract() != 0.0 {
                    f64::NAN
                } else {
                    let mut factorial: f64 = 1.0;
                    let v_floor = v.floor() as i32;
                    for i in 2..v_floor {
                        factorial *= i as f64;
                    }
                    factorial *= v;
                    factorial
                }
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::evaluator::functions::Function;

    #[test]
    fn test_fn() {
        let f = Function {
            name: "sin",
            function: f64::sin,
        };
        assert_near!(f.evaluate(std::f64::consts::PI / 2.0), 1.0);
    }
}
