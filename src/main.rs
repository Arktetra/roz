use std::{
    env,
    fs,
    io::{self, Write},
    process::ExitCode
};

use roz::Flag;

pub mod literal;
pub mod lexer;
pub mod expr;
pub mod parser;
pub mod interpreter;
pub mod roz;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        roz::run_prompt(Flag::Run);
    } else if args.len() == 2 {
        if args[1] == "--ast" {
            roz::run_prompt(Flag::Ast);
        } else {
            let filename = &args[1];
            let filecontent = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                    String::new()
                });
    
            println!("{}", roz::run(&filecontent));
        }
    } else {
        if args.len() > 3 {
            writeln!(io::stderr(), "Usage: {}", args[0]).unwrap();
            writeln!(io::stderr(), "Usage: {} <filename>", args[0]).unwrap();
            writeln!(io::stderr(), "Usage: {} ast", args[0]).unwrap();
        }
    }
    
    ExitCode::SUCCESS
}
