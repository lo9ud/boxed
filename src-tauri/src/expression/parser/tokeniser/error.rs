use std::num::ParseFloatError;

use super::super::Position;
use log::warn;

pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug)]
pub struct TokenError {
    pub message: String,
    pub position: Option<Position>,
}
impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.position {
            Some(position) => write!(
                f,
                " [{:0>3}:{:0>3}] {}",
                position.line, position.column, self.message
            ),
            None => write!(f, "{}", self.message),
        }
    }
}
impl std::error::Error for TokenError {}
impl TokenError {
    fn new(message: &str, position: Option<Position>) -> Self {
        TokenError {
            message: message.to_string(),
            position,
        }
    }
    pub fn expected(expected: &[&str], found: &str, position: Position) -> Self {
        TokenError {
            message: format!("Expected {:?} but found {}", expected, found),
            position: Some(position),
        }
    }
    pub fn unexpected(found: char, position: Position) -> Self {
        TokenError {
            message: format!("Disallowed char {} found", found),
            position: Some(position),
        }
    }
    pub fn exhausted(read_size: usize) -> Self {
        TokenError {
            message: format!("Attempted {} char read on exhausted source", read_size),
            position: None,
        }
    }
    pub fn no_source() -> Self {
        TokenError {
            message: "No source provided".to_string(),
            position: None,
        }
    }
    pub fn bad_source(source: &str) -> Self {
        TokenError {
            message: format!("Cannot read from source {}", source),
            position: None,
        }
    }
    pub fn read_failed() -> Self {
        TokenError {
            message: "Failed to read from source".to_string(),
            position: None,
        }
    }
    pub fn warn(&self) {
        warn!("{}", self);
    }
}

impl From<ParseFloatError> for TokenError {
    fn from(error: ParseFloatError) -> Self {
        TokenError {
            message: format!("Failed to parse float: {}", error.to_string()),
            position: None,
        }
    }
}
