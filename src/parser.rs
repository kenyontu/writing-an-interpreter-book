use std::{borrow::BorrowMut, mem};

use crate::{
    ast::{
        self,
        expressions::{IdentExpression, InfixExpression, IntegerLiteral, PrefixExpression},
        statements::{ExpressionStatement, LetStatement, ReturnStatement},
        Expression,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

/// Enum containing the operators in the language, so we can assign
/// them to a precedence level.
pub enum Precedence {
    /// The lowest level of precedence
    Lowest,
    /// For `==` operators
    Equals,
    /// For `>` or `<` operators
    LessGreater,
    /// For `+` operators
    Sum, // +
    /// For `*` operators
    Product,
    /// For `-x` or `!x` operators
    Prefix,
    /// For function calls like `my_function()`
    Call,
}

impl Precedence {
    /// Returns the precedence value of each variant
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
    lexer: Lexer<'a>,
    /// The current token being parsed
    cur_token: Token,
    /// The next token to parse
    peek_token: Token,
    /// The list of parsing errors
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

    /// Starts parsing the input
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

    /// Returns the list of parsing errors
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    /// Advance to the next token
    fn next_token(&mut self) {
        // Replaces the value of both `self.cur_token` and `self.peek_token`:
        // - `self.cur_token` receives the current value of `self.peek_token`
        // - `self.peek_token` receives the next token from the lexer
        self.cur_token = mem::replace(self.peek_token.borrow_mut(), self.lexer.next_token());
    }

    /// Checks if the current token is of a given type
    fn cur_token_is(&self, token_type: &TokenType) -> bool {
        &self.cur_token.token_type == token_type
    }

    fn cur_precedence(&self) -> Precedence {
        self.cur_token.token_type.precedence()
    }

    /// Checks if the peek token is of a given type
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
    }

    /// Writes a parse error when the next token isn't the one expected
    fn peek_error(&mut self, token_type: &TokenType) {
        let error_msg = format!(
            "expected next token to be \"{}\", got \"{}\" instead",
            token_type.get_literal(),
            self.peek_token.token_type.get_literal()
        );
        self.errors.push(error_msg);
    }

    fn peek_precedence(&self) -> Precedence {
        self.peek_token.token_type.precedence()
    }

    fn expect_peek(&mut self, token_type: &TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
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

    /// Parsers `self.cur_token` as a return statement.
    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        // TODO: The book left the value undefined, so I'm using dummy value until the
        // comes back to this to implement it
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

    fn parse_identifier(&self) -> Option<ast::Expression> {
        let ident = IdentExpression {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        Some(ast::Expression::Ident(ident))
    }

    /// Parsers `self.cur_token` as an integer literal.
    fn parse_integer_literal(&mut self) -> Option<ast::Expression> {
        let value = match self.cur_token.literal.parse::<i64>() {
            Ok(v) => v,
            Err(e) => {
                let msg = format!(
                    "Could not parse {} as integer: {}",
                    self.cur_token.literal, e
                );
                self.errors.push(msg);
                return None;
            }
        };

        let lit = IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        };

        Some(ast::Expression::Integer(lit))
    }

    fn parse_prefix_expression(&mut self) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        let operator = token.literal.clone();
        self.next_token();

        let right = self.parse_expression(Precedence::Prefix.value())?;
        let prefix = PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        };

        Some(ast::Expression::Prefix(prefix))
    }

    fn prefix_parse(&mut self) -> Option<ast::Expression> {
        match self.cur_token.token_type {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Minus => self.parse_prefix_expression(),
            TokenType::Bang => self.parse_prefix_expression(),
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Option<ast::Expression> {
        let token = self.cur_token.clone();
        let operator = token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence.value())?;
        let infix = InfixExpression {
            token,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        };

        Some(Expression::Infix(infix))
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

    fn parse_expression(&mut self, precedence: usize) -> Option<ast::Expression> {
        let mut left_expression = self.prefix_parse();

        while !self.peek_token_is(&TokenType::Semicolon)
            && precedence < self.peek_precedence().value()
        {
            if !self.peek_token.token_type.is_infix() {
                return left_expression;
            }

            self.next_token();

            left_expression = self.parse_infix_expression(left_expression?);
        }

        left_expression
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use ast::{NodeTrait, Statement};

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

    fn test_integer_literal(expression: &Expression, value: &i64) -> bool {
        let Expression::Integer(int) = expression else {
            eprintln!(
                "Expression isn't an Integer, got {}",
                expression.to_string()
            );
            return false;
        };

        if &int.value != value {
            eprintln!("Integer value is not \"{}\", got \"{}\"", value, int.value);
            return false;
        }

        if int.token_literal() != value.to_string() {
            eprintln!(
                "Integer token literal isn't \"{}\", got \"{}\"",
                value,
                int.token_literal()
            );
            return false;
        }

        true
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

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

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

        let Expression::Integer(integer_literal) = &stmt.expression else {
            panic!("Expression isn't an Integer");
        };

        assert_eq!(integer_literal.value, 5);
        assert_eq!(integer_literal.token_literal(), "5");
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let tests: Vec<(&str, &str, i64)> = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for (input, operator, value) in tests.iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "The program should have 1 statement"
            );

            let Statement::Expression(stmt) = &program.statements[0] else {
                panic!("Statement isn't an expression");
            };

            let Expression::Prefix(prefix) = &stmt.expression else {
                panic!("Statement isn't a prefix expression");
            };

            assert_eq!(
                &prefix.operator, operator,
                "Operator is not \"{}\", got \"{}\"",
                operator, prefix.operator
            );

            assert!(test_integer_literal(prefix.right.as_ref(), value));
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let tests: Vec<(&str, i64, &str, i64)> = vec![
            ("5 + 5", 5, "+", 5),
            ("5 - 5", 5, "-", 5),
            ("5 * 5", 5, "*", 5),
            ("5 / 5", 5, "/", 5),
            ("5 > 5", 5, ">", 5),
            ("5 < 5", 5, "<", 5),
            ("5 == 5", 5, "==", 5),
            ("5 != 5", 5, "!=", 5),
        ];

        for (input, left_value, operator, right_value) in tests.iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            assert_eq!(
                program.statements.len(),
                1,
                "The program should have 1 statement"
            );

            let Statement::Expression(stmt) = &program.statements[0] else {
                panic!("Statement isn't an expression");
            };

            let Expression::Infix(infix) = &stmt.expression else {
                panic!("Statement isn't a prefix expression");
            };

            assert!(test_integer_literal(infix.left.as_ref(), left_value));

            assert_eq!(
                &infix.operator, operator,
                "Operator is not \"{}\", got \"{}\"",
                operator, infix.operator
            );

            assert!(test_integer_literal(infix.right.as_ref(), right_value));
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests: Vec<(&str, &str)> = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];

        for (input, expected) in tests.iter() {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parser_errors(&parser);

            assert_eq!(&program.to_string(), expected);
        }
    }
}
