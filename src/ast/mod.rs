pub mod expressions;
pub mod statements;

use expressions::IdentExpression;
use statements::{LetStatement, ReturnStatement};

pub trait NodeTrait {
    fn token_literal(&self) -> &str;
}

// Using trait inheritance
pub trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

pub trait ExpressionTrait {
    fn expression_node(&self);
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        use Statement::*;
        match self {
            Let(s) => s.token_literal(),
            Return(s) => s.token_literal(),
        }
    }
}

pub enum Expression {
    Ident(IdentExpression),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl NodeTrait for Program {
    fn token_literal(&self) -> &str {
        // TDOO: Should this return a Option<String> ?
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}
