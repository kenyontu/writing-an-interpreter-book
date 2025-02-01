use std::{borrow::BorrowMut, mem};

use crate::{
    ast::{
        self,
        expressions::IdentExpression,
        statements::{ExpressionStatement, LetStatement, ReturnStatement},
        Expression, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn value(&self) -> usize {
        match self {
            Precedence::Lowest => 1,
            Precedence::Equals => 2,
            Precedence::LessGreater => 3,
            Precedence::Sum => 4,
            Precedence::Product => 5,
            Precedence::Prefix => 6,
            Precedence::Call => 7,
        }
    }
}

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
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn cur_token_is(&self, token_type: &TokenType) -> bool {
        &self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
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

    fn prefix_parse(&self) -> Option<ast::Expression> {
        match self.cur_token.token_type {
            TokenType::Illegal => todo!(),
            TokenType::Eof => todo!(),
            TokenType::Ident => {
                let ident = IdentExpression {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                };

                Some(ast::Expression::Ident(ident))
            }
            TokenType::Int => todo!(),
            TokenType::Assign => todo!(),
            TokenType::Plus => todo!(),
            TokenType::Minus => todo!(),
            TokenType::Bang => todo!(),
            TokenType::Asterisk => todo!(),
            TokenType::Slash => todo!(),
            TokenType::Comma => todo!(),
            TokenType::LessThan => todo!(),
            TokenType::GreaterThan => todo!(),
            TokenType::Semicolon => todo!(),
            TokenType::LeftParen => todo!(),
            TokenType::RightParen => todo!(),
            TokenType::LeftBrace => todo!(),
            TokenType::RightBrace => todo!(),
            TokenType::Function => todo!(),
            TokenType::Let => todo!(),
            TokenType::True => todo!(),
            TokenType::False => todo!(),
            TokenType::If => todo!(),
            TokenType::Else => todo!(),
            TokenType::Return => todo!(),
            TokenType::Equal => todo!(),
            TokenType::NotEqual => todo!(),
        }
    }

    fn infix_parse(&self, expression: ast::Expression) -> Option<ast::Expression> {
        todo!();
    }

    fn parse_expression_statement(&mut self) -> Option<ast::Statement> {
        let expression = self.parse_expression(Precedence::Lowest.value())?;

        let stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            expression,
        };

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(ast::Statement::Expression(stmt))
    }

    fn parse_expression(&self, precedence: usize) -> Option<ast::Expression> {
        self.prefix_parse()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use ast::{statements::ExpressionStatement, NodeTrait, Statement};

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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "The program should contain 1 statement"
        );

        let Statement::Expression(stmt) = &program.statements[0] else {
            panic!("Statement isn't an expression");
        };

        let Expression::Ident(ident) = &stmt.expression else {
            panic!("Expression isn't an identifier");
        };

        assert_eq!(ident.token_literal(), "foobar");
    }
}
