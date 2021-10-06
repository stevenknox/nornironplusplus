use crate::runtime::Value;

use super::{
    op::{BinaryOp, UnaryOp},
    Var,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ExprNode {
    expr: Expr,
    line: usize,
}

impl ExprNode {
    pub fn new(expr: Expr, line: usize) -> ExprNode {
        Self { expr, line }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Unary(UnaryOp, Box<ExprNode>),
    Binary(Box<ExprNode>, BinaryOp, Box<ExprNode>),
    Literal(Value),
    Var(Var),
}