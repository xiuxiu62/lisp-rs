use crate::{expression::List, func, Error, Expression, Result};
use std::collections::HashMap;
use tracing::debug;

pub struct Environment {
    data: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, indentifier: &str) -> Option<&Expression> {
        self.data.get(indentifier)
    }

    // Returns the old value when updating an existing expression
    pub fn set(&mut self, identifier: &str, expression: Expression) -> Option<Expression> {
        self.data.insert(identifier.to_owned(), expression)
    }
}

impl Default for Environment {
    fn default() -> Self {
        debug!("adding: addition");
        let addition = func!("+", |args: List| -> Result<Expression> {
            Ok(Expression::Number(
                parse_floats(args)?.iter().fold(0.0, |sum, a| sum + a),
            ))
        });

        debug!("adding: subtraction");
        let subtraction = func!("-", |args: List| -> Result<Expression> {
            let floats = parse_floats(args)?;
            let first = *floats
                .first()
                .ok_or_else(|| Error::Parse("expected at least one number".to_owned()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(first - sum_of_rest))
        });

        let mut environment = Self::new();
        environment.data.insert("+".to_owned(), addition);
        environment.data.insert("-".to_owned(), subtraction);
        environment
    }
}

#[derive(Default)]
pub struct EnvironmentBuilder(Environment);

impl EnvironmentBuilder {
    pub fn set(mut self, identifier: &str, expression: Expression) -> Self {
        self.0.data.insert(identifier.to_owned(), expression);
        self
    }

    pub fn build(self) -> Environment {
        self.0
    }
}

fn parse_floats(args: List) -> Result<Vec<f64>> {
    args.iter().map(parse_float).collect()
}

fn parse_float(expr: &Expression) -> Result<f64> {
    match expr {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error::Parse("expected a number".to_owned())),
    }
}
