use tracing::debug;

use crate::{expression::List, Environment, Error, Expression, Parser, Result};

#[derive(Default)]
pub struct Runtime {
    environment: Environment,
    parser: Parser,
}

impl<'a> Runtime {
    pub fn new(environment: Environment) -> Self {
        Self {
            environment: environment,
            parser: Parser::default(),
        }
    }

    pub fn evaluate(&'a self, expression: &'a Expression) -> Result<Expression> {
        debug!("evaluation: {expression}");

        match expression {
            Expression::Symbol(identifier) => self.evaluate_symbol(identifier),
            Expression::Number(_) | Expression::Bool(_) => Ok(expression.clone()),
            Expression::List(list) => self.evaluate_list(list),
            Expression::Function(_) => Err(Error::Evaluation("unexpected form".to_owned())),
        }
    }

    pub fn parse(&self, expression: &str) -> Result<(Expression, Vec<String>)> {
        self.parser.parse(expression)
    }

    fn evaluate_symbol(&'a self, identifier: &str) -> Result<Expression> {
        self.environment
            .get(identifier)
            .ok_or_else(|| Error::Evaluation(format!("unexpected symbol {identifier}")))
            .map(|expr| expr.clone())
    }

    fn evaluate_list(&'a self, list: &'a List) -> Result<Expression> {
        let first = list
            .first()
            .ok_or_else(|| Error::Evaluation("expected a non-empty list".to_owned()))?;
        let rest = &list[1..];

        let first = self.evaluate(first)?;
        match first {
            Expression::Function(function) => {
                let mut args = vec![];
                for arg in rest {
                    let arg = self.evaluate(arg)?;
                    args.push(arg);
                }

                function.call(args)
            }
            _ => Err(Error::Evaluation(
                "first form must be a function".to_owned(),
            )),
        }
    }
}
