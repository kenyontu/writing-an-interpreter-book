use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
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
        let token: Token = match self.ch {
            Some('=') => Token::new(TokenType::Assign, "=".to_string()),
            Some(';') => Token::new(TokenType::Semicolon, ";".to_string()),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string()),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string()),
            Some(',') => Token::new(TokenType::Comma, ",".to_string()),
            Some('+') => Token::new(TokenType::Plus, "+".to_string()),
            Some('{') => Token::new(TokenType::LeftBrace, "{".to_string()),
            Some('}') => Token::new(TokenType::RightBrace, "}".to_string()),
            _ => Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let expected_values = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LeftParen, "("),
            (TokenType::RightParen, ")"),
            (TokenType::LeftBrace, "{"),
            (TokenType::RightBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for expected in expected_values.iter() {
            let token = lexer.next_token();

            assert_eq!(token.token_type, expected.0);
            assert_eq!(token.literal, expected.1);
        }
    }
}
