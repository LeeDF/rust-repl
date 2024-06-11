#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    LT,
    GT,
    EQ,
    NOT_EQ,
    BANG,

    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn lookup_ident(s: &str) -> TokenType {
        match s {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub(crate) typ: TokenType,
    pub literal: String,
}

impl Token {
    pub(crate) fn new(typ: TokenType, literal: String) -> Token {
        Token { typ, literal }
    }
}
