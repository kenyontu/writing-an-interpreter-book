use std::fmt::Display;

use crate::{
    ast::{Expression, ExpressionTrait, NodeTrait},
    token::Token,
};

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}

impl NodeTrait for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl ExpressionTrait for InfixExpression {
    fn expression_node(&self) {}
}
