mod environment;
mod error;
#[macro_use]
mod expression;
mod lexer;
mod parser;

pub use environment::{Environment, EnvironmentBuilder};
pub use error::{Error, Result};
pub use expression::Expression;
pub use lexer::Lexer;
pub use parser::Parser;

#[cfg(test)]
mod tests {
    use super::{Expression, Lexer, Parser, Result};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn parse() -> Result<()> {
        let lexer = Lexer::default();
        let parser = Parser::default();

        let test_data = "(2 2 +)";
        let tokens = lexer.tokenize(test_data);
        let _exprs = parser.parse(&tokens)?;

        println!("{:?}", list!(num!(2), num!(2), sym!("+")));
        // assert_eq!(exprs.0, list!(vec![num!(2), num!(2), sym!("+")]));

        Ok(())
    }
}
