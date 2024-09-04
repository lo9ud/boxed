use log::{debug, trace};

use crate::expression::parser::{Position, Token, TokenType, TokenValue, Tokeniser};

use super::{
    error::ParseResult,
    node::{BinaryOpType, Identifier, Node, UnaryOpType},
    ParseError,
};

pub struct Assembler {
    source: Tokeniser,
    position: Position,
    current: Token,
}

impl Assembler {
    fn expect(&mut self, token_type: TokenType) -> ParseResult<()> {
        trace!("Expecting {token:?}, found {token:?}", token = token_type);
        if self.current.token_type == token_type {
            self.current = self.source.next().ok_or(ParseError::unexpected_eof())?;
            Ok(())
        } else {
            Err(ParseError::expected(
                &token_type,
                &self.current.token_type,
                self.current.position,
            ))
        }
    }

    fn advance(&mut self) {
        self.current = self.source.next().unwrap_or(Token::eof());
    }

    fn parse(&mut self) -> ParseResult<Node> {
        debug!("Parsing expression");
        self.expect(TokenType::Assign);
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_and_expr()?;
        while self.current.token_type == TokenType::Or {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((BinaryOpType::Or, Box::new(self.parse_and_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_and_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_not_expr()?;
        while self.current.token_type == TokenType::And {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((BinaryOpType::And, Box::new(self.parse_not_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_not_expr(&mut self) -> ParseResult<Node> {
        if self.current.token_type == TokenType::Not {
            self.advance();
            Ok(Node::UnaryOp {
                op: Some(UnaryOpType::Not),
                right: Box::new(self.parse_not_expr()?),
            })
        } else {
            self.parse_cmp_expr()
        }
    }

    fn parse_cmp_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_add_expr()?;
        while let Some(op) = match self.current.token_type {
            TokenType::Equal => Some(BinaryOpType::Eq),
            TokenType::NEqual => Some(BinaryOpType::Ne),
            TokenType::LessThan => Some(BinaryOpType::Lt),
            TokenType::GreaterThan => Some(BinaryOpType::Gt),
            TokenType::LTEqual => Some(BinaryOpType::Le),
            TokenType::GTEqual => Some(BinaryOpType::Ge),
            _ => None,
        } {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((op, Box::new(self.parse_add_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_add_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_mul_expr()?;
        while let Some(op) = match self.current.token_type {
            TokenType::Plus => Some(BinaryOpType::Add),
            TokenType::Minus => Some(BinaryOpType::Sub),
            _ => None,
        } {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((op, Box::new(self.parse_mul_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_mul_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_pow_expr()?;
        while let Some(op) = match self.current.token_type {
            TokenType::Mul => Some(BinaryOpType::Mul),
            TokenType::Div => Some(BinaryOpType::Div),
            TokenType::Mod => Some(BinaryOpType::Mod),
            _ => None,
        } {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((op, Box::new(self.parse_pow_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_pow_expr(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_unary_expr()?;
        while self.current.token_type == TokenType::Exp {
            self.advance();
            node = Node::BinaryOp {
                left: Box::new(node),
                right: Some((BinaryOpType::Pow, Box::new(self.parse_unary_expr()?))),
            };
        }
        Ok(node)
    }

    fn parse_unary_expr(&mut self) -> ParseResult<Node> {
        if self.current.token_type == TokenType::Minus {
            self.advance();
            Ok(Node::UnaryOp {
                op: Some(UnaryOpType::Neg),
                right: Box::new(self.parse_primary()?),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> ParseResult<Node> {
        match self.current.token_type {
            TokenType::String => {
                let value = match self.current.value {
                    TokenValue::String(ref s) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance();
                Ok(Node::String(value))
            }
            TokenType::Number => {
                let value = match self.current.value {
                    TokenValue::Number(n) => n,
                    _ => unreachable!(),
                };
                self.advance();
                Ok(Node::Number(value))
            }
            TokenType::Boolean => {
                let value = match self.current.value {
                    TokenValue::Boolean(b) => b,
                    _ => unreachable!(),
                };
                self.advance();
                Ok(Node::Boolean(value))
            }
            TokenType::Identifier => {
                let value = match self.current.value {
                    TokenValue::String(ref s) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance();
                if self.current.token_type == TokenType::OpenParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current.token_type != TokenType::CloseParen {
                        args.push(self.parse_or_expr()?);
                        if self.current.token_type == TokenType::Comma {
                            self.advance();
                        }
                    }
                    self.expect(TokenType::CloseParen)?;
                    Ok(Node::Function {
                        name: Identifier(value),
                        args,
                    })
                } else {
                    Ok(Node::Identifier(Identifier(value)))
                }
            }
            TokenType::OpenParen => {
                self.advance();
                let node = self.parse_or_expr()?;
                self.expect(TokenType::CloseParen)?;
                Ok(node)
            }
            _ => Err(ParseError::expected_primary(&self.current)),
        }
    }

    fn from_string(input: &str) -> ParseResult<Self> {
        let mut source = Tokeniser::from_string(&input.to_string());
        let t = source.next().ok_or(ParseError::unexpected_eof())?;
        Ok(Assembler {
            source,
            position: Position::new(0, 0),
            current: t,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use flexi_logger::{Logger, LoggerHandle};
    use log::info;

    fn init_logger() -> LoggerHandle {
        Logger::try_with_str("trace").unwrap().start().unwrap()
    }

    #[test]
    fn check_assembler() {
        let _logger = init_logger();

        let mut assembler = Assembler::from_string("=2*FUNC(hello, 3+2, 6          ^7)").unwrap();
        for _ in 0..10 {
            info!("{:?}", assembler.current);
            assembler.advance();
        }
    }

    #[test]
    fn test_assembler() {
        let _logger = init_logger();

        let mut assembler = Assembler::from_string("=2*FUNC(hello, 3+2, 6          ^7)").unwrap();
        match assembler.parse() {
            Ok(node) => {
                println!("{}", node.make_expr());
                node.pprint(0)
            }
            Err(e) => {
                e.error();
                panic!();
            }
        }
    }
}
