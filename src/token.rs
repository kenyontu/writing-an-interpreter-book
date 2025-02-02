use crate::parser::Precedence;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    LessThan,
    GreaterThan,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,
}

impl TokenType {
    pub fn get_literal(&self) -> &str {
        match self {
            TokenType::Int => "int",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Comma => ",",
            TokenType::LessThan => "<",
            TokenType::GreaterThan => ">",
            TokenType::Semicolon => ";",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::Function => "function",
            TokenType::Let => "let",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::Equal => "==",
            TokenType::NotEqual => "!=",
            _ => "",
        }
    }

    pub fn precedence(&self) -> Precedence {
        use TokenType::*;
        match self {
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            LessThan | GreaterThan => Precedence::LessGreater,
            Equal | NotEqual => Precedence::Equals,
            _ => Precedence::Lowest,
        }
    }

    pub fn is_infix(&self) -> bool {
        use TokenType::*;
        matches!(
            self,
            Plus | Minus | Asterisk | Slash | LessThan | GreaterThan | Equal | NotEqual
        )
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}
