use tracing::debug;

#[derive(Debug, Default)]
pub struct Lexer;

impl Lexer {
    pub fn tokenize(&self, expression: &str) -> Vec<String> {
        debug!("lexing: {expression}");

        expression
            .replace('(', " ( ")
            .replace(')', " ) ")
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    }
}
