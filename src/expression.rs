use crate::Result;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Expression {
    Symbol(String),
    Number(f64),
    List(List),
    Function(Function),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match self {
            Self::Symbol(symbol) => symbol.clone(),
            Self::Number(number) => number.to_string(),
            Self::List(list) => list
                .iter()
                .fold("".to_owned(), |acc, expr| format!("{acc} {expr}"))
                .trim()
                .to_owned(),
            Self::Function(function) => format!("{function}"),
        };

        write!(f, "{message}")
    }
}

pub type List = Vec<Expression>;

// TODO: impl PartialEq
#[derive(Clone)]
pub struct Function {
    identifier: String,
    inner: fn(List) -> Result<Expression>,
}

impl Function {
    pub fn new(identifier: String, inner: fn(List) -> Result<Expression>) -> Self {
        Self { identifier, inner }
    }

    pub fn call(&self, args: List) -> Result<Expression> {
        (self.inner)(args)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function({})", self.identifier)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function({})", self.identifier)
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
    ($ident:expr, $f:expr) => {
        Expression::Function(crate::expression::Function::new($ident.to_owned(), $f))
    };
}
