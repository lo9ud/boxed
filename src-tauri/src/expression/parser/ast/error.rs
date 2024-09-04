use crate::expression::parser::Token;

use super::super::{Position, TokenError, TokenType};
use log::{error, warn};

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: Option<Position>,
    source: Option<TokenError>,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
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

impl ParseError {
    fn new(message: &str, position: Option<&Position>) -> Self {
        ParseError {
            message: message.to_string(),
            position: position.cloned(),
            source: None,
        }
    }

    pub fn expected(expected: &TokenType, found: &TokenType, position: Position) -> Self {
        ParseError {
            message: format!("Expected {:?} but found {:?}", expected, found),
            position: Some(position),
            source: None,
        }
    }

    pub fn unexpected_eof() -> Self {
        ParseError {
            message: "Unexpected end of file".to_string(),
            position: None,
            source: None,
        }
    }

    pub fn expected_primary(token: &Token) -> Self {
        ParseError::new(
            &format!(
                "Expected primary expression ({:?}), found {:?}",
                &[
                    TokenType::String,
                    TokenType::Number,
                    TokenType::Identifier,
                    TokenType::OpenParen
                ],
                token.token_type
            ),
            Some(&token.position),
        )
    }

    pub fn warn(&self) {
        match self.source {
            Some(ref error) => error!("{}: {}", self, error),
            None => warn!("{}", self),
        }
    }

    pub fn error(&self) {
        match self.source {
            Some(ref error) => error!("{}: {}", self, error),
            None => error!("{}", self),
        }
    }
}

impl From<TokenError> for ParseError {
    fn from(error: TokenError) -> Self {
        ParseError {
            message: "Token error".to_string(),
            position: error.position,
            source: Some(error),
        }
    }
}
