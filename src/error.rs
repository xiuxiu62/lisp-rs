use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Evaluation(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Parse(message) => format!("parse error: {message}"),
            Self::Evaluation(message) => format!("evaluation error: {message}"),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for Error {}
