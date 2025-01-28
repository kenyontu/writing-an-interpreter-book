use crate::token::Token;

use super::{Expression, NodeTrait, StatementTrait};

pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
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
