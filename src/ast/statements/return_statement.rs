use std::fmt::Display;

use crate::{
    ast::{Expression, NodeTrait, StatementTrait},
    token::Token,
};

pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.value)
    }
}

impl NodeTrait for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl StatementTrait for ReturnStatement {
    fn statement_node(&self) {
        todo!()
    }
}
