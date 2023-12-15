#[derive(PartialEq, Eq, Debug)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}

impl From<&str> for TokenType {
    fn from(s: &str) -> Self {
        match s {
            "EOF" => TokenType::EOF,
            "IDENT" => TokenType::IDENT,
            "INT" => TokenType::INT,
            "=" => TokenType::ASSIGN,
            "+" => TokenType::PLUS,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "FUNCTION" => TokenType::FUNCTION,
            "LET" => TokenType::LET,
            _ => TokenType::ILLEGAL,
        }
    }
}

pub struct Token {
    pub(crate) typ: TokenType,
    literal: String,
}

impl Token {
    pub fn new(typ: TokenType, literal: String) -> Token {
        Token {
            typ,
            literal,
        }
    }
}







