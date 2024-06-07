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

use crate::evaluator::{Evaluator, Token};

pub(crate) fn tokenize(expression: &str, evaluator: &Evaluator) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                    num_str.push(chars[i]);
                    i += 1;
                }
                let number = num_str.parse::<f64>().unwrap();
                tokens.push(Token::Number(number));
                continue; // Skip the increment below because it's already done
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '^' => tokens.push(Token::Exponent),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            ' ' | '\n' => {} // Ignore spaces
            _ => {
                // We now look for a function.
                match parse_functions(&chars, chars.len(), i, &evaluator) {
                    Ok((token, consumed)) => {
                        tokens.push(token);
                        i += consumed;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        i += 1;
    }

    Ok(tokens)
}

fn parse_functions(
    chars: &Vec<char>,
    exp_len: usize,
    i: usize,
    evaluator: &Evaluator,
) -> Result<(Token, usize), String> {
    for function in evaluator.function_register() {
        if let Some((token, consumed)) = function.is_token(chars, &exp_len, &i) {
            return Ok((token, consumed));
        }
    }
    Err(format!("Invalid token '{}' at position: {}", &chars[i], i))
}
