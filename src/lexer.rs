use std::io::{self, Write};
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon, Minus, Plus, Slash, Star,

    // Operators
    Equal, EqualEqual, Bang, BangEqual,
    Less, LessEqual, Greater, GreaterEqual,

    //Literals
    Identifier, String, Number,

    // reserved words
    And, Or, Class, Super, This, If, Else, For, While,
    False, True, Fn, Return, Print, Let, Nil, 

    EOF
}

impl TokenType {
    pub fn to_string(&self) -> String{
        match self {
            Self::LeftParen => "LEFT_PAREN".to_string(),
            Self::RightParen => "RIGHT_PAREN".to_string(),
            Self::LeftBrace => "LEFT_BRACE".to_string(),
            Self::RightBrace => "RIGHT_BRACE".to_string(),
            Self::Comma => "COMMA".to_string(),
            Self::Dot => "DOT".to_string(),
            Self::Semicolon => "SEMICOLON".to_string(),
            Self::Minus => "MINUS".to_string(),
            Self::Plus => "PLUS".to_string(),
            Self::Slash => "SLASH".to_string(),
            Self::Star => "STAR".to_string(),
            Self::Equal => "EQUAL".to_string(),
            Self::EqualEqual => "EQUALEQUAL".to_string(),
            Self::Bang => "BANG".to_string(),
            Self::BangEqual => "BANGEQUAL".to_string(),
            Self::Less => "LESS".to_string(),
            Self::LessEqual => "LESSEQUAL".to_string(),
            Self::Greater => "GREATER".to_string(),
            Self::GreaterEqual => "GREATEREQUAL".to_string(),
            Self::Identifier => "IDENTIFIER".to_string(),
            Self::String => "STRING".to_string(),
            Self::Number => "NUMBER".to_string(),
            Self::And => "AND".to_string(),
            Self::Or => "OR".to_string(),
            Self::Class => "CLASS".to_string(),
            Self::Super => "SUPER".to_string(),
            Self::This => "THIS".to_string(),
            Self::If => "IF".to_string(),
            Self::Else => "ELSE".to_string(),
            Self::For => "FOR".to_string(),
            Self::While => "WHILE".to_string(),
            Self::False => "FALSE".to_string(),
            Self::True => "TRUE".to_string(),
            Self::Fn => "FN".to_string(),
            Self::Return => "RETURN".to_string(),
            Self::Print => "PRINT".to_string(),
            Self::Let => "LET".to_string(),
            Self::Nil => "NIL".to_string(),
            Self::EOF => "EOF".to_string()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
    Null
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(x) => {
                if x.to_string().ends_with(".0") || !x.to_string().contains('.') {
                    return format!("{:.1}", x);
                } else {
                    return format!("{}", x);
                }
            }
            Self::String(x) => x.to_string(),
            Self::Null => "null".to_string()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type.to_string(), self.lexeme, self.literal.to_string())
    }
}

fn keywords() -> &'static HashMap<&'static str, TokenType> {
    static HASHMAP: OnceLock<HashMap<&str, TokenType>> = OnceLock::new();

    HASHMAP.get_or_init(|| {
        HashMap::from([
            ("and",     TokenType::And),
            ("or",      TokenType::Or),
            ("class",   TokenType::Class),
            ("super",   TokenType::Super),
            ("this",    TokenType::This),
            ("if",      TokenType::If),
            ("else",    TokenType::Else),
            ("for",     TokenType::For),
            ("while",   TokenType::While),
            ("false",   TokenType::False),
            ("true",    TokenType::True),
            ("fn",      TokenType::Fn),
            ("return",  TokenType::Return),
            ("print",   TokenType::Print),
            ("Let",     TokenType::Let),
            ("nil",     TokenType::Nil)
        ])
    })
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool
}

impl Lexer {
    pub fn new(source: &String) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(TokenType::EOF, "".to_string(), Literal::Null, self.line)
        )
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Null),
            ')' => self.add_token(TokenType::RightParen, Literal::Null),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Null),
            '}' => self.add_token(TokenType::RightBrace, Literal::Null),
            ',' => self.add_token(TokenType::Comma, Literal::Null),
            '.' => self.add_token(TokenType::Dot, Literal::Null),
            ';' => self.add_token(TokenType::Semicolon, Literal::Null),
            '-' => self.add_token(TokenType::Minus, Literal::Null),
            '+' => self.add_token(TokenType::Plus, Literal::Null),
            '/' => self.add_token(TokenType::Slash, Literal::Null),
            '*' => self.add_token(TokenType::Star, Literal::Null),
            '=' => {
                let token_type = self.next_char_equal('=', TokenType::EqualEqual, TokenType::Equal);
                self.add_token(token_type, Literal::Null);
            }
            '!' => {
                let token_type = self.next_char_equal('=', TokenType::BangEqual, TokenType::Bang);
                self.add_token(token_type, Literal::Null);
            }
            '<' => {
                let token_type = self.next_char_equal('=', TokenType::LessEqual, TokenType::Less);
                self.add_token(token_type, Literal::Null);
            }
            '>' => {
                let token_type = self.next_char_equal('=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_token(token_type, Literal::Null);
            }
            '"' => {
                self.string();
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            x => {
                if x.is_alphabetic() || x == '_' {
                    self.identifier();
                } else if x.is_digit(10) {
                    self.number();
                } else {
                    self.error(&format!("Unexpected character: {}", c));
                }
            }
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(
            Token::new(token_type, text.to_string(), literal, self.line)
        )
    }

    pub fn identifier(&mut self) {
        loop {
            if let Some(x) = self.peek() {
                if x.is_alphanumeric() || x == '_' {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let text = &self.source[self.start..self.current];

        if let Some(token_type) = keywords().get(text) {
            self.add_token(token_type.clone(), Literal::Null);
        } else {
            self.add_token(TokenType::Identifier, Literal::Null);
        }
    }

    pub fn string(&mut self) {
        loop {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            
            if self.advance() == '"' {
                let text = &self.source[self.start + 1..self.current - 1];
                self.add_token(TokenType::String, Literal::String(text.to_string()));
                break;
            }

            if self.is_at_end() {
                self.error("Unterminated string.");
                break;
            }
        }
    }

    pub fn number(&mut self) {
        loop {
            if let Some(x) = self.peek() {
                if x.is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if let Some('.') = self.peek() {
            if let Some(x) = self.peek_next() {
                if x.is_digit(10) {
                    self.advance();
                }
            }
        }

        loop {
            if let Some(x) = self.peek() {
                if x.is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let text = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Literal::Number(text.parse::<f32>().unwrap()));
    }

    pub fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    pub fn next_char_equal(&mut self, c: char, equal_type: TokenType, unequal_type: TokenType) -> TokenType {
        if let Some(x) = self.peek() {
            if x == c {
                self.advance();
                return equal_type;
            }
        }
        return unequal_type;
    }

    pub fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len()
    }

    pub fn error(&mut self, message: &str) {
        writeln!(io::stderr(), "[line {}] Error: {}", self.line, message).unwrap();
        self.had_error = true;
    } 
}