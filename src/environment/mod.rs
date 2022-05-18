use crate::{expression::List, func, Error, Expression, Result};
use std::collections::HashMap;
use tracing::debug;

mod builder;
pub mod util;

pub use builder::EnvironmentBuilder;

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
        let mut environment = Self::new();

        macro_rules! local_env_add {
            ($ident:expr, $f:expr) => {
                crate::env_add!(environment, ($ident, $f))
            };
        }

        local_env_add!("+", |args: List| -> Result<Expression> {
            Ok(Expression::Number(
                util::parse_floats(args)?.iter().fold(0.0, |sum, a| sum + a),
            ))
        });

        local_env_add!("-", |args: List| -> Result<Expression> {
            let floats = util::parse_floats(args)?;
            let first = *floats
                .first()
                .ok_or_else(|| Error::Parse("expected at least one number".to_owned()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(Expression::Number(first - sum_of_rest))
        });

        local_env_add!("*", |args: List| -> Result<Expression> {
            let floats = util::parse_floats(args)?;
            let first = *floats
                .first()
                .ok_or_else(|| Error::Parse("expected at least one number".to_owned()))?;

            Ok(Expression::Number(
                floats[1..].iter().fold(first, |sum, a| sum * a),
            ))
        });

        local_env_add!("/", |args: List| -> Result<Expression> {
            let floats = util::parse_floats(args)?;
            let first = *floats
                .first()
                .ok_or_else(|| Error::Parse("expected at least one number".to_owned()))?;

            Ok(Expression::Number(
                floats[1..].iter().fold(first, |sum, a| sum / a),
            ))
        });

        local_env_add!("=", ensure_tonicity!(|a, b| a == b));
        local_env_add!(">", ensure_tonicity!(|a, b| a > b));
        local_env_add!(">=", ensure_tonicity!(|a, b| a >= b));
        local_env_add!("<", ensure_tonicity!(|a, b| a < b));
        local_env_add!("<=", ensure_tonicity!(|a, b| a <= b));

        // local_env_add!("define", |args: List| ->Result<Expression> {

        // })

        environment
    }
}

#[macro_export]
macro_rules! env_add {
    ($env:expr, ($ident:expr, $f:expr)) => {
        debug!("adding: {}", $ident);
        let function = func!($ident, $f);
        $env.data.insert($ident.to_owned(), function);
    };
}

macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: List| -> Result<Expression> {
            let floats = util::parse_floats(args)?;
            let first = floats.first().ok_or(Error::Evaluation(
                "expected at least one number".to_string(),
            ))?;

            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            }

            Ok(Expression::Bool(f(first, rest)))
        }
    }};
}

pub(crate) use ensure_tonicity;
