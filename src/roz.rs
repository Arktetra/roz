use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::{
    interpreter::{Interpreter, RuntimeError, RuntimeException},
    lexer::{Lexer, Token, TokenType},
    parser::Parser,
};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

pub fn run_prompt() {
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

        run(&input);

        unsafe {
            HAD_ERROR = false;
        }
    }
}

pub fn run_file(filename: &str) -> ExitCode {
    let filecontent = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    run(&filecontent);

    unsafe {
        if HAD_ERROR {
            ExitCode::from(65)
        } else if HAD_RUNTIME_ERROR {
            ExitCode::from(70)
        } else {
            ExitCode::SUCCESS
        }
    }
}

pub fn run(input: &str) {
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();

    let mut parser = Parser::new(lexer.tokens);
    let mut interpreter = Interpreter::new();

    match parser.parse() {
        Ok(stmts) => {
            unsafe {
                if HAD_ERROR {
                    return;
                }
            }

            if let Err(runtime_exception) = interpreter.interpret(&stmts) {
                match runtime_exception {
                    RuntimeException::Error(runtime_err) => runtime_error(runtime_err),
                    RuntimeException::Return(_) => (),
                }
            }
        }
        Err(parse_err) => error(&parse_err.token, &parse_err.message),
    }
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

pub fn runtime_error(error: RuntimeError) {
    writeln!(
        io::stderr(),
        "{}\n[line {}]",
        error.message,
        error.token.line
    )
    .unwrap();

    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
}

pub fn report(line: usize, whr: &str, message: &str) {
    // whr = where because where is a rust keyword
    writeln!(io::stderr(), "[Line {}] Error {}: {}", line, whr, message).unwrap();

    unsafe {
        HAD_ERROR = true;
    }
}
