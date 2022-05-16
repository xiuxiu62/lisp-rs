use crate::{func, Error, Expression, Result};
use std::collections::HashMap;

pub struct Environment<'a> {
    data: HashMap<&'a str, Expression>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<'a> Default for Environment<'a> {
    fn default() -> Self {
        let addition = func!(|args: &[Expression]| -> Result<Expression> {
            let sum = parse_floats(args)?.iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(sum))
        });

        let subtraction = func!(|args: &[Expression]| -> Result<Expression> {
            let floats = parse_floats(args)?;
            let first = *floats
                .first()
                .ok_or(Error::Parse("expected at least one number".to_owned()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(first - sum_of_rest))
        });

        EnvironmentBuilder::new()
            .func("+", addition)
            .func("-", subtraction)
            .build()
    }
}

pub struct EnvironmentBuilder<'a>(Environment<'a>);

impl<'a> EnvironmentBuilder<'a> {
    pub fn new() -> Self {
        Self(Environment::new())
    }

    pub fn func(mut self, ident: &'a str, f: Expression) -> Self {
        self.0.data.insert(ident, f);
        self
    }

    pub fn build(self) -> Environment<'a> {
        self.0
    }
}

fn parse_floats(args: &[Expression]) -> Result<Vec<f64>> {
    args.iter().map(|n| parse_float(n)).collect()
}

fn parse_float(expr: &Expression) -> Result<f64> {
    match expr {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error::Parse("expected a number".to_owned())),
    }
}
