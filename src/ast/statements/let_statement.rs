use std::fmt::Display;

use crate::{
    ast::{expressions::IdentExpression, Expression, NodeTrait, StatementTrait},
    token::Token,
};

pub struct LetStatement {
    pub token: Token,
    pub name: IdentExpression,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value
        )
    }
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {}
}
