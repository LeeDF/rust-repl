use crate::token::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    //始终指向下一个字符
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    //读取下一个字符
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn next_token(&mut self) -> token::Token {
        let tk = match self.ch {
            b'=' => token::Token::new(token::TokenType::ASSIGN, self.ch.to_string()),
            b';' => token::Token::new(token::TokenType::SEMICOLON, self.ch.to_string()),
            b'(' => token::Token::new(token::TokenType::LPAREN, self.ch.to_string()),
            b')' => token::Token::new(token::TokenType::RPAREN, self.ch.to_string()),
            b',' => token::Token::new(token::TokenType::COMMA, self.ch.to_string()),
            b'+' => token::Token::new(token::TokenType::PLUS, self.ch.to_string()),
            b'{' => token::Token::new(token::TokenType::LBRACE, self.ch.to_string()),
            b'}' => token::Token::new(token::TokenType::RBRACE, self.ch.to_string()),
            _ => token::Token::new(token::TokenType::EOF, String::from("")),
        };
        self.read_char();
        tk
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let input = String::from("=+(){},;");
        let tests = vec![
            token::TokenType::ASSIGN,
            token::TokenType::PLUS,
            token::TokenType::LPAREN,
            token::TokenType::RPAREN,
            token::TokenType::LBRACE,
            token::TokenType::RBRACE,
            token::TokenType::COMMA,
            token::TokenType::SEMICOLON,
            token::TokenType::EOF,
        ];
        let mut l = Lexer::new(input);
        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok.typ, tt);
        }
    }
}