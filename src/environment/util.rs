use crate::{expression::List, Error, Expression, Result};

pub fn parse_floats(args: List) -> Result<Vec<f64>> {
    args.iter().map(parse_float).collect()
}

fn parse_float(expr: &Expression) -> Result<f64> {
    match expr {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error::Parse("expected a number".to_owned())),
    }
}
