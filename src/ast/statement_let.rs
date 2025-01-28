use crate::token::Token;

use super::{expression_ident::IdentExpression, Expression, NodeTrait, StatementTrait};

pub struct LetStatement {
    pub token: Token,
    pub name: IdentExpression,
    pub value: Expression,
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {}
}
