use crate::{
    ast::{ExpressionTrait, NodeTrait},
    token::Token,
};

pub struct IdentExpression {
    pub token: Token,
    pub value: String,
}

impl NodeTrait for IdentExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl ExpressionTrait for IdentExpression {
    fn expression_node(&self) {}
}
