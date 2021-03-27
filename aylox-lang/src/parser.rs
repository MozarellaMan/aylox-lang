use std::mem;

use crate::{
    ast::*,
    error::ParserError,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("no previous token found")
    }

    fn is_at_end(&self) -> bool {
        self.peek()._type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("no token found")
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.token_match(&[TokenType::Var]) {
            if let Ok(stmt) = self.var_declaration() {
                return Ok(stmt);
            } else {
                self.synchronize()
            }
        }

        if let Ok(stmt) = self.statement() {
            Ok(stmt)
        } else {
            self.synchronize();
            Err(ParserError::Generic)
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.token_match(&[TokenType::Print]) {
            return self.print_statement();
        } else if self.token_match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::new_block(Block::new(self.block_statement()?)));
        }

        self.expression_statement()
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after block.")?;
        Ok(statements)
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self
            .consume(
                &TokenType::Identifier(String::new()),
                "Expected variable name",
            )?
            .clone();
        let mut initializer: Option<Expr> = None;

        if self.token_match(&[TokenType::Equal]) {
            initializer = Some(self.expression()?)
        }

        self.consume(
            &TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;
        Ok(Stmt::new_var(Var::new(name, initializer)))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression(Expression::new(expr)))
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;

        if self.token_match(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            if let Expr::Variable(var) = expr {
                let name = var.name;
                return Ok(Expr::Assign(Assign::new(name, Box::new(value))));
            }
            let err = ParserError::UnexpectedToken {
                lexeme: equals.lexeme,
                msg: "Invalid assignment target".to_string(),
                line: equals.line,
            };

            println!("{}", err);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.token_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn token_match(&mut self, types: &[TokenType]) -> bool {
        for _type in types.iter() {
            if self.check(_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, _type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        mem::discriminant(&self.peek()._type) == mem::discriminant(_type)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.token_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(operator, Box::new(right))));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.token_match(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::new(LiteralVal::Bool(false))));
        }
        if self.token_match(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::new(LiteralVal::Bool(true))));
        }
        if self.token_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::new(LiteralVal::Nil(Nil))));
        }

        if self.token_match(&[
            TokenType::Number(0f64),
            TokenType::String(String::new()),
            TokenType::Identifier(String::new()),
        ]) {
            match &self.previous()._type {
                TokenType::Number(num) => {
                    return Ok(Expr::Literal(Literal::new(LiteralVal::Number(*num))))
                }
                TokenType::String(string) => {
                    return Ok(Expr::Literal(Literal::new(LiteralVal::String(
                        string.clone(),
                    ))))
                }
                TokenType::Identifier(_) => {
                    return Ok(Expr::Variable(Variable::new(self.previous().clone())))
                }
                _ => {}
            }
        }

        if self.token_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Grouping::new(Box::new(expr))));
        }
        Err(Parser::error(self.peek(), "Expected expression"))
    }

    fn consume(&mut self, _type: &TokenType, msg: &str) -> Result<&Token, ParserError> {
        if self.check(_type) {
            return Ok(self.advance());
        }
        let error = Parser::error(self.peek(), msg);
        println!("{}", error);
        Err(error)
    }

    fn error(token: &Token, msg: &str) -> ParserError {
        match token._type {
            TokenType::Eof => ParserError::UnexpectedEof {
                line: token.line,
                msg: msg.to_string(),
            },
            _ => ParserError::UnexpectedToken {
                line: token.line,
                lexeme: token.lexeme.clone(),
                msg: msg.to_string(),
            },
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous()._type.is_semicolon() {
                return;
            }

            match self.peek()._type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return
                | TokenType::Print => {
                    return;
                }
                _ => {}
            }
        }

        self.advance();
    }
}
