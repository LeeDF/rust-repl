use core::fmt;

use crate::token::token::Token;
use downcast_rs::{impl_downcast, Downcast};
pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait Statement: Node + Downcast {
    fn statement_node(&self);
}

impl_downcast!(Statement);

pub trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        for v in self.statements.iter() {
            out.push_str(v.to_string().as_str());
        }
        out
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for stmt in self.statements.iter() {
            s.push_str(&stmt.token_literal());
        }
        write!(f, "{}", s)
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn get_statements_len(&self) -> usize {
        self.statements.len()
    }

    pub fn get_statement(&self, index: usize) -> &Box<dyn Statement> {
        &self.statements[index]
    }

    pub fn get_let_statement(&self, index: usize) -> Result<LetStatement, std::io::Error> {
        // self.statements[index].downcast_ref;
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not a LetStatement",
        ))
    }
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {}
}
impl Identifier {
    pub fn new(token: Token, value: String) -> Identifier {
        Identifier { token, value }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        out.push_str(self.name.to_string().as_str());
        out.push_str(" = ");
        out.push_str(self.value.to_string().as_str());
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
        LetStatement { token, name, value }
    }
}


pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        out.push_str(self.return_value.to_string().as_str());
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Box<dyn Expression>)->ReturnStatement{
        ReturnStatement{
            token,
            return_value,
        }
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}


#[cfg(test)]
mod tests{
    use crate::token::token::TokenType;
    use super::*;
    
    #[test]
    fn test_string() {
        let mut program = Program::new();
        let token = Token::new(TokenType::LET, String::from("let"));
        let name = Identifier::new(Token::new(TokenType::IDENT, String::from("myVar")), String::from("myVar"));
        let value = Identifier::new(Token::new(TokenType::IDENT, String::from("anotherVar")), String::from("anotherVar"));
        let letstmt = LetStatement::new(token, name, Box::new(value));
        program.statements.push(Box::new(letstmt));
       
    }
}