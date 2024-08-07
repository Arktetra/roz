use std::env;
use std::fs;
use std::io::{self, Write};

pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EOF
}

impl TokenType {
    pub fn to_string(&self) -> String{
        match self {
            Self::LeftParen => "LEFT_PAREN".to_string(),
            Self::RightParen => "RIGHT_PAREN".to_string(),
            Self::LeftBrace => "LEFT_BRACE".to_string(),
            Self::RightBrace => "RIGHT_BRACE".to_string(),
            Self::EOF => "EOF".to_string()
        }
    }
}

pub enum Literal {
    Number(f32),
    String(String),
    Null
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(x) => format!("{}", x),
            Self::String(x) => x.to_string(),
            Self::Null => "null".to_string()
        }
    }
}

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

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Lexer {
    pub fn new(source: &String) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
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
            _ => writeln!(io::stderr(), "Unexpected character: {}", c).unwrap(),
        }
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(
            Token::new(token_type, text.to_string(), literal, self.line)
        )
    }

    pub fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return c;
    }

    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {

            let filecontent = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
            });

            let mut lexer = Lexer::new(&filecontent);

            if !filecontent.is_empty() {
                // panic!("Scanner not implemented");
                lexer.scan_tokens();

                for token in lexer.tokens {
                    println!("{}", token.to_string());
                }

            } else {
                println!("EOF null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
