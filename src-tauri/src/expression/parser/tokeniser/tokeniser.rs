use super::{
    super::Position,
    error::{TokenError, TokenResult},
    token::{Token, TokenType},
};
use std::{collections::VecDeque, vec::IntoIter};

use log::{debug, error, trace};

pub struct Tokeniser {
    source: Option<IntoIter<char>>,
    pub position: Position,
    buffer: VecDeque<char>,
    bufsiz: usize,
    exhausted: bool,
}

impl std::fmt::Debug for Tokeniser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokeniser")
            .field("position", &self.position)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl Default for Tokeniser {
    fn default() -> Self {
        Tokeniser {
            source: None,
            position: Position { line: 1, column: 1 },
            buffer: VecDeque::new(),
            bufsiz: 10,
            exhausted: false,
        }
    }
}

impl Tokeniser {
    pub fn new() -> Self {
        Default::default()
    }
    fn init(&mut self, source: IntoIter<char>) {
        self.source = Some(source);
    }

    pub fn from_string(input: &String) -> Self {
        let mut tokeniser = Tokeniser::new();
        tokeniser.init(input.chars().collect::<Vec<char>>().into_iter());
        tokeniser
    }

    fn pad_buffer(&mut self) -> TokenResult<()> {
        if let Some(source) = &mut self.source {
            for _ in 0..(self.bufsiz - self.buffer.len()) {
                self.buffer.push_back(source.next().unwrap_or('\0'));
            }
            Ok(())
        } else {
            Err(TokenError::no_source())
        }
    }

    fn advance(&mut self) -> TokenResult<()> {
        if self.buffer.is_empty() {
            self.pad_buffer()?;
        }
        if let Some(c) = self.buffer.pop_front() {
            if c == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
        };
        trace!("Advancing, current buffer: {:?}", self.buffer);
        Ok(())
    }

    fn get_char(&mut self) -> TokenResult<char> {
        if self.buffer.len() < self.bufsiz / 2 {
            self.pad_buffer()?;
        }
        Ok(self
            .buffer
            .front()
            .copied()
            .ok_or(TokenError::exhausted(1))?)
    }

    pub fn get_token(&mut self) -> TokenResult<Token> {
        if self.exhausted {
            return Err(TokenError::exhausted(0));
        }
        let mut c = self.get_char()?;
        let mut value = String::new();
        let start = self.position;

        match c {
            '0'..='9' => {
                trace!("Found number");
                while c.is_digit(10) {
                    value.push(c);
                    self.advance()?;
                    c = self.get_char()?;
                }
                debug!("Number: {}", value);
                Ok(Token::number(value.parse()?, &start))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                trace!("Found identifier");
                while c.is_alphanumeric() {
                    value.push(c);
                    self.advance()?;
                    c = self.get_char()?;
                }
                if value == "true" || value == "false" {
                    debug!("Boolean: {}", value);
                    Ok(Token::boolean(
                        if value == "true" { true } else { false },
                        &start,
                    ))
                } else {
                    debug!("Identifier: {}", value);
                    Ok(Token::identifier(value, &start))
                }
            }
            '"' | '\'' => {
                trace!("Found string");
                let quote = c;
                trace!("Quote: {}", quote);
                self.advance()?;
                c = self.get_char()?;
                while c != quote {
                    if c == '\\' {
                        trace!("Found escape character");
                        self.advance()?;
                        c = self.get_char()?;
                        match c {
                            'n' => {
                                trace!("Found newline");
                                value.push('\n')
                            }
                            't' => {
                                trace!("Found tab");
                                value.push('\t')
                            }
                            '\\' => {
                                trace!("Found backslash");
                                value.push('\\')
                            }
                            '\'' => {
                                trace!("Found single quote");
                                value.push('\'')
                            }
                            '"' => {
                                trace!("Found double quote");
                                value.push('"')
                            }
                            _ => {
                                error!("Unexpected escape character: \\{}", c);
                                return Err(TokenError::unexpected(c, self.position));
                            }
                        }
                        self.advance()?;
                        c = self.get_char()?;
                    } else {
                        value.push(c);
                        self.advance()?;
                        c = self.get_char()?;
                    }
                }
                trace!("Found closing quote");
                self.advance()?;
                debug!("String: {}", value);
                Ok(Token::string(value, &start))
            }
            ' ' | '\t' | '\n' => {
                trace!("Found whitespace");
                while c.is_whitespace() {
                    self.advance()?;
                    c = self.get_char()?;
                }
                trace!("Whitespace ended");
                self.get_token()
            }
            '@' => {
                self.advance()?;
                debug!("Symbol: @");
                Ok(Token::symbol(TokenType::At, &start))
            }
            '&' => {
                self.advance()?;
                debug!("Symbol: &");
                Ok(Token::symbol(TokenType::And, &start))
            }
            '|' => {
                self.advance()?;
                debug!("Symbol: |");
                Ok(Token::symbol(TokenType::Or, &start))
            }
            '=' => {
                trace!("Found =");
                self.advance()?;
                c = self.get_char()?;
                if c == '=' {
                    trace!("Found =");
                    self.advance()?;
                    debug!("Symbol: ==");
                    Ok(Token::symbol(TokenType::Equal, &start))
                } else {
                    debug!("Symbol: =");
                    Ok(Token::symbol(TokenType::Assign, &start))
                }
            }
            '<' => {
                trace!("Found <");
                self.advance()?;
                c = self.get_char()?;
                if c == '=' {
                    trace!("Found =");
                    self.advance()?;
                    debug!("Symbol: <=");
                    Ok(Token::symbol(TokenType::LTEqual, &start))
                } else {
                    debug!("Symbol: <");
                    Ok(Token::symbol(TokenType::LessThan, &start))
                }
            }
            '>' => {
                trace!("Found >");
                self.advance()?;
                c = self.get_char()?;
                if c == '=' {
                    trace!("Found =");
                    self.advance()?;
                    debug!("Symbol: >=");
                    Ok(Token::symbol(TokenType::GTEqual, &start))
                } else {
                    debug!("Symbol: >");
                    Ok(Token::symbol(TokenType::GreaterThan, &start))
                }
            }
            '!' => {
                trace!("Found !");
                self.advance()?;
                c = self.get_char()?;
                if c == '=' {
                    trace!("Found =");
                    self.advance()?;
                    debug!("Symbol: !=");
                    Ok(Token::symbol(TokenType::NEqual, &start))
                } else {
                    debug!("Symbol: !");
                    Ok(Token::symbol(TokenType::Not, &start))
                }
            }
            '(' => {
                self.advance()?;
                debug!("Symbol: (");
                Ok(Token::symbol(TokenType::OpenParen, &start))
            }
            ')' => {
                self.advance()?;
                debug!("Symbol: )");
                Ok(Token::symbol(TokenType::CloseParen, &start))
            }
            '[' => {
                self.advance()?;
                debug!("Symbol: [");
                Ok(Token::symbol(TokenType::OpenBracket, &start))
            }
            ']' => {
                self.advance()?;
                debug!("Symbol: ]");
                Ok(Token::symbol(TokenType::CloseBracket, &start))
            }
            '{' => {
                self.advance()?;
                debug!("Symbol: {{");
                Ok(Token::symbol(TokenType::OpenBrace, &start))
            }
            '}' => {
                self.advance()?;
                debug!("Symbol: }}");
                Ok(Token::symbol(TokenType::CloseBrace, &start))
            }
            ',' => {
                self.advance()?;
                debug!("Symbol: ,");
                Ok(Token::symbol(TokenType::Comma, &start))
            }
            ':' => {
                self.advance()?;
                debug!("Symbol: :");
                Ok(Token::symbol(TokenType::Colon, &start))
            }
            '+' => {
                self.advance()?;
                debug!("Symbol: +");
                Ok(Token::symbol(TokenType::Plus, &start))
            }
            '-' => {
                self.advance()?;
                debug!("Symbol: -");
                Ok(Token::symbol(TokenType::Minus, &start))
            }
            '*' => {
                self.advance()?;
                debug!("Symbol: *");
                Ok(Token::symbol(TokenType::Mul, &start))
            }
            '/' => {
                self.advance()?;
                debug!("Symbol: /");
                Ok(Token::symbol(TokenType::Div, &start))
            }
            '%' => {
                self.advance()?;
                debug!("Symbol: %");
                Ok(Token::symbol(TokenType::Mod, &start))
            }
            '^' => {
                self.advance()?;
                debug!("Symbol: ^");
                Ok(Token::symbol(TokenType::Exp, &start))
            }
            '#' => {
                self.advance()?;
                debug!("Symbol: #");
                Ok(Token::symbol(TokenType::Concat, &start))
            }
            '\0' => {
                debug!("End of input");
                self.exhausted = true;
                Ok(Token::symbol(TokenType::EOF, &start))
            }
            _ => {
                error!("Unexpected character: {}", c);
                Err(TokenError::unexpected(c, start))
            }
        }
    }
}

impl std::iter::Iterator for Tokeniser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        if let Some(_) = self.source {
            let t = self.get_token().ok();
            if let Some(token) = t {
                if token.token_type == TokenType::EOF {
                    self.exhausted = true;
                }
                Some(token)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn sanity() {
        println!("Tokeniser test");
        for token in super::Tokeniser::from_string(
            &r#"123 true "string" | & ! == != <= >= >< + - * / % ^"#.to_string(),
        ) {
            println!("Token found: {:?}", token.token_type);
            if token.token_type == super::TokenType::EOF {
                break;
            }
        }
    }
}
