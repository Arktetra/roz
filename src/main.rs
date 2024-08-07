use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

pub enum TokenType {
    // single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Semicolon, Minus, Plus, Slash, Star,

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
    line: usize,
    had_error: bool
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
            '\n' => self.line += 1,
            _ => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", self.line, c).unwrap();
                self.had_error = true;
            }
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

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::from(1);
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

                if lexer.had_error {
                    return ExitCode::from(65);
                }

            } else {
                println!("EOF null");
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
