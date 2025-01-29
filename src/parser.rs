use std::{borrow::BorrowMut, mem};

use crate::{
    ast::{
        self,
        expressions::IdentExpression,
        statements::{LetStatement, ReturnStatement},
        Expression,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub cur_token: Token,
    pub peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut lexer = lexer;
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.cur_token = mem::replace(self.peek_token.borrow_mut(), self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while !self.cur_token_is(&TokenType::Eof) {
            if let Some(s) = self.parse_statement() {
                program.statements.push(s);
            }

            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Illegal => None,
            TokenType::Eof => None,
            TokenType::Ident => None,
            TokenType::Int => None,
            TokenType::Assign => None,
            TokenType::Plus => None,
            TokenType::Minus => None,
            TokenType::Bang => None,
            TokenType::Asterisk => None,
            TokenType::Slash => None,
            TokenType::Comma => None,
            TokenType::LessThan => None,
            TokenType::GreaterThan => None,
            TokenType::Semicolon => None,
            TokenType::LeftParen => None,
            TokenType::RightParen => None,
            TokenType::LeftBrace => None,
            TokenType::RightBrace => None,
            TokenType::Function => None,
            TokenType::True => None,
            TokenType::False => None,
            TokenType::If => None,
            TokenType::Else => None,
            TokenType::Return => self.parse_return_statement(),
            TokenType::Equal => None,
            TokenType::NotEqual => None,
        }
    }

    fn cur_token_is(&self, token_type: &TokenType) -> bool {
        self.cur_token.token_type == token_type.clone()
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == token_type.clone()
    }

    fn expect_peek(&mut self, token_type: &TokenType) -> bool {
        if self.peek_token_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let name = IdentExpression {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(&TokenType::Assign) {
            return None;
        }

        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        // TODO: The book left the value undefined, and should come back
        // to this in the parsing expressions chapter, for now I will
        // assign a dummy value
        let dummy_value = IdentExpression {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        let let_stmt = LetStatement {
            token: self.cur_token.clone(),
            name,
            value: Expression::Ident(dummy_value),
        };

        Some(ast::Statement::Let(let_stmt))
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        let dummy_value = IdentExpression {
            token: Token {
                token_type: TokenType::Ident,
                literal: "foo".to_string(),
            },
            value: "foo".to_string(),
        };

        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            value: Expression::Ident(dummy_value),
        };

        self.next_token();

        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Return(stmt))
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn peek_error(&mut self, token_type: &TokenType) {
        let error_msg = format!(
            "expected next token to be \"{}\", got \"{}\" instead",
            token_type.get_literal(),
            self.peek_token.token_type.get_literal()
        );
        self.errors.push(error_msg);
    }
}

#[cfg(test)]
mod tests {
    use ast::NodeTrait;

    use super::*;

    fn test_let_statement(statement: &ast::Statement, name: &str) -> bool {
        let ast::Statement::Let(statement) = statement else {
            panic!("Not a Let statement");
        };

        if statement.name.value != name {
            panic!("statement.name.value not {}", name);
        }

        if statement.name.token_literal() != name {
            panic!("statement.name.token_literal() not {}", name);
        }

        true
    }

    fn check_parser_errors(parser: &Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }

        eprintln!("Parser has {} errors", errors.len());
        for error in errors.iter() {
            eprintln!("Parser error: {}", error)
        }
        panic!()
    }

    #[test]
    fn test_let_statements() {
        let input = r#"
          let x = 5;
          let y = 10;
          let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);
        assert!(test_let_statement(&program.statements[0], "x"));
        assert!(test_let_statement(&program.statements[1], "y"));
        assert!(test_let_statement(&program.statements[2], "foobar"));
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
          return 5;
          return 10;
          return 993322;
        "#;

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. Got {}",
                program.statements.len()
            );
        }

        for stmt in program.statements.iter() {
            use ast::Statement::*;
            match stmt {
                Return(s) => {
                    if s.token_literal() != "return" {
                        panic!(
                            "ReturnStatement literal is not \"return\", got \"{}\"",
                            s.token_literal()
                        )
                    }
                }
                _ => panic!(
                    "stmt is not a ReturnStatement. Got {}",
                    stmt.token_literal()
                ),
            }
        }
    }
}
