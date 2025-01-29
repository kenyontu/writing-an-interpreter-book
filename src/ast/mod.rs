pub mod expressions;
pub mod statements;

use std::fmt::Display;

use expressions::IdentExpression;
use statements::{ExpressionStatement, LetStatement, ReturnStatement};

pub trait NodeTrait: Display {
    fn token_literal(&self) -> &str;
}

// Using trait inheritance
pub trait StatementTrait: NodeTrait {
    fn statement_node(&self);
}

pub trait ExpressionTrait: NodeTrait {
    fn expression_node(&self);
}

pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    pub fn token_literal(&self) -> &str {
        use Statement::*;
        match self {
            Let(s) => s.token_literal(),
            Return(s) => s.token_literal(),
            Expression(s) => s.token_literal(),
        }
    }

    pub fn to_string(&self) -> String {
        use Statement::*;
        match self {
            Let(s) => s.to_string(),
            Return(s) => s.to_string(),
            Expression(s) => s.to_string(),
        }
    }
}

pub enum Expression {
    Ident(IdentExpression),
}

impl Expression {
    pub fn to_string(&self) -> String {
        use Expression::*;
        match self {
            Ident(e) => e.to_string(),
        }
    }
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

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in self.statements.iter() {
            write!(f, "{}\n", stmt.to_string())?;
        }
        Ok(())
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

mod tests {
    use crate::token::{Token, TokenType};

    use super::*;

    #[test]
    fn test_to_string() {
        let statements: Vec<Statement> = vec![Statement::Let(LetStatement {
            token: Token {
                token_type: TokenType::Let,
                literal: "let".to_string(),
            },
            name: IdentExpression {
                token: Token {
                    token_type: TokenType::Ident,
                    literal: "myVar".to_string(),
                },
                value: "myVar".to_string(),
            },
            value: Expression::Ident(IdentExpression {
                token: Token {
                    token_type: TokenType::Ident,
                    literal: "anotherVar".to_string(),
                },
                value: "anotherVar".to_string(),
            }),
        })];

        let program = Program { statements };
        assert_eq!(program.to_string(), "let myVar = anotherVar;\n");
    }
}
