
use crate::ast::ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement};

pub type prefix_parse_fn = fn () ->Expression;
pub type infix_parse_fn = fn (exp: Expression) -> Expression;
