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

use std::fmt::Debug;

use crate::evaluator::Token;

#[derive(Clone, Debug)]
pub(crate) struct Constant {
    name: &'static str,
    value: f64,
}

impl Constant {

    pub(crate) fn name(&self) -> &'static str {
        &self.name
    }

    pub(crate) fn is_token(
        &self,
        chars: &Vec<char>,
        exp_len: &usize,
        i: &usize,
    ) -> Option<(Token, usize)> {
        // All these lengths need to be done in characters not Strings because of unicode
        let name = self.name();
        let cons_as_chars = name.chars().collect::<Vec<char>>();
        let name_len = cons_as_chars.len();
        let expr_chars_to_compare = &chars[*i..*i + name_len];
        if i + name_len - 1 < *exp_len  && expr_chars_to_compare == cons_as_chars
        {
            //Need to check the next character is not alphanumeric, otherwise it is the name of a different function
            if (i + name_len < *exp_len) && chars[*i + name_len].is_alphanumeric() {
                None
            } else {
                Some((Token::Number(self.value), name_len))
            }
        } else {
            None
        }
    }

}


pub(crate) fn get_all() -> Vec<Constant> {
    vec![
        Constant {
            name: "π",
            value: std::f64::consts::PI,
        },
        Constant {
            name: "e",
            value: std::f64::consts::E,
        },
        Constant {
            name: "Φ",
            value: 1.61803,
        },
        Constant {
            name: "C",
            value: 299792458.0
        },
        Constant {
            name: "ℎ",
            value: 6.626e-34
        },
        Constant {
            name: "G",
            value: 6.674e-11
        },

    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::evaluator::constants::Constant;

    #[test]
    fn test_const() {
        let f = Constant {
            name: "Pi",
            value: 50.2,
        };
        assert_near!(f.value, 50.2);
    }
}
