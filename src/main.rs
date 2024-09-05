use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

pub mod literal;
pub mod lexer;
pub mod expr;
pub mod parser;
pub mod interpreter;

use lexer::Lexer;


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
