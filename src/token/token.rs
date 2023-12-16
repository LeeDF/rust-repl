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

impl TokenType {
    pub fn lookup_ident(s: &str) -> TokenType {
        match s {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            _ => TokenType::IDENT,
        }
    }
}



#[derive(PartialEq, Eq, Debug)]
pub struct Token {
    pub(crate) typ: TokenType,
    literal: String,
}

impl Token {
    pub(crate) fn new(typ: TokenType, literal: String) -> Token {
        Token { typ, literal }
    }
}
