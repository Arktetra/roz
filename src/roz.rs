use std::io::{self, Write};

use crate::{
    expr::AstPrinter,
    lexer::{Lexer, Token, TokenType},
    parser::Parser,
    interpreter::Interpreter
};

static mut HAD_ERROR: bool = false;

pub enum Flag {
    Ast,
    Run
}

/// Runs the interpreter in REPL mode. `flag` is used to set the 
/// interpreter to print ast or the result.
/// 
/// # Examples
/// 
/// ```
/// use roz;
/// 
/// roz::run_prompt(Flag::Run);
/// ```
pub fn run_prompt(flag: Flag) {
    loop {
        print!("#> ");
        let mut input = String::new();

        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Did not enter correct string");

        if input.trim() == "" {
            break;
        }

        match flag {
            Flag::Ast =>  println!("#> {}", ast(&input)),
            Flag::Run =>  println!("#> {}", run(&input))
        } 
    }
}

pub fn run(input: &str) -> String {
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();

    let mut parser = Parser::new(lexer.tokens);

    let mut interpreter = Interpreter;
    return interpreter.interpret(parser.expression());
}


pub fn ast(input: &str) -> String {
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();

    let mut parser = Parser::new(lexer.tokens);
    let mut printer = AstPrinter;

    return printer.print(&parser.expression());
}

pub fn lexical_error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error(token: &Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.line, "at the end", message);
    } else {
        report(token.line, &format!("at '{}'", token.lexeme), message);
    }
}

pub fn report(line: usize, whr: &str, message: &str) {
    // whr = where because where is a rust keyword
    writeln!(io::stderr(), "[Line {}] Error {}: {}", line, whr, message).unwrap();

    unsafe {
        HAD_ERROR = true;
    }
}
