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

use crate::evaluator::AstNode;
use crate::evaluator::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Result<AstNode, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_mul_div()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::Plus | Token::Minus => {
                    self.consume_token();
                    let op = token.clone();
                    let right = self.parse_mul_div()?;
                    node = AstNode::BinaryOp {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_mul_div(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_exponent()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::Multiply | Token::Divide => {
                    self.consume_token();
                    let op = token.clone();
                    let right = self.parse_exponent()?;
                    node = AstNode::BinaryOp {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(node)
    }
    fn parse_exponent(&mut self) -> Result<AstNode, String> {
        let mut node = self.parse_primary()?;

        while let Some(token) = self.peek_token() {
            match token {
                Token::Exponent => {
                    self.consume_token();
                    let op = token.clone();
                    let right = self.parse_primary()?;
                    node = AstNode::BinaryOp {
                        left: Box::new(node),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<AstNode, String> {
        if let Some(token) = self.next_token() {
            match token {
                Token::Number(value) => Ok(AstNode::Number(value)),
                Token::OpenParen => {
                    let node = self.parse_expression();
                    match self.next_token() {
                        Some(Token::CloseParen) => (),
                        _ => return Err(String::from("Unmatched opening parenthesis")),
                    }
                    node
                }
                Token::Minus => {
                    let expr = self.parse_primary()?;
                    Ok(AstNode::UnaryOp {
                        op: Token::Minus,
                        expr: Box::new(expr),
                    })
                }
                Token::UnaryFunction(func) => {
                    let option = self.next_token();
                    match option {
                        Some(Token::OpenParen) => (),
                        _ => {
                            return Err(String::from(
                                "Function must be followed by opening parenthesis",
                            ))
                        }
                    }
                    let expr = self.parse_expression()?;
                    match self.next_token() {
                        Some(Token::CloseParen) => (),
                        _ => return Err(String::from("Unmatched opening parenthesis")),
                    }
                    Ok(AstNode::Function {
                        func,
                        expr: Box::new(expr),
                    })
                }
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            return Err(String::from("Unexpected end of token stream"));
        }
    }

    fn peek_token(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            Some(self.tokens[self.current].clone())
        } else {
            None
        }
    }

    fn consume_token(&mut self) {
        self.current += 1;
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.current < self.tokens.len() {
            let pos = self.current;
            self.current += 1;
            Some(self.tokens[pos].clone())
        } else {
            None
        }
    }
}
