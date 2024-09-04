use super::{error::ExpressionResult, parser::Node, Function};

pub struct Expression {
    root: ExprNode,
}

enum BinaryOpType {
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

impl BinaryOpType {
    fn from_binary_op(op: super::parser::ast::BinaryOpType) -> Self {
        match op {
            super::parser::ast::BinaryOpType::Add => Self::Add,
            super::parser::ast::BinaryOpType::Sub => Self::Sub,
            super::parser::ast::BinaryOpType::Mul => Self::Mul,
            super::parser::ast::BinaryOpType::Div => Self::Div,
            super::parser::ast::BinaryOpType::Mod => Self::Mod,
            super::parser::ast::BinaryOpType::Pow => Self::Pow,
            super::parser::ast::BinaryOpType::Eq => Self::Eq,
            super::parser::ast::BinaryOpType::Ne => Self::Ne,
            super::parser::ast::BinaryOpType::Lt => Self::Lt,
            super::parser::ast::BinaryOpType::Le => Self::Le,
            super::parser::ast::BinaryOpType::Gt => Self::Gt,
            super::parser::ast::BinaryOpType::Ge => Self::Ge,
            super::parser::ast::BinaryOpType::And => Self::And,
            super::parser::ast::BinaryOpType::Or => Self::Or,
        }
    }
}

enum UnaryOpType {
    Neg,
    Not,
}

impl UnaryOpType {
    fn from_unary_op(op: super::parser::ast::UnaryOpType) -> Self {
        match op {
            super::parser::ast::UnaryOpType::Neg => Self::Neg,
            super::parser::ast::UnaryOpType::Not => Self::Not,
        }
    }
}

enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl LiteralValue {
    fn from_literal(node: Node) -> Self {
        match node {
            Node::Number(value) => Self::Number(value),
            Node::Boolean(value) => Self::Boolean(value),
            Node::String(value) => Self::String(value),
            _ => unreachable!(),
        }
    }
}

enum ExprNode {
    BinaryOp {
        left: Box<ExprNode>,
        op: BinaryOpType,
        right: Box<ExprNode>,
    },
    UnaryOp {
        op: UnaryOpType,
        right: Box<ExprNode>,
    },
    Literal {
        value: LiteralValue,
    },
    Variable {
        name: String,
    },
    FunctionCall {
        target: Function,
        args: Vec<ExprNode>,
    },
}

impl ExprNode {
    fn static_eval() -> Option<LiteralValue> {
        todo!();
    }

    fn is_static(&self) -> bool {
        match self {
            Self::BinaryOp { left, right, .. } => left.is_static() && right.is_static(),
            Self::UnaryOp { right, .. } => right.is_static(),
            Self::Literal { .. } => true,
            Self::Variable { .. } => false,
            Self::FunctionCall { args, .. } => args.iter().all(Self::is_static),
        }
    }
}

impl Expression {
    pub fn from_ast(ast: Node) -> ExpressionResult<Self> {
        let ast = ast.reduce();
        Ok(Expression {
            root: Self::from_ast_node(*ast),
        })
    }

    fn from_ast_node(node: Node) -> ExprNode {
        match node {
            Node::BinaryOp {
                left,
                right: Some((op, right)),
            } => ExprNode::BinaryOp {
                left: Box::new(Self::from_ast_node(*left)),
                op: BinaryOpType::from_binary_op(op),
                right: Box::new(Self::from_ast_node(*right)),
            },
            Node::UnaryOp {
                op: Some(op),
                right,
            } => ExprNode::UnaryOp {
                op: UnaryOpType::from_unary_op(op),
                right: Box::new(Self::from_ast_node(*right)),
            },
            Node::Number(_) | Node::Boolean(_) | Node::String(_) => ExprNode::Literal {
                value: LiteralValue::from_literal(node),
            },
            Node::Identifier(name) => ExprNode::Variable { name: name.0 },
            Node::Function { name, args } => ExprNode::FunctionCall {
                target: Function::from_string(name.0),
                args: args.into_iter().map(Self::from_ast_node).collect(),
            },
            _ => unreachable!(),
        }
    }

    fn eval_static(&self) -> Self {
        unimplemented!()
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
    fn from_ast() {
        let _logger = &LOGGER;
        let ast = Node::BinaryOp {
            left: Box::new(Node::Number(1.0)),
            right: Some((
                super::super::parser::ast::BinaryOpType::Add,
                Box::new(Node::Number(2.0)),
            )),
        };
        let expr = Expression::from_ast(ast).unwrap();
    }
}
