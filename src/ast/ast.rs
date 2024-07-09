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

pub trait Expression: Node + Downcast {
    fn expression_node(&self);
}
impl_downcast!(Expression);

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
            // println!("pgm to string {:?}", v.to_string());
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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}
impl Expression for Boolean {
    fn expression_node(&self) {}
}
impl Boolean {
    pub fn new(token: Token, value: bool) -> Boolean {
        Boolean { token, value }
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
    pub fn new(token: Token, return_value: Box<dyn Expression>) -> ReturnStatement {
        ReturnStatement {
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

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> ExpressionStatement {
        ExpressionStatement { token, expression }
    }
}

pub struct IntegerLiteral {
    token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> IntegerLiteral {
        IntegerLiteral { token, value }
    }
}
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::from("(");
        ans.push_str(&self.operator);
        ans.push_str(&self.right.to_string());
        ans.push_str(")");
        ans
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, right: Box<dyn Expression>) -> PrefixExpression {
        PrefixExpression {
            token,
            operator,
            right,
        }
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::from("(");
        ans.push_str(&self.left.to_string());
        ans.push_str(" ");
        ans.push_str(&self.operator);
        ans.push_str(" ");

        ans.push_str(&self.right.to_string());
        ans.push_str(")");
        ans
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

impl InfixExpression {
    pub fn new(
        token: Token,
        left: Box<dyn Expression>,
        operator: String,
        right: Box<dyn Expression>,
    ) -> InfixExpression {
        InfixExpression {
            token,
            left,
            operator,
            right,
        }
    }
}

pub struct IFExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: BlockStatement,
}

impl Node for IFExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::from("if");
        ans.push_str(self.condition.to_string().as_str());
        ans.push_str(" ");
        ans.push_str(self.consequence.to_string().as_str());

        if self.alternative.statements.len() > 0 {
            ans.push_str(" ");

            ans.push_str("else ");

            ans.push_str(self.alternative.to_string().as_str());
        }
       
        ans
    }
}

impl Expression for IFExpression {
    fn expression_node(&self) {}
}

impl IFExpression {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        alternative: BlockStatement,
    ) -> IFExpression {
        IFExpression {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::new();
        for st in &self.statements {
            ans.push_str(st.to_string().as_str());
        }

        ans
    }
}

impl Expression for BlockStatement {
    fn expression_node(&self) {}
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> BlockStatement {
        BlockStatement { token, statements }
    }
}

pub struct FunctionLiteral {
    pub token: Token,
    pub params: Vec<Identifier>,
    pub body: BlockStatement
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::new();
        ans.push_str(self.token_literal().as_str());
        ans.push_str("(");
        for st in &self.params {
            ans.push_str(st.to_string().as_str());
            ans.push_str(",");
        }
        if ans.ends_with(",") {
            ans.pop();
        }
        ans.push_str(")");

        ans.push_str(self.body.to_string().as_str());
        ans
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

impl FunctionLiteral {
    pub fn new(token: Token, params: Vec<Identifier>, body: BlockStatement) -> FunctionLiteral {
        FunctionLiteral { token, params, body }
    }
}


pub struct CallExpression {
    pub token: Token,
    pub func: Box<dyn Expression>,
    pub args: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn to_string(&self) -> String {
        let mut ans = String::new();
        ans.push_str(self.func.to_string().as_str());
        ans.push_str("(");
        for st in &self.args {
            ans.push_str(st.to_string().as_str());
            ans.push_str(",");
        }
        if ans.ends_with(","){
            ans.pop();
        }
        ans.push_str(")");


        ans
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}

impl CallExpression {
    pub fn new(token: Token, func: Box<dyn Expression>, args: Vec<Box<dyn Expression>>) -> CallExpression {
        CallExpression { token, func, args }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token::TokenType;

    #[test]
    fn test_string() {
        let mut program = Program::new();
        let token = Token::new(TokenType::LET, String::from("let"));
        let name = Identifier::new(
            Token::new(TokenType::IDENT, String::from("myVar")),
            String::from("myVar"),
        );
        let value = Identifier::new(
            Token::new(TokenType::IDENT, String::from("anotherVar")),
            String::from("anotherVar"),
        );
        let letstmt = LetStatement::new(token, name, Box::new(value));
        program.statements.push(Box::new(letstmt));
    }
}
