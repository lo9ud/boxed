use super::super::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// A string
    String,
    /// A number
    Number,
    /// A boolean value
    Boolean,
    /// An identifier to a variable or function
    Identifier,
    /// ( - Open parenthesis
    OpenParen,
    /// ) - Close parenthesis
    CloseParen,
    /// [ - Open bracket
    OpenBracket,
    /// ] - Close bracket
    CloseBracket,
    /// { - Open brace
    OpenBrace,
    /// } - Close brace
    CloseBrace,
    /// , - Comma
    Comma,
    /// : - Colon
    Colon,
    /// + - Plus
    Plus,
    /// - - Minus
    Minus,
    /// * - Multiply
    Mul,
    /// / - Divide
    Div,
    /// % - Modulus
    Mod,
    /// ^ - Exponent
    Exp,
    /// # - Concatenate
    Concat,
    /// & - And
    And,
    /// | - Or
    Or,
    /// ! - Not
    Not,
    /// = - Assign
    Assign,
    /// == - Equal
    Equal,
    /// != - Not equal
    NEqual,
    /// < - Less than
    LessThan,
    /// > - Greater than
    GreaterThan,
    /// <= - Less than or equal
    LTEqual,
    /// >= - Greater than or equal
    GTEqual,
    /// @ - Column specifier
    At,
    /// EOF - End of file
    EOF,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Symbol,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: TokenValue,
    pub position: Position,
}

impl Token {
    pub fn string(value: String, position: &Position) -> Self {
        Token {
            token_type: TokenType::String,
            value: TokenValue::String(value),
            position: position.to_owned(),
        }
    }

    pub fn number(value: f64, position: &Position) -> Self {
        Token {
            token_type: TokenType::Number,
            value: TokenValue::Number(value),
            position: position.to_owned(),
        }
    }

    pub fn identifier(value: String, position: &Position) -> Self {
        Token {
            token_type: TokenType::Identifier,
            value: TokenValue::String(value),
            position: position.to_owned(),
        }
    }

    pub fn boolean(value: bool, position: &Position) -> Self {
        Token {
            token_type: TokenType::Boolean,
            value: TokenValue::Boolean(value),
            position: position.to_owned(),
        }
    }

    pub fn symbol(token_type: TokenType, position: &Position) -> Self {
        Token {
            token_type,
            value: TokenValue::Symbol,
            position: position.to_owned(),
        }
    }

    pub fn eof() -> Self {
        Token {
            token_type: TokenType::EOF,
            value: TokenValue::Symbol,
            position: Position::new(0, 0),
        }
    }

    pub fn matches(&self, token_type: &[&TokenType]) -> bool {
        token_type
            .iter()
            .any(|x| std::mem::discriminant(&self.token_type) == std::mem::discriminant(x))
    }
}
