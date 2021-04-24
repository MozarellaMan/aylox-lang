use std::mem;

use crate::{
    ast::*,
    error::ParserError,
    token::{Token, TokenType},
};

type ParseExprResult = Result<Expr, ParserError>;
type ParseStmtResult = Result<Stmt, ParserError>;
type ParseStmtsResult = Result<Vec<Stmt>, ParserError>;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseStmtsResult {
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

    fn declaration(&mut self) -> ParseStmtResult {
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

    fn statement(&mut self) -> ParseStmtResult {
        if self.token_match(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.token_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.token_match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::new_block(Block::new(self.block_statement()?)));
        }
        if self.token_match(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.token_match(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.token_match(&[TokenType::Fun]) {
            return self.function(FunctionKind::Function);
        }
        if self.token_match(&[TokenType::Return]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> ParseStmtResult {
        let keyword = self.previous().clone();
        let val = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expected ';' after return value.")?;
        Ok(Stmt::Return_(Return_::new(keyword, val)))
    }

    fn function(&mut self, kind: FunctionKind) -> ParseStmtResult {
        let name = self
            .consume(
                &TokenType::Identifier(String::new()),
                &format!("Expected {} name", kind),
            )?
            .clone();
        self.consume(
            &TokenType::LeftParen,
            &format!("Expected '(' after {} name", kind),
        )?;
        let mut parameters: Vec<Token> = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let err = ParserError::FunctionParameterLength {
                        line: self.previous().line,
                    };
                    println!("{}", err);
                }
                parameters.push(
                    self.consume(
                        &TokenType::Identifier(String::new()),
                        "Expected parameter name",
                    )?
                    .clone(),
                );
                if !self.token_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(
            &TokenType::LeftBrace,
            &format!("Expected '{{' before {} body", kind),
        )?;
        let body = self.block_statement()?;
        Ok(Stmt::Function(Function::new(name, parameters, body)))
    }

    fn for_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'for'")?;
        let initializer = if self.token_match(&[TokenType::Semicolon]) {
            None
        } else if self.token_match(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&TokenType::RightParen, "Expected ')' after 'for' clauses")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            let increment_expression = Stmt::Expression(Expression::new(increment));
            body = Stmt::Block(Block::new(vec![body, increment_expression]));
        }

        let condition = if let Some(condition) = condition {
            condition
        } else {
            Expr::Literal(Literal::new(Value::Bool(true)))
        };

        body = Stmt::While_(While_::new(condition, Box::new(body)));

        if let Some(initializer) = initializer {
            body = Stmt::Block(Block::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(
            &TokenType::RightParen,
            "Expected ')' after 'while' condition",
        )?;
        let body = self.statement()?;

        Ok(Stmt::While_(While_::new(condition, Box::new(body))))
    }

    fn if_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after 'if' condition")?;
        let then_branch = self.statement()?;
        let else_branch = if self.token_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::new_if_(If_::new(
            condition,
            Box::new(then_branch),
            else_branch,
        )))
    }

    fn block_statement(&mut self) -> ParseStmtsResult {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after block.")?;
        Ok(statements)
    }

    fn var_declaration(&mut self) -> ParseStmtResult {
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

    fn print_statement(&mut self) -> ParseStmtResult {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn expression_statement(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::Expression(Expression::new(expr)))
    }

    fn expression(&mut self) -> ParseExprResult {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseExprResult {
        let expr = self.or()?;

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

    fn or(&mut self) -> ParseExprResult {
        let mut expr = self.and()?;

        while self.token_match(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::new_logical(Logical::new(Box::new(expr), operator, Box::new(right)))
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseExprResult {
        let mut expr = self.equality()?;

        while self.token_match(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::new_logical(Logical::new(Box::new(expr), operator, Box::new(right)))
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseExprResult {
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

    fn comparison(&mut self) -> ParseExprResult {
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

    fn term(&mut self) -> ParseExprResult {
        let mut expr = self.factor()?;

        while self.token_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseExprResult {
        let mut expr = self.unary()?;

        while self.token_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseExprResult {
        if self.token_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary::new(operator, Box::new(right))));
        }
        self.call()
    }

    fn call(&mut self) -> ParseExprResult {
        let mut expr = self.primary()?;

        loop {
            if !self.token_match(&[TokenType::LeftParen]) {
                break;
            }
            expr = self.finish_call(expr)?;
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParseExprResult {
        let mut arguments = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let err = ParserError::FunctionArgumentLength {
                        line: self.previous().line,
                    };
                    println!("{}", err);
                }
                arguments.push(self.expression()?);
                if !self.token_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Expr::Call(Call::new(
            Box::new(callee),
            paren.clone(),
            arguments,
        )))
    }

    fn primary(&mut self) -> ParseExprResult {
        if self.token_match(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::new(Value::Bool(false))));
        }
        if self.token_match(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::new(Value::Bool(true))));
        }
        if self.token_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::new(Value::Nil(Nil))));
        }

        if self.token_match(&[
            TokenType::Number(0f64),
            TokenType::String(String::new()),
            TokenType::Identifier(String::new()),
        ]) {
            match &self.previous()._type {
                TokenType::Number(num) => {
                    return Ok(Expr::Literal(Literal::new(Value::Number(*num))))
                }
                TokenType::String(string) => {
                    return Ok(Expr::Literal(Literal::new(Value::String(string.clone()))))
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
        Err(Parser::error(self.peek(), "Expected expression."))
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
