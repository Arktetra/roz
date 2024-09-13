use crate::{
    expr::Expr,
    lexer::{Token, TokenType},
    literal::Literal,
};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    pub fn equality(&mut self) -> Result<Expr, ParseError> {
        let left = self.comparison()?;

        while self.match_token_type(Vec::from([TokenType::BangEqual, TokenType::EqualEqual])) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            return Ok(Expr::Binary(Box::new(left), Box::new(operator), Box::new(right)));
        }

        return Ok(left);
    }

    pub fn comparison(&mut self) -> Result<Expr, ParseError> {
        let left = self.term()?;

        while self.match_token_type(Vec::from([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            return Ok(Expr::Binary(Box::new(left), Box::new(operator), Box::new(right)));
        }

        return Ok(left);
    }

    pub fn term(&mut self) -> Result<Expr, ParseError> {
        let left = self.factor()?;

        while self.match_token_type(Vec::from([TokenType::Plus, TokenType::Minus])) {
            let operator = self.previous().clone();
            let right = self.term()?;

            return Ok(Expr::Binary(Box::new(left), Box::new(operator), Box::new(right)));
        }

        return Ok(left);
    }

    pub fn factor(&mut self) -> Result<Expr, ParseError> {
        let left = self.unary()?;

        while self.match_token_type(Vec::from([TokenType::Star, TokenType::Slash])) {
            let operator = self.previous().clone();
            let right = self.term()?;

            return Ok(Expr::Binary(Box::new(left), Box::new(operator), Box::new(right)));
        }

        return Ok(left);
    }

    pub fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator = self.previous().clone();
            let right = self.term()?;

            return Ok(Expr::Unary(Box::new(operator), Box::new(right)));
        }

        return self.primary();
    }

    pub fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(Vec::from([TokenType::True])) {
            return Ok(Expr::Literal(Box::new(Literal::Bool(true))));
        }

        if self.match_token_type(Vec::from([TokenType::False])) {
            return Ok(Expr::Literal(Box::new(Literal::Bool(false))));
        }

        if self.match_token_type(Vec::from([TokenType::Number, TokenType::String])) {
            return Ok(Expr::Literal(Box::new(self.previous().literal.clone())));
        }

        if self.match_token_type(Vec::from([TokenType::LeftParen])) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.match_token_type(Vec::from([TokenType::Nil])) {
            return Ok(Expr::Literal(Box::new(Literal::Null)));
        }

        return Err(ParseError {token: self.peek().clone(), message: "Unable to parse the provided expression".to_string()});
    }

    pub fn match_token_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError{token: self.peek().clone(), message: message.to_string()})
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
