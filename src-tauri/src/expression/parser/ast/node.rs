/*
 * <start> ::= '=' <or_expr>
 * <or_expr> ::= <and_expr> {'or' <and_expr>}
 * <and_expr> ::= <not_expr> {'and' <not_expr>}
 * <not_expr> ::= '!' <not_expr> | <cmp_expr>
 * <cmp_expr> ::= <add_expr> {('==' | '!=' | '<' | '<=' | '>' | '>=') <add_expr>}
 * <add_expr> ::= <mul_expr> {('+' | '-') <mul_expr>}
 * <mul_expr> ::= <pow_expr> {('*' | '/' | '%') <pow_expr>}
 * <pow_expr> ::= <unary_expr> {'^' <unary_expr>}
 * <unary_expr> ::= {'-'} <primary_expr>
 * <primary_expr> ::= <number> | <string> | <boolean> | <variable> | <function> | '(' <or_expr> ')'
 * <function> ::= <identifier> <arg_list>
 * <variable> ::= <identifier> | <col_spec>
 * <col_spec> ::= ':' <number> [ <colfilter> ]
 * <colfilter> ::= ('rand' | 'min' | 'max' | <identifier>) '@' <number>
 * <arg_list> ::= '(' [<or_expr> {',' <or_expr>}] ')'
 */

use log::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

impl std::fmt::Display for BinaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOpType::Add => write!(f, "+"),
            BinaryOpType::Sub => write!(f, "-"),
            BinaryOpType::Mul => write!(f, "*"),
            BinaryOpType::Div => write!(f, "/"),
            BinaryOpType::Mod => write!(f, "%"),
            BinaryOpType::Pow => write!(f, "^"),
            BinaryOpType::Eq => write!(f, "=="),
            BinaryOpType::Ne => write!(f, "!="),
            BinaryOpType::Lt => write!(f, "<"),
            BinaryOpType::Le => write!(f, "<="),
            BinaryOpType::Gt => write!(f, ">"),
            BinaryOpType::Ge => write!(f, ">="),
            BinaryOpType::And => write!(f, "&"),
            BinaryOpType::Or => write!(f, "|"),
        }
    }
}

impl BinaryOpType {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOpType::Or => 1,
            BinaryOpType::And => 2,
            BinaryOpType::Eq
            | BinaryOpType::Ne
            | BinaryOpType::Lt
            | BinaryOpType::Le
            | BinaryOpType::Gt
            | BinaryOpType::Ge => 4,
            BinaryOpType::Add | BinaryOpType::Sub => 5,
            BinaryOpType::Mul | BinaryOpType::Div | BinaryOpType::Mod => 6,
            BinaryOpType::Pow => 7,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpType {
    Neg,
    Not,
}

impl std::fmt::Display for UnaryOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOpType::Neg => write!(f, "-"),
            UnaryOpType::Not => write!(f, "!"),
        }
    }
}

impl UnaryOpType {
    fn precedence(&self) -> u8 {
        match self {
            UnaryOpType::Neg => 8,
            UnaryOpType::Not => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum Node {
    BinaryOp {
        left: Box<Node>,
        right: Option<(BinaryOpType, Box<Node>)>,
    },
    UnaryOp {
        op: Option<UnaryOpType>,
        right: Box<Node>,
    },
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(Identifier),
    Function {
        name: Identifier,
        args: Vec<Node>,
    },
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::BinaryOp { left, right } => match right {
                Some((op, right)) => write!(f, "({} {} {})", left, op, right),
                None => write!(f, "{}", left),
            },
            Node::UnaryOp { op, right } => match op {
                Some(op) => write!(f, "{}{}", op, right),
                None => write!(f, "{}", right),
            },
            Node::Number(n) => write!(f, "{}", n),
            Node::String(s) => write!(f, "{}", s),
            Node::Boolean(b) => write!(f, "{}", b),
            Node::Identifier(id) => write!(f, "{}", id.0),
            Node::Function { name, args } => {
                write!(f, "{}(", name.0)?;
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;
                    if i < args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl Node {
    pub fn pprint(&self, indent: usize) {
        match self {
            Node::BinaryOp { left, right } => {
                if let Some((op, right)) = right {
                    println!(
                        "{:indent$}<BinaryOp BinOpType ={:?}>",
                        "",
                        op,
                        indent = indent
                    );
                    left.pprint(indent + 2);
                    right.pprint(indent + 2);
                } else {
                    println!("{:indent$}<BinaryOp>", "", indent = indent);
                    left.pprint(indent + 2);
                }
                println!("{:indent$}</BinaryOp>", "", indent = indent);
            }
            Node::UnaryOp { op, right } => {
                if let Some(op) = op {
                    println!(
                        "{:indent$}<UnaryOp UnaryOpType ={:?}>",
                        "",
                        op,
                        indent = indent
                    );
                } else {
                    println!("{:indent$}<UnaryOp>", "", indent = indent);
                }
                right.pprint(indent + 2);
                println!("{:indent$}</UnaryOp>", "", indent = indent);
            }
            Node::Number(n) => {
                println!("{:indent$}<Number ={:?}>", "", n, indent = indent);
            }
            Node::String(s) => {
                println!("{:indent$}<String ={:?}>", "", s, indent = indent);
            }
            Node::Boolean(b) => {
                println!("{:indent$}<Boolean ={:?}>", "", b, indent = indent);
            }
            Node::Identifier(id) => {
                println!("{:indent$}<Identifier ={:?}>", "", id, indent = indent);
            }
            Node::Function { name, args } => {
                println!(
                    "{:indent$}<Function FunctionName ={:?}>",
                    "",
                    name,
                    indent = indent
                );
                for arg in args {
                    arg.pprint(indent + 2);
                }
                println!("{:indent$}</Function>", "", indent = indent);
            }
        }
    }

    pub fn reduce(&self) -> Box<Node> {
        return match self {
            Node::BinaryOp { left, right } => match right {
                Some((op, right)) => {
                    left.reduce();
                    right.reduce();
                    Box::new(self.clone())
                }
                None => left.reduce(),
            },

            Node::UnaryOp { op, right } => match op {
                Some(op) => {
                    right.reduce();
                    Box::new(self.clone())
                }
                None => right.reduce(),
            },

            Node::Function { name, args } => {
                for arg in args {
                    arg.reduce();
                }
                Box::new(self.clone())
            }
            _ => Box::new(self.clone()),
        };
    }

    fn precedence(&self) -> u8 {
        match self {
            Node::BinaryOp {
                left: _,
                right: Some((op, _)),
            } => op.precedence(),
            Node::BinaryOp { left, right: None } => left.precedence(),
            Node::UnaryOp {
                op: Some(op),
                right: _,
            } => op.precedence(),
            _ => u8::MAX,
        }
    }

    pub fn make_expr(&self) -> String {
        match self {
            Node::BinaryOp {
                left,
                right: Some((op, right)),
            } => {
                debug!("making expr from binary op {}|{}|{} ", left, op, right);
                let mut s = "".to_string();
                info!(
                    "left prec: {}, self prec: {}, self > left: {}",
                    left.precedence(),
                    self.precedence(),
                    self.precedence() > left.precedence()
                );
                if self.precedence() > left.precedence() {
                    debug!(
                        "left {} ({}) precedes self {} ({}), wrapping in parens for left",
                        left,
                        left.precedence(),
                        self,
                        self.precedence()
                    );
                    s.push_str(&format!("({})", left.make_expr()));
                } else {
                    debug!(
                        "left {} ({}) does not precede self {} ({}), no wrapping for left",
                        left,
                        left.precedence(),
                        self,
                        self.precedence()
                    );
                    s.push_str(&left.make_expr());
                }

                s.push_str(&format!(" {}", op));

                info!(
                    "right prec: {}, self prec: {}, self > right: {}",
                    right.precedence(),
                    self.precedence(),
                    self.precedence() > right.precedence()
                );
                if self.precedence() > right.precedence() {
                    debug!(
                        "right {} ({}) precedes self {} ({}), wrapping in parens for right",
                        right,
                        right.precedence(),
                        self,
                        self.precedence()
                    );
                    s.push_str(&format!(" ({})", right.make_expr()));
                } else {
                    debug!(
                        "right {} ({}) does not precede self {} ({}), no wrapping for right",
                        right,
                        right.precedence(),
                        self,
                        self.precedence()
                    );
                    s.push_str(&format!(" {}", right.make_expr()));
                }
                debug!("returning {}", s);
                s
            }
            Node::Function { name, args } => {
                let mut s = name.0.clone();
                s.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    s.push_str(&arg.make_expr());
                    if i < args.len() - 1 {
                        s.push_str(", ");
                    }
                }
                s.push_str(")");
                s
            }
            _ => format!("{}", self),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    static LOGGER: once_cell::sync::Lazy<flexi_logger::LoggerHandle> =
        once_cell::sync::Lazy::new(|| {
            flexi_logger::Logger::try_with_str("trace")
                .unwrap()
                .start()
                .unwrap()
        });

    #[test]
    fn make_expr() {
        let _logger = &LOGGER;
        let ast = Node::BinaryOp {
            left: Box::new(Node::Number(1.0)),
            right: Some((BinaryOpType::Add, Box::new(Node::Number(2.0)))),
        };
        let ast = Node::BinaryOp {
            left: Box::new(ast.clone()),
            right: Some((BinaryOpType::Mul, Box::new(ast))),
        };
        let ast = Node::BinaryOp {
            left: Box::new(ast),
            right: Some((BinaryOpType::Pow, Box::new(Node::Number(8.0)))),
        };
        let ast = Node::BinaryOp {
            left: Box::new(Node::Number(5.0)),
            right: Some((BinaryOpType::Add, Box::new(ast))),
        };
        println!("{}", ast.make_expr());
    }

    #[test]
    fn test_ast_reduce() {
        let _logger = &LOGGER;
        let ast = Node::BinaryOp {
            left: Box::new(Node::BinaryOp {
                left: Box::new(Node::BinaryOp {
                    left: Box::new(Node::BinaryOp {
                        left: Box::new(Node::BinaryOp {
                            left: Box::new(Node::Number(1.0)),
                            right: Some((BinaryOpType::Add, Box::new(Node::Number(2.0)))),
                        }),
                        right: None,
                    }),
                    right: None,
                }),
                right: Some((BinaryOpType::Mul, Box::new(Node::Number(3.0)))),
            }),
            right: Some((BinaryOpType::Pow, Box::new(Node::Number(4.0)))),
        };
        let reduced_ast = ast.reduce();
        println!("Original AST:");
        ast.pprint(0);
        println!("\nReduced AST:");
        reduced_ast.pprint(0);
    }
}
