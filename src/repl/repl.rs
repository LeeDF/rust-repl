use std::io::Write;

use crate::{lexer, token::token};


const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let mut l = lexer::lexer::Lexer::new(input);
        loop {
            let tok = l.next_token();
            if tok.typ == token::TokenType::EOF {
                break;
            }

            println!("{:?}", tok);
        }
    }

}