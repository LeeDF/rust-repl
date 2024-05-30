
use crate::{lexer::lexer::Lexer, token::Token};

struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

