use crate::token::token::TokenType;

pub const LOWEST: i8 = 1;
pub const EQUALS: i8 = 2; // ==
pub const LESSGREATER: i8 = 3; // > or <
pub const SUM: i8 = 4; //+
pub const PRODUCT: i8 = 5; // *
pub const PREFIX: i8 = 6; // -X or !X
pub const CALL: i8 = 7; //fn(x)

pub fn get_precedence(typ: TokenType) -> i8 {
    match typ {
        TokenType::EQ => EQUALS,
        TokenType::NOT_EQ => EQUALS,
        TokenType::LT => LESSGREATER,
        TokenType::GT => LESSGREATER,
        TokenType::PLUS => SUM,
        TokenType::MINUS => SUM,
        TokenType::SLASH => PRODUCT,
        TokenType::ASTERISK => PRODUCT,
        TokenType::LPAREN => CALL,
        _ => LOWEST,
    }
}
