use crate::Result;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Expression {
    Symbol(String),
    Number(f64),
    List(Vec<Expression>),
    Function(Function),
}

// TODO: impl PartialEq
pub struct Function(fn(&[Expression]) -> Result<Expression>);

impl Function {
    pub fn new(f: fn(&[Expression]) -> Result<Expression>) -> Self {
        Self(f)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function").finish()
    }
}

#[macro_export]
macro_rules! num {
    ($n:expr) => {
        Expression::Number($n as f64)
    };
}

#[macro_export]
macro_rules! sym {
    ($s:expr) => {
        Expression::Symbol($s.to_owned())
    };
}

#[macro_export]
macro_rules! list {
    ($($s:expr),*) => {
        Expression::List(vec![$($s),*])
    };
}

#[macro_export]
macro_rules! func {
    ($f:expr) => {
        Expression::Function(crate::expression::Function::new($f))
    };
}
