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

use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::time::Instant;

use log::info;

use crate::evaluator::constants::Constant;
use crate::evaluator::functions::Function;
use crate::evaluator::parser::Parser;
use crate::evaluator::tokeniser::tokenize;
use crate::history;

mod functions;
pub(crate) mod parser;
pub(crate) mod tokeniser;
pub(crate) mod constants;

#[derive(Clone, Debug)]
pub(crate) enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    OpenParen,
    CloseParen,
    UnaryFunction(Rc<Function>),
}

impl Token {
    pub(crate) fn perform_binary(&self, left: f64, right: f64, _mode: &AngleMode) -> f64 {
        match self {
            Token::Plus => left + right,
            Token::Minus => left - right,
            Token::Multiply => left * right,
            Token::Divide => left / right,
            Token::Exponent => left.powf(right),
            _ => panic!("Unexpected operator in binary operation: {:?}", self),
        }
    }

    pub(crate) fn perform_unary(&self, val: f64, mode: &AngleMode) -> f64 {
        match self {
            Token::Minus => -val,
            Token::UnaryFunction(f) => f.evaluate(val, mode),
            _ => panic!("Unexpected operator in unary operation: {:?}", self),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AstNode {
    Number(f64),
    UnaryOp {
        op: Token,
        expr: Box<AstNode>,
    },
    BinaryOp {
        left: Box<AstNode>,
        op: Token,
        right: Box<AstNode>,
    },
    Function {
        func: Rc<Function>,
        expr: Box<AstNode>,
    },
}

impl AstNode {
    pub(crate) fn evaluate(&self, mode: &AngleMode) -> f64 {
        match self {
            AstNode::Number(value) => *value,
            AstNode::UnaryOp { op, expr } => {
                let value = expr.evaluate(mode);
                op.perform_unary(value, mode)
            }
            AstNode::BinaryOp { left, op, right } => {
                let left_val = left.evaluate(mode);
                let right_val = right.evaluate(mode);
                op.perform_binary(left_val, right_val, mode)
            }
            AstNode::Function { func, expr } => func.evaluate(expr.evaluate(mode), mode),
        }
    }
}

#[derive(Debug)]
pub(crate) enum AngleMode {
    Degrees,
    Radians,
    Gradians,
}

impl Default for AngleMode {
    fn default() -> Self {
        AngleMode::Degrees
    }
}
impl Display for AngleMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AngleMode::Degrees => "Degrees",
            AngleMode::Radians => "Radians",
            AngleMode::Gradians => "Grads"
        })
    }
}

impl AngleMode {
    pub(crate) fn get_from_name(name: &str) -> AngleMode{
        match name {
            "Degrees" => AngleMode::Degrees,
            "Radians" => AngleMode::Radians,
            "Grads" => AngleMode::Gradians,
            _ => Self::default()
        }
    }
}



pub(crate) struct Evaluator<'a> {
    angle_mode: &'a AngleMode,
    function_register: Vec<Function>,
    constant_register: Vec<&'static Constant>,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn create(angle_mode: &'a AngleMode) -> Self {
        Self {
            angle_mode,
            function_register: functions::get_all(),
            constant_register: constants::get_all(),
        }
    }

    pub(crate) fn with_mode(mode: &'a AngleMode) -> Self {
        Self::create(mode)
    }

    pub(crate) fn evaluate(&self, expression: &str) -> Result<f64, String> {

        let t_start = Instant::now();

        if expression.is_empty() {
            return Err("Please supply an expression to evaluate".to_string());
        }
        tokenize(expression, &self)
            .and_then(|tokens| {
                let mut parser = Parser::new(tokens);
                let ast = parser.parse()?;
                Ok(ast.evaluate(&self.angle_mode))
            })
            .inspect(|result| {
                history::manager().add((&expression, &result));
                history::manager().save();
                let duration = Instant::now().duration_since(t_start);
                info!("Evaluated \"{}\" in {} micro seconds", expression.trim(), duration.as_micros());
            })
            .inspect_err(|_| {
                let duration = Instant::now().duration_since(t_start);
                info!("Failed to evaluate \"{}\" in {} micro seconds", expression.trim(), duration.as_micros());
            })

    }
    pub fn function_register(&self) -> &Vec<Function> {
        &self.function_register
    }
    pub fn constant_register(&self) -> &Vec<&'static Constant> {
        &self.constant_register
    }
}
