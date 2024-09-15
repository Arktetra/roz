use std::{
    env,
    io::{self, Write},
    process::ExitCode
};

pub mod callable;
pub mod environment;
pub mod function;
pub mod literal;
pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod r#return;
pub mod stmt;
pub mod roz;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        roz::run_prompt();
    } else if args.len() == 2 {
            return roz::run_file(&args[1]);
    } else {
        if args.len() > 3 {
            writeln!(io::stderr(), "Usage: {}", args[0]).unwrap();
            writeln!(io::stderr(), "Usage: {} <filename>", args[0]).unwrap();
        }
    }
    
    ExitCode::SUCCESS
}
