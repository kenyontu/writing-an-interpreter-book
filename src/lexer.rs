use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::token::{Token, TokenType};

static KEYWORDS: Lazy<HashMap<&str, TokenType>> = Lazy::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert("fn", TokenType::Function);
    keywords.insert("let", TokenType::Let);
    keywords.insert("true", TokenType::True);
    keywords.insert("false", TokenType::False);
    keywords.insert("if", TokenType::If);
    keywords.insert("else", TokenType::Else);
    keywords.insert("return", TokenType::Return);

    keywords
});

pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token = match self.ch {
            Some('=') => {
                if matches!(self.peek_char(), Some('=')) {
                    self.read_char();
                    Token::new(TokenType::Equal, "==".to_string())
                } else {
                    Token::new(TokenType::Assign, "=".to_string())
                }
            }
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some('-') => Token::new(TokenType::Minus, "-".to_string()),
            Some('!') => {
                if matches!(self.peek_char(), Some('=')) {
                    self.read_char();
                    Token::new(TokenType::NotEqual, "!=".to_string())
                } else {
                    Token::new(TokenType::Bang, "!".to_string())
                }
            }
            Some('/') => Token::new(TokenType::Slash, "/".to_string()),
            Some('*') => Token::new(TokenType::Asterisk, "*".to_string()),
            Some('<') => Token::new(TokenType::LessThan, "<".to_string()),
            Some('>') => Token::new(TokenType::GreaterThan, ">".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some(';') => Token::new(TokenType::Semicolon, ";".to_string()),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string()),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string()),
            Some('{') => Token::new(TokenType::LeftBrace, "{".to_string()),
            Some('}') => Token::new(TokenType::RightBrace, "}".to_string()),
            Some(ch) => {
                if Self::is_letter(&ch) {
                    let literal = self.read_identifier();
                    return Token::new(Self::lookup_ident(literal), literal.to_string());
                } else if Self::is_digit(&ch) {
                    let literal = self.read_number();
                    return Token::new(TokenType::Int, literal.to_string());
                } else {
                    Token::new(TokenType::Illegal, ch.to_string())
                }
            }
            _ => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if Self::is_letter(&ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position..self.position].as_ref()
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while let Some(ch) = self.ch {
            if Self::is_digit(&ch) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position..self.position].as_ref()
    }

    fn is_letter(ch: &char) -> bool {
        ch.is_alphabetic() || ch == &'_'
    }

    fn is_digit(ch: &char) -> bool {
        ch.is_ascii_digit()
    }

    fn lookup_ident(ident: &str) -> TokenType {
        match KEYWORDS.get(ident) {
            Some(token_type) => token_type.to_owned(),
            None => TokenType::Ident,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = r#"
          let five = 5;
          let ten = 10;
          
          let add = fn(x, y) {
            x + y;
          };

          let result = add(five, ten);
          !-/*5;
          5 < 10 > 5;

          if (5 < 10) {
            return true;
          } else {
            return false;
          }

          10 == 10;
          10 != 9;
        "#;

        let expected_values = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LeftParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RightParen, ")"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LeftParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RightParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LessThan, "<"),
            (TokenType::Int, "10"),
            (TokenType::GreaterThan, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LeftParen, "("),
            (TokenType::Int, "5"),
            (TokenType::LessThan, "<"),
            (TokenType::Int, "10"),
            (TokenType::RightParen, ")"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LeftBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RightBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Equal, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEqual, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(&input);

        for expected in expected_values.iter() {
            let token = lexer.next_token();

            assert_eq!(token.token_type, expected.0);
            assert_eq!(token.literal, expected.1);
        }
    }
}
