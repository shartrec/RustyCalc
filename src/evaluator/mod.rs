use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use crate::evaluator::functions::Function;
use crate::evaluator::parser::Parser;
use crate::evaluator::tokeniser::tokenize;

mod functions;
pub(crate) mod parser;
pub(crate) mod tokeniser;

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
        AngleMode::Radians
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

pub(crate) struct Evaluator<'a> {
    angle_mode: &'a AngleMode,
    function_register: Vec<Function>,
}

impl<'a> Evaluator<'a> {
    pub(crate) fn create(angle_mode: &'a AngleMode) -> Self {
        Self {
            angle_mode,
            function_register: functions::get_all(),
        }
    }

    pub(crate) fn with_mode(mode: &'a AngleMode) -> Self {
        Self::create(mode)
    }

    pub(crate) fn evaluate(&self, expression: &str) -> Result<f64, String> {
        if expression.is_empty() {
            return Err("Please supply an expression to evaluate".to_string());
        }
        match tokenize(expression, &self) {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let ast = parser.parse()?;
                Ok(ast.evaluate(&self.angle_mode))
            }
            Err(s) => Err(s),
        }
    }
    pub fn function_register(&self) -> &Vec<Function> {
        &self.function_register
    }
}
