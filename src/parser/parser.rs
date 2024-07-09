use std::collections::HashMap;
use std::future;

use crate::ast::ast::{
    BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
    IFExpression, Identifier, InfixExpression, IntegerLiteral, LetStatement, Node,
    PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::lexer::Lexer;
use crate::parser::consts;
use crate::{token::token::TokenType, token::Token};

use super::consts::PREFIX;

// pub type PrefixParseFn = fn() -> Box<dyn Expression>;
// pub type InfixParseFn = fn(exp: dyn Expression) -> Box<dyn Expression>;

pub struct Parser {
    lexer: Box<Lexer>,
    cur_token: Token,
    peek_token: Token,
    erros: Vec<String>,
    prefix_parses: HashMap<TokenType, fn(&mut Parser) -> Box<dyn Expression>>,
    infix_parses:
        HashMap<TokenType, fn(&mut Parser, exp: Box<dyn Expression>) -> Box<dyn Expression>>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let tk1 = lexer.next_token();
        let tk2 = lexer.next_token();
        let mut ans = Parser {
            lexer: Box::new(lexer),
            cur_token: tk1,
            peek_token: tk2,
            erros: Vec::new(),
            prefix_parses: HashMap::new(),
            infix_parses: HashMap::new(),
        };
        ans.register_prefix(TokenType::IDENT, Parser::parse_indentifier);
        ans.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        ans.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        ans.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        ans.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        ans.register_prefix(TokenType::FALSE, Parser::parse_boolean);
        ans.register_prefix(TokenType::LPAREN, Parser::parse_grouped_expression);
        ans.register_prefix(TokenType::IF, Parser::parse_if_expression);
        ans.register_prefix(TokenType::FUNCTION, Parser::parse_function_literal);

        ans.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        ans.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        ans.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        ans.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        ans.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        ans.register_infix(TokenType::NOT_EQ, Parser::parse_infix_expression);
        ans.register_infix(TokenType::LT, Parser::parse_infix_expression);
        ans.register_infix(TokenType::GT, Parser::parse_infix_expression);
        ans.register_infix(TokenType::LPAREN, Parser::parse_call_expression);
        ans
    }


    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        while self.cur_token.typ != TokenType::EOF {
            let stmt = self.parse_statement();
            if let Ok(stmt) = stmt {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        Ok((program))
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }


    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        match self.cur_token.typ {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            return Err("expect_peek failed".to_string());
        }

        let name = Identifier::new(self.cur_token.clone(), self.cur_token.literal.clone());
        if !self.expect_peek(TokenType::ASSIGN) {
            return Err("expect_peek failed".to_string());
        }
        
        self.next_token();
        let val = self.parse_expression(consts::LOWEST);
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(LetStatement::new(token, name, val)))
    }

    fn parse_return_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone();
        self.next_token();
        let val = self.parse_expression(consts::LOWEST);
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Ok(Box::new(ReturnStatement::new(token, val)))
    }

    fn parse_expression_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = self.cur_token.clone();
        // println!("parse_expression_statement {:?}", token);
        let expression = self.parse_expression(consts::LOWEST);
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        let exp = ExpressionStatement::new(token, expression);
        Ok(Box::new(exp))
    }

    fn tmp_value(&self) -> Box<dyn Expression> {
        let expr = Identifier::new(Token::new(TokenType::IDENT, "".to_string()), "".to_string());
        Box::new(expr)
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.typ == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.typ == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn register_prefix(&mut self, typ: TokenType, f: fn(&mut Parser) -> Box<dyn Expression>) {
        self.prefix_parses.insert(typ, f);
    }

    fn register_infix(
        &mut self,
        typ: TokenType,
        f: fn(&mut Parser, Box<dyn Expression>) -> Box<dyn Expression>,
    ) {
        self.infix_parses.insert(typ, f);
    }

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        let err = format!("no prefix parse fnction for {:?}", t);
        self.erros.push(err)
    }

    fn parse_expression(&mut self, precedence: i8) -> Box<dyn Expression> {
        if let Some(prefix) = self.prefix_parses.get(&self.cur_token.typ) {
            // println!("parse_expression prefix cur_token {:?}, peek_token {:?}", self.cur_token, self.peek_token);
            let mut left = prefix(self);
            while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
                // println!("parse_expression infix-get  cur_token {:?}, peek_token {:?}", self.cur_token, self.peek_token);

                if let Some(infix) = self.infix_parses.get(&self.peek_token.typ) {
                // println!("parse_expression infix cur_token {:?}, peek_token {:?}", self.cur_token, self.peek_token);

                    let infixc = infix.clone();
                    self.next_token();
                    left = infixc(self, left);
                } else {
                    return left;
                }
            }
            left
        } else {
            self.no_prefix_parse_fn_error(self.cur_token.typ);
            self.tmp_value()
        }
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(PREFIX);
        Box::new(PrefixExpression::new(token, operator, right))
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let prececedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(prececedence);
        Box::new(InfixExpression::new(token, left, operator, right))
    }

    fn parse_boolean(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let value = self.cur_token_is(TokenType::TRUE);
        Box::new(Boolean::new(token, value))
    }

    fn parse_grouped_expression(&mut self) -> Box<dyn Expression> {
        self.next_token();
        let exp = self.parse_expression(consts::LOWEST);
        if self.expect_peek(TokenType::RPAREN) {
            exp
        } else {
            self.tmp_value()
        }
    }

    fn parse_if_expression(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return self.tmp_value();
        }
        self.next_token();
        let condition = self.parse_expression(consts::LOWEST);
        if !self.expect_peek(TokenType::RPAREN) {
            return self.tmp_value();
        }
        if !self.expect_peek(TokenType::LBRACE) {
            return self.tmp_value();
        }
        let consequence = self.parse_block_statement();
        let mut alternative = BlockStatement::new(token.clone(), Vec::new());
        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(TokenType::LBRACE) {
                return self.tmp_value();
            }
            alternative = self.parse_block_statement();
        }
        Box::new(IFExpression::new(
            token,
            condition,
            consequence,
            alternative,
        ))
    }

    fn parse_function_literal(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return self.tmp_value();
        }
        let parmas = self.parse_function_params();
        if !self.expect_peek(TokenType::LBRACE) {
            self.peek_error(TokenType::LBRACE);
            return self.tmp_value();
        }
        let body = self.parse_block_statement();
        // println!("parse_function_literal {:?}, {:?}", parmas, body.to_string());
        Box::new(FunctionLiteral::new(token, parmas, body))
    }

    fn parse_function_params(&mut self) -> Vec<Identifier> {
        let mut ans = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return ans;
        }
        self.next_token();
        let token = self.cur_token.clone();
        let val = self.cur_token.literal.clone();
        ans.push(Identifier::new(token, val));
        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let token = self.cur_token.clone();
            let val = self.cur_token.literal.clone();
            ans.push(Identifier::new(token, val));
        }
        // println!("parse_function_params {:?}", ans);
        if !self.expect_peek(TokenType::RPAREN) {
            self.peek_error(TokenType::RPAREN);
            return Vec::new();
        }
        ans
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        let mut states = Vec::new();
        self.next_token();
        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            let st = self.parse_statement();
            if let Ok(st) = st {
                states.push(st);
            }
            self.next_token();
        }
        BlockStatement::new(token, states)
    }

    fn parse_call_expression(&mut self, func: Box<dyn Expression>) -> Box<dyn Expression> {
        println!("parse_call_expression");
        let token = self.cur_token.clone();
        let args = self.parse_call_args();

        Box::new(CallExpression::new(token, func, args))
    }

    fn parse_call_args(&mut self) -> Vec<Box<dyn Expression>> {
        let mut ans = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return ans;
        }
        self.next_token();
        let exp = self.parse_expression(consts::LOWEST);
        // println!("parse_call_args {:?}", exp.to_string());
        ans.push(exp);
        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let exp = self.parse_expression(consts::LOWEST);
            println!("parse_call_args {:?}", exp.to_string());

            ans.push(exp);
        }
        if !self.expect_peek(TokenType::RPAREN) {
            return ans;
        }
        ans
    }

    fn parse_indentifier(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        let value = self.cur_token.literal.clone();
        // println!("parse_indentifier {:?}, {:?}", token, value);
        Box::new(Identifier::new(token, value))
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        let token = self.cur_token.clone();
        match token.literal.parse::<i64>() {
            Ok(val) => Box::new(IntegerLiteral::new(token, val)),
            Err(e) => self.tmp_value(),
        }
    }

    fn peek_precedence(&self) -> i8 {
        consts::get_precedence(self.peek_token.typ)
    }
    fn cur_precedence(&self) -> i8 {
        consts::get_precedence(self.cur_token.typ)
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.typ
        );
        self.erros.push(msg);
    }

    fn check_errors(&self) -> bool {
        self.erros.len() == 0
    }

    fn print_errors(&self) {
        for e in self.erros.iter() {
            println!("errors: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::iter;
    use std::vec;

    use crate::ast::ast::ExpressionStatement;
    use crate::ast::ast::LetStatement;
    use crate::ast::ast::Node;
    use crate::lexer;

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        "#;
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let prgm = p.parse_program().unwrap();
        assert_eq!(prgm.get_statements_len(), 1);
        println!("{:#?} {}", prgm, prgm.to_string());
        let tests = vec!["x"];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = prgm.get_statement(i);
            assert_eq!(stmt.token_literal(), String::from("let"));
            if let Some(let_stmt) = stmt.downcast_ref::<LetStatement>() {
                println!("let_stmt val: {}", let_stmt.value.to_string());
                assert_eq!(let_stmt.name.value, tt.to_string());
                assert_eq!(let_stmt.name.token_literal(), tt.to_string());
            } else {
                panic!("stmt not LetStatement");
            }
        }
    }

    #[test]
    fn test_return_statment() {
        let input = r#"
        return 5;
        return 10;
        return 993 322;
        "#;
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let prgm = p.parse_program().unwrap();
        assert_eq!(prgm.get_statements_len(), 3);
        println!("{:#?}", prgm);
        for i in 0..3 {
            let stmt = prgm.get_statement(i);
            assert_eq!(stmt.token_literal(), String::from("return"));
        }
    }

    #[test]
    fn test_indentifierExpression() {
        let input = "foobar;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let prgm = p.parse_program().unwrap();
        assert_eq!(prgm.get_statements_len(), 1);
        let stmt: &Box<dyn Statement> = prgm.get_statement(0);
        if let Some(exp_stmt) = stmt.downcast_ref::<ExpressionStatement>() {
            if let Some(ident) = exp_stmt.expression.downcast_ref::<Identifier>() {
                // println!("{:?}", ident);
                assert_eq!(ident.value, String::from("foobar"));
                assert_eq!(ident.token_literal(), String::from("foobar"));
            } else {
                panic!("ident not Identifier")
            }
        } else {
            panic!("stmt not ExpressionStatement");
        }
    }

    fn get_integer(expression: Box<dyn Expression>) -> i64 {
        if let Some(exp_stmt) = expression.downcast_ref::<IntegerLiteral>() {
            exp_stmt.value
        } else {
            0
        }
    }

    struct prefixTest {
        input: String,
        operator: String,
        intervalue: i64,
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let mut tests = Vec::new();
        tests.push(prefixTest {
            input: "!5;".to_string(),
            operator: "!".to_string(),
            intervalue: 5,
        });
        tests.push(prefixTest {
            input: "-15;".to_string(),
            operator: "-".to_string(),
            intervalue: 15,
        });
        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);
            if let Some(exp_stmt) = stmt.downcast_ref::<ExpressionStatement>() {
                if let Some(ident) = exp_stmt.expression.downcast_ref::<PrefixExpression>() {
                    // println!("{:?}", ident);
                    assert_eq!(ident.operator, tt.operator);
                    if let Some(exp_stmt) = ident.right.downcast_ref::<IntegerLiteral>() {
                        println!("value {:?}", exp_stmt.value);
                        assert_eq!(exp_stmt.value, tt.intervalue)
                    } else {
                        panic!("value not IntegerLiteral")
                    }
                } else {
                    panic!("ident not PrefixExpression")
                }
            } else {
                panic!("stmt not ExpressionStatement");
            }
        }
    }

    struct infixTest {
        input: String,
        leftvalue: i64,
        operator: String,
        rightvalue: i64,
    }

    #[test]
    fn test_parse_infix_expressions() {
        let mut tests = Vec::new();
        tests.push(infixTest {
            input: "5 + 5;".to_string(),
            leftvalue: 5,
            operator: "+".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 - 5;".to_string(),
            leftvalue: 5,
            operator: "-".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 * 5;".to_string(),
            leftvalue: 5,
            operator: "*".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 / 5;".to_string(),
            leftvalue: 5,
            operator: "/".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 > 5;".to_string(),
            leftvalue: 5,
            operator: ">".to_string(),
            rightvalue: 5,
        });
        tests.push(infixTest {
            input: "5 < 5;".to_string(),
            leftvalue: 5,
            operator: "<".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 == 5;".to_string(),
            leftvalue: 5,
            operator: "==".to_string(),
            rightvalue: 5,
        });

        tests.push(infixTest {
            input: "5 != 5;".to_string(),
            leftvalue: 5,
            operator: "!=".to_string(),
            rightvalue: 5,
        });

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);
            if let Some(exp_stmt) = stmt.downcast_ref::<ExpressionStatement>() {
                if let Some(ident) = exp_stmt.expression.downcast_ref::<InfixExpression>() {
                    // println!("{:?}", ident);
                    assert_eq!(ident.operator, tt.operator);
                    if let Some(leftv) = ident.left.downcast_ref::<IntegerLiteral>() {
                        if let Some(rightv) = ident.right.downcast_ref::<IntegerLiteral>() {
                            // println!("left : {:?} right: {:?}", leftv.value, rightv.value);
                            assert_eq!(leftv.value, tt.leftvalue);
                            assert_eq!(rightv.value, tt.rightvalue);
                        } else {
                            panic!("value not IntegerLiteral")
                        }
                    } else {
                        panic!("value not IntegerLiteral")
                    }
                } else {
                    panic!("ident not PrefixExpression")
                }
            } else {
                panic!("stmt not ExpressionStatement");
            }
        }
    }

    struct OperatorPrecedence {
        input: String,
        expect: String,
    }

    #[test]
    fn test_parse_operator_precedence() {
        let mut tests = Vec::new();
        tests.push(OperatorPrecedence {
            input: "1 + (2 + 3) + 4".to_string(),
            expect: "((1 + (2 + 3)) + 4)".to_string(),
        });

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);

            assert_eq!(stmt.to_string(), tt.expect);
        }
    }

    #[test]
    fn test_parse_if() {
        let mut tests = Vec::new();
        tests.push(OperatorPrecedence {
            input: "if (x < y) { x } else { y }".to_string(),
            expect: "if(x < y) x else y".to_string(),
        });

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);

            assert_eq!(stmt.to_string(), tt.expect);
        }
    }

    #[test]
    fn test_parse_function() {
        let mut tests = Vec::new();
        tests.push(OperatorPrecedence {
            input: "fn(x, y) {x + y; }".to_string(),
            expect: "fn(x,y)(x + y)".to_string(),
        });

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);

            assert_eq!(stmt.to_string(), tt.expect);
        }
    }

    #[test]
    fn test_parse_call() {
        let mut tests = Vec::new();
        tests.push(OperatorPrecedence {
            input: "add( 1, 2 * 3, 4 + 5);".to_string(),
            expect: "add(1,(2 * 3),(4 + 5))".to_string(),
        });

        for tt in tests {
            let l = Lexer::new(tt.input);
            let mut parser = Parser::new(l);
            let prgm = parser.parse_program().unwrap();
            if parser.check_errors() {
                parser.print_errors();
            }
            println!("prgm {:?}", prgm);

            assert_eq!(prgm.get_statements_len(), 1);
            let stmt: &Box<dyn Statement> = prgm.get_statement(0);

            assert_eq!(stmt.to_string(), tt.expect);
        }
    }
}
