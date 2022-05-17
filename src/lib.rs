mod environment;
mod error;
mod runtime;
#[macro_use]
mod expression;
mod lexer;
mod parser;

pub use environment::{Environment, EnvironmentBuilder};
pub use error::{Error, Result};
pub use expression::Expression;
pub use lexer::Lexer;
pub use parser::Parser;
pub use runtime::Runtime;

#[cfg(test)]
mod tests {
    use super::{Expression, Result, Runtime};

    #[test]
    fn parse() -> Result<()> {
        let runtime = Runtime::default();

        let data = "(+ 2 2)";
        // let expression = runtime.parse(data)?;
        // let result = runtime.evaluate(&expression.0)?;

        // println!("Result: {result}");
        println!("{:?}", list!(num!(2), num!(2), sym!("+")));

        Ok(())
    }
}
