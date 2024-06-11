use crate::ast::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement};
use crate::lexer::lexer::Lexer;
use crate::{token::token::TokenType, token::Token};

struct Parser {
    lexer: Box<Lexer>,
    cur_token: Token,
    peek_token: Token,
    erros: Vec<String>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let tk1 = lexer.next_token();
        let tk2 = lexer.next_token();
        Parser {
            lexer: Box::new(lexer),
            cur_token: tk1,
            peek_token: tk2,
            erros: Vec::new(),
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        while self.cur_token.typ != TokenType::EOF {
            let stmt = self.parse_statement();
            if let Ok(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Ok((program))
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        match self.cur_token.typ {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => Err("parse_statement failed".to_string()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return Err("expect_peek failed".to_string());
        }

        let name = Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());
        if !self.expect_peek(TokenType::ASSIGN) {
            return Err("expect_peek failed".to_string());
        }
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(LetStatement::new(token, name, self.tmp_value())))
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone();
        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(ReturnStatement::new(token, self.tmp_value())))
    }

    fn tmp_value(&self) -> Box<dyn Expression> {
        let expr = Identifier::new(Token::new(TokenType::IDENT, "".to_string()), "".to_string());
        Box::new(expr)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.typ == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.typ == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.typ
        );
        self.erros.push(msg);
    }

    fn check_errors(&self) -> bool {
        self.erros.len() == 0
    }

    fn print_errors(&self) {
        for e in self.erros.iter() {
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ast::ast::LetStatement;
    use crate::ast::ast::Node;

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let prgm = p.parse_program().unwrap();
        assert_eq!(prgm.get_statements_len(), 3);
        println!("{:#?}", prgm);
        let tests = vec!["x", "y", "foobar"];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = prgm.get_statement(i);
            assert_eq!(stmt.token_literal(), String::from("let"));
            if let Some(let_stmt) = stmt.downcast_ref::<LetStatement>() {
                assert_eq!(let_stmt.name.value, tt.to_string());
                assert_eq!(let_stmt.name.token_literal(), tt.to_string());
            } else {
                panic!("stmt not LetStatement");
            }
        }
    }

    #[test]
    fn test_return_statment() {
        let input = r#"
        return 5;
        return 10;
        return 993 322;
        "#;
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let prgm = p.parse_program().unwrap();
        assert_eq!(prgm.get_statements_len(), 3);
        println!("{:#?}", prgm);
        for i in 0..3 {
            let stmt = prgm.get_statement(i);
            assert_eq!(stmt.token_literal(), String::from("return"));
        }
    }
}
