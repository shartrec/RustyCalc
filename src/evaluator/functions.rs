use std::fmt::Debug;
use std::rc::Rc;

use crate::evaluator::{AngleMode, Token};

#[derive(Clone, Debug)]
pub(crate) struct Function {
    name: &'static str,
    function: fn(f64, &AngleMode) -> f64,
}

impl Function {
    pub(crate) fn evaluate(&self, val: f64, mode: &AngleMode) -> f64 {
        (self.function)(val, mode)
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

fn do_trig(v: f64, mode: &AngleMode, f: fn(f64) -> f64) -> f64 {
    let v_radians = match mode {
        AngleMode::Radians => v,
        AngleMode::Degrees => v.to_radians(),
        AngleMode::Gradians => (v * 0.9).to_radians()
    };
    f(v_radians)
}

fn do_atrig(v: f64, mode: &AngleMode, f: fn(f64) -> f64) -> f64 {
    let r = f(v);
    match mode {
        AngleMode::Radians => r,
        AngleMode::Degrees => r.to_degrees(),
        AngleMode::Gradians => r.to_degrees() / 0.9
    }
}

pub(crate) fn get_all() -> Vec<Function> {
    vec![
        Function {
            name: "sin",
            function: |v, mode| do_trig(v, mode, f64::sin)
        },
        Function {
            name: "cos",
            function: |v, mode| do_trig(v, mode, f64::cos),
        },
        Function {
            name: "tan",
            function: |v, mode| do_trig(v, mode, f64::tan),
        },
        Function {
            name: "asin",
            function: |v, mode| do_atrig(v, mode, f64::asin),
        },
        Function {
            name: "acos",
            function: |v, mode| do_atrig(v, mode, f64::acos),
        },
        Function {
            name: "atan",
            function: |v, mode| do_atrig(v, mode, f64::atan),
        },
        Function {
            name: "exp",
            function: |v, _| v.exp(),
        },
        Function {
            name: "ln",
            function: |v, _| v.ln(),
        },
        Function {
            name: "log",
            function: |v, _| v.log10(),
        },
        Function {
            name: "log2",
            function: |v, _| v.log2(),
        },
        Function {
            name: "sqrt",
            function: |v, _| v.sqrt(),
        },
        Function {
            name: "abs",
            function: |v, _| v.abs(),
        },
        Function {
            name: "ceil",
            function: |v, _| v.ceil(),
        },
        Function {
            name: "floor",
            function: |v, _| v.floor(),
        },
        Function {
            name: "factorial",
            function: |v, _| -> f64 {
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
    use crate::evaluator::AngleMode;
    use crate::evaluator::functions::Function;

    #[test]
    fn test_fn() {
        let f = Function {
            name: "sin",
            function: |v, _| v.sin(),
        };
        assert_near!(f.evaluate(std::f64::consts::PI / 2.0, &AngleMode::Radians), 1.0);
    }
}
