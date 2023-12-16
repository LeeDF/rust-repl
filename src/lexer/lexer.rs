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
        self.skip_whitespace();
        let tk = match self.ch {
            b'=' => token::Token::new(token::TokenType::ASSIGN, u8to_String(self.ch)),
            b';' => token::Token::new(token::TokenType::SEMICOLON, u8to_String(self.ch)),
            b'(' => token::Token::new(token::TokenType::LPAREN, u8to_String(self.ch)),
            b')' => token::Token::new(token::TokenType::RPAREN, u8to_String(self.ch)),
            b',' => token::Token::new(token::TokenType::COMMA, u8to_String(self.ch)),
            b'+' => token::Token::new(token::TokenType::PLUS, u8to_String(self.ch)),
            b'{' => token::Token::new(token::TokenType::LBRACE, u8to_String(self.ch)),
            b'}' => token::Token::new(token::TokenType::RBRACE, u8to_String(self.ch)),
            0 => token::Token::new(token::TokenType::EOF, String::from("")),
            _ => {
                if is_letter(self.ch) {
                    let lit = self.read_identifier();
                    let ty = token::TokenType::lookup_ident(lit.as_str());
                    return token::Token::new(ty, lit);
                } else if is_digit(self.ch) {
                    return token::Token::new(token::TokenType::INT, self.read_number());
                } else {
                    token::Token::new(token::TokenType::ILLEGAL, u8to_String(self.ch))
                }
            }
        };
        self.read_char();
        tk
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        let sl = &self.input[pos..self.position];
        String::from(sl)
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        let sl = &self.input[pos..self.position];
        String::from(sl)
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

fn is_letter(c: u8) -> bool {
    (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z') || (c == b'_')
}

fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

fn u8to_String(c: u8) -> String {
    match std::char::from_u32(c as u32) {
        Some(c) => c.to_string(),
        None => panic!("invalid unicode"),
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
        {
            let mut l = Lexer::new(input);
            for tt in tests {
                let tok = l.next_token();
                assert_eq!(tok.typ, tt);
            }
        }
        let s = String::from("abcdefg");
        let sl1 = &s[1..2];
        println!("{:?}", String::from(sl1));
        // let sl2 = &s.as_bytes()[1..2];
        // println!("{:?}", String::from_utf8(sl2).unwrap());
    }

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"let five = 5;
        let ten = 10;
        let add = fn(x,y) {
          x + y ;
        };
        let result = add(five, ten);
        "#,
        );
        let tests = vec![
            token::Token::new(token::TokenType::LET, String::from("let")),
            token::Token::new(token::TokenType::IDENT, String::from("five")),
            token::Token::new(token::TokenType::ASSIGN, String::from("=")),
            token::Token::new(token::TokenType::INT, String::from("5")),
            token::Token::new(token::TokenType::SEMICOLON, String::from(";")),
            token::Token::new(token::TokenType::LET, String::from("let")),
            token::Token::new(token::TokenType::IDENT, String::from("ten")),
            token::Token::new(token::TokenType::ASSIGN, String::from("=")),
            token::Token::new(token::TokenType::INT, String::from("10")),
            token::Token::new(token::TokenType::SEMICOLON, String::from(";")),
            token::Token::new(token::TokenType::LET, String::from("let")),
            token::Token::new(token::TokenType::IDENT, String::from("add")),
            token::Token::new(token::TokenType::ASSIGN, String::from("=")),
            token::Token::new(token::TokenType::FUNCTION, String::from("fn")),
            token::Token::new(token::TokenType::LPAREN, String::from("(")),
            token::Token::new(token::TokenType::IDENT, String::from("x")),
            token::Token::new(token::TokenType::COMMA, String::from(",")),
            token::Token::new(token::TokenType::IDENT, String::from("y")),
            token::Token::new(token::TokenType::RPAREN, String::from(")")),
            token::Token::new(token::TokenType::LBRACE, String::from("{")),
            token::Token::new(token::TokenType::IDENT, String::from("x")),
            token::Token::new(token::TokenType::PLUS, String::from("+")),
            token::Token::new(token::TokenType::IDENT, String::from("y")),
            token::Token::new(token::TokenType::SEMICOLON, String::from(";")),
            token::Token::new(token::TokenType::RBRACE, String::from("}")),
            token::Token::new(token::TokenType::SEMICOLON, String::from(";")),
            token::Token::new(token::TokenType::LET, String::from("let")),
            token::Token::new(token::TokenType::IDENT, String::from("result")),
            token::Token::new(token::TokenType::ASSIGN, String::from("=")),
            token::Token::new(token::TokenType::IDENT, String::from("add")),
            token::Token::new(token::TokenType::LPAREN, String::from("(")),
            token::Token::new(token::TokenType::IDENT, String::from("five")),
            token::Token::new(token::TokenType::COMMA, String::from(",")),
            token::Token::new(token::TokenType::IDENT, String::from("ten")),
            token::Token::new(token::TokenType::RPAREN, String::from(")")),
            token::Token::new(token::TokenType::SEMICOLON, String::from(";")),
            token::Token::new(token::TokenType::EOF, String::from("")),
        ];
        let mut l = Lexer::new(input);
        for tt in tests {
            let tok = l.next_token();
            // println!("{:?}", tok);
            assert_eq!(tok, tt);
        }
    }

    #[test]
    fn test_u8to_String() {
        let r = u8to_String(b'=');
        let l = String::from("=");
        assert_eq!(l, r);
    }
}
