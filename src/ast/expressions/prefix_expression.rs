use std::fmt::Display;

use crate::{
    ast::{Expression, ExpressionTrait, NodeTrait},
    token::Token,
};

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl NodeTrait for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl ExpressionTrait for PrefixExpression {
    fn expression_node(&self) {}
}
