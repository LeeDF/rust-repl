use std::io::Write;

use crate::{ast::ast::Node, lexer, parser::parser, token::token};


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
        let mut p = parser::Parser::new(l);
        let pgm = p.parse_program().unwrap();
        println!("parsed {:?} {}", pgm, pgm.get_statements_len());
        std::io::stdout().write(pgm.to_string().as_bytes());
        std::io::stdout().write(b"\n");
        std::io::stdout().flush();
    }

}