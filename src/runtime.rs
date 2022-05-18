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
            environment,
            parser: Parser::default(),
        }
    }

    pub fn evaluate(&'a mut self, expression: &'a Expression) -> Result<Expression> {
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

    fn evaluate_list(&'a mut self, list: &'a List) -> Result<Expression> {
        let first = list
            .first()
            .ok_or_else(|| Error::Evaluation("expected a non-empty list".to_owned()))?;
        let rest = &list[1..];

        match self.evaluate_builtin(first, rest) {
            Some(res) => res,
            None => match self.evaluate(first)? {
                Expression::Function(function) => function.call(
                    rest.iter()
                        .map(|x| self.evaluate(x))
                        .collect::<Result<Vec<Expression>>>()?,
                ),
                _ => Err(Error::Evaluation(
                    "first form must be a function".to_owned(),
                )),
            },
        }
    }

    fn evaluate_builtin(
        &mut self,
        expression: &Expression,
        args: &[Expression],
    ) -> Option<Result<Expression>> {
        match expression {
            Expression::Symbol(symbol) => match symbol.as_ref() {
                "if" => Some(self.evaluate_if(args)),
                "define" => Some(self.evaluate_define(args)),
                _ => None,
            },
            _ => None,
        }
    }

    fn evaluate_if(&mut self, args: &[Expression]) -> Result<Expression> {
        let test_form = args
            .first()
            .ok_or_else(|| Error::Evaluation("expected test form".to_owned()))?;
        let test_eval = self.evaluate(test_form)?;
        match test_eval {
            Expression::Bool(b) => {
                let form_idx = if b { 1 } else { 2 };
                let res_form = args
                    .get(form_idx)
                    .ok_or_else(|| Error::Evaluation(format!("expected form idx={}", form_idx)))?;

                self.evaluate(res_form)
            }
            _ => Err(Error::Evaluation(format!(
                "unexpected test form='{}'",
                test_form.to_owned()
            ))),
        }
    }

    fn evaluate_define(&mut self, args: &[Expression]) -> Result<Expression> {
        let first_form = args
            .first()
            .ok_or_else(|| Error::Evaluation("expected first form".to_owned()))?;
        let first_str = match first_form {
            Expression::Symbol(s) => Ok(s.clone()),
            _ => Err(Error::Evaluation(
                "expected first form to be a symbol".to_owned(),
            )),
        }?;
        let second_form = args
            .get(1)
            .ok_or_else(|| Error::Evaluation("expected second form".to_owned()))?;
        if args.len() > 2 {
            return Err(Error::Evaluation("def can only have two forms ".to_owned()));
        }
        let second_eval = self.evaluate(second_form)?;
        self.environment.set(&first_str, second_eval);

        Ok(first_form.clone())
    }
}
