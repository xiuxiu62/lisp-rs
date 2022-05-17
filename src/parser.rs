use crate::{Error, Expression, Lexer, Result};
use std::num::ParseFloatError;
use tracing::debug;

#[derive(Debug, Default)]
pub struct Parser(Lexer);

impl Parser {
    pub fn parse(&self, expression: &str) -> Result<(Expression, Vec<String>)> {
        debug!("parsing: {expression}");

        let tokens = self.tokenize(expression);
        self.parse_tokens(tokens)
    }

    fn parse_tokens(&self, tokens: Vec<String>) -> Result<(Expression, Vec<String>)> {
        let (token, rest) = tokens
            .split_first()
            .ok_or_else(|| Error::Parse("could not get token".to_owned()))?;

        match &token[..] {
            "(" => self.read_seq(rest.to_owned()),
            ")" => Err(Error::Parse("unexpected `)`".to_owned())),
            _ => Ok((self.parse_atom(token), rest.to_owned())),
        }
    }

    fn read_seq(&self, tokens: Vec<String>) -> Result<(Expression, Vec<String>)> {
        let mut res: Vec<Expression> = vec![];
        let mut xs = tokens;

        loop {
            let (next_token, rest) = xs
                .split_first()
                .ok_or_else(|| Error::Parse("could not find closing `)`".to_owned()))?;

            if *next_token == ")" {
                return Ok((Expression::List(res), rest.to_owned())); // skip `)`, head to the token after
            }

            let (exp, new_xs) = self.parse_tokens(xs)?;
            res.push(exp);
            xs = new_xs;
        }
    }

    fn parse_atom(&self, token: &str) -> Expression {
        let potential_float: std::result::Result<f64, ParseFloatError> = token.parse();
        match potential_float {
            Ok(v) => Expression::Number(v),
            Err(_) => Expression::Symbol(token.to_owned()),
        }
    }

    fn tokenize(&self, expression: &str) -> Vec<String> {
        self.0.tokenize(expression)
    }
}
