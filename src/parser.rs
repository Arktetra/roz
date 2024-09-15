use crate::{
    lexer::{Token, TokenType},
    literal::Literal,
    roz,
    stmt::{Expr, Stmt},
};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    pub fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token_type(&[TokenType::Let]) {
            return self.var_declaration();
        }

        if self.match_token_type(&[TokenType::Fn]) {
            return self.fn_declaration("function");
        }

        return self.statement();
    }

    pub fn fn_declaration(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expected {} name", kind))?
            .clone();

        self.consume(
            TokenType::LeftParen,
            &format!("Expected '(' after {} name", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParseError {
                        token: self.peek().clone(),
                        message: "Can't have more than 255 parameters.".to_string(),
                    });
                }
                parameters.push(
                    self.consume(TokenType::Identifier, "Expected parameter name")?
                        .clone(),
                );

                if !self.match_token_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expected '{{' before {} body", kind),
        )?;
        let body = self.block()?;

        Ok(Stmt::Function(name, parameters, Box::new(body)))
    }

    pub fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name")?
            .clone();

        let mut initializer = Expr::Literal(Literal::Null);
        if self.match_token_type(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expected ';'")?;

        return Ok(Stmt::Var(name, initializer));
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token_type(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.match_token_type(&[TokenType::LeftBrace]) {
            return self.block();
        }

        if self.match_token_type(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.match_token_type(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.match_token_type(&[TokenType::For]) {
            return self.for_statement();
        }

        return self.expression_statement();
    }

    pub fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "';' expected.")?;

        return Ok(Stmt::Print(expr));
    }

    pub fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;

        self.consume(TokenType::Semicolon, "';' expected.")?;

        return Ok(Stmt::Expression(expr));
    }

    pub fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' before expression.")?;
        let expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

        let then_stmt = self.statement()?;

        let mut else_stmt = Stmt::None;
        if self.match_token_type(&[TokenType::Else]) {
            else_stmt = self.statement()?;
        }

        Ok(Stmt::If(expr, Box::new(then_stmt), Box::new(else_stmt)))
    }

    pub fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' before expression.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after expression.")?;

        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    pub fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expected '(' before expressions.")?;

        let initializer;
        if self.match_token_type(&[TokenType::Semicolon]) {
            initializer = Stmt::None;
        } else if self.match_token_type(&[TokenType::Let]) {
            initializer = self.var_declaration()?;
        } else {
            initializer = self.expression_statement()?;
        }

        let mut condition = Expr::None;
        if !self.check(&TokenType::Semicolon) {
            condition = self.expression()?;
        }
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let mut increment = Expr::None;
        if !self.check(&TokenType::RightParen) {
            increment = self.expression()?;
        }
        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if increment != Expr::None {
            body = Stmt::Block(Vec::from([body, Stmt::Expression(increment)]));
        }

        if condition == Expr::None {
            condition = Expr::Literal(Literal::Bool(true));
        }

        body = Stmt::While(condition, Box::new(body));

        if initializer != Stmt::None {
            body = Stmt::Block(Vec::from([initializer, body]));
        }

        return Ok(body);
    }

    pub fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}'.")?;

        Ok(Stmt::Block(statements))
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_token_type(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assign(name, Box::new(value)));
                }
                _ => {
                    return Err(ParseError {
                        token: equals.clone(),
                        message: "invalid assignment target.".to_string(),
                    });
                }
            }
        }

        Ok(expr)
    }

    pub fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token_type(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    pub fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token_type(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    pub fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    pub fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    pub fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    pub fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token_type(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        return Ok(expr);
    }

    pub fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        return self.call();
    }

    pub fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token_type(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    pub fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    // we are returning a error here because the parser is still in a valid state.
                    roz::error(self.peek(), "Can't have more than 255 arguments.");
                }

                arguments.push(self.expression()?);

                if !self.match_token_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(Expr::Call(Box::new(callee), paren.clone(), arguments))
    }

    pub fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_token_type(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.match_token_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_token_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Null));
        }

        if self.match_token_type(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        return Err(ParseError {
            token: self.peek().clone(),
            message: "Unable to parse the provided expression".to_string(),
        });
    }

    pub fn match_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                token: self.peek().clone(),
                message: message.to_string(),
            })
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
