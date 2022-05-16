use crate::{Error, Expression, Result};
use std::num::ParseFloatError;

#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse<'a>(&self, tokens: &'a [String]) -> Result<(Expression, &'a [String])> {
        let (token, rest) = tokens
            .split_first()
            .ok_or(Error::Parse("could not get token".to_owned()))?;

        match &token[..] {
            "(" => self.read_seq(rest),
            ")" => Err(Error::Parse("unexpected `)`".to_owned())),
            _ => Ok((self.parse_atom(token), rest)),
        }
    }

    fn read_seq<'a>(&self, tokens: &'a [String]) -> Result<(Expression, &'a [String])> {
        let mut res: Vec<Expression> = vec![];
        let mut xs = tokens;

        loop {
            let (next_token, rest) = xs
                .split_first()
                .ok_or(Error::Parse("could not find closing `)`".to_owned()))?;

            if *next_token == ")" {
                return Ok((Expression::List(res), rest)); // skip `)`, head to the token after
            }

            let (exp, new_xs) = self.parse(&xs)?;
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
}
