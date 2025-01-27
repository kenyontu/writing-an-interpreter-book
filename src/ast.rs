use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

// Using trait inheritance
pub trait StatementTrait: Node {
    fn statement_node(&self);
}

pub enum Statement {
    Let(LetStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        use Statement::*;
        match self {
            Let(s) => s.token_literal(),
        }
    }
}

pub trait Expression {
    fn expression_node(&self);
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

impl Node for Program {
    fn token_literal(&self) -> &str {
        // TDOO: Should this return a Option<String> ?
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl StatementTrait for LetStatement {
    fn statement_node(&self) {}
}
