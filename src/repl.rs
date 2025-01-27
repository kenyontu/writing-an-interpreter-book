use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenType};

pub fn start() {
    let mut input = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "\n" {
                    break;
                }

                println!("input: {input}");

                let mut lexer = Lexer::new(&input);
                let mut token = lexer.next_token();

                while token.token_type != TokenType::Eof {
                    println!("{:?}", token);
                    token = lexer.next_token();
                }

                println!();
            }
            Err(error) => println!("Error: {error}"),
        }
    }
}
