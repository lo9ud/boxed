use super::{
    parser::{ParseError, Position},
    ValueType,
};

#[derive(Debug)]
pub struct ExpressionError {
    message: String,
    position: Option<Position>,
    source: Option<ParseError>,
}

pub type ExpressionResult<T> = Result<T, ExpressionError>;

impl std::error::Error for ExpressionError {}

impl std::fmt::Display for ExpressionError {
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

impl ExpressionError {
    pub fn new(message: &str, position: Option<&Position>) -> Self {
        ExpressionError {
            message: message.to_string(),
            position: position.cloned(),
            source: None,
        }
    }

    pub fn missing_arguments() -> Self {
        ExpressionError {
            message: "Expected at least one argument".to_string(),
            position: None,
            source: None,
        }
    }

    pub fn type_error(expected: &ValueType, found: &ValueType, position: Position) -> Self {
        ExpressionError {
            message: format!("Type {:?} not valid, require {:?}", expected, found),
            position: Some(position),
            source: None,
        }
    }
}
