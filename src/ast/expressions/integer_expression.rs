use std::fmt::Display;

use crate::{
    ast::{ExpressionTrait, NodeTrait},
    token::Token,
};

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl NodeTrait for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl ExpressionTrait for IntegerLiteral {
    fn expression_node(&self) {}
}
