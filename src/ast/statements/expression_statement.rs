use std::fmt::Display;

use crate::{
    ast::{Expression, NodeTrait, StatementTrait},
    token::Token,
};

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression.to_string())
    }
}

impl NodeTrait for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl StatementTrait for ExpressionStatement {
    fn statement_node(&self) {}
}
