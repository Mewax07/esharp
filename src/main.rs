use std::io::{Write, stdin, stdout};

use crate::parser::{eval::Environment, repl::{LineResult, process_line}};

pub(crate) mod parser;
pub(crate) mod utils;

fn main() {
    let mut env = Environment::new();
    let stdin = stdin();

    loop {
        print!("> ");
        stdout().flush().ok();
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "quit" || line == "exit" {
            break;
        }

        match process_line(line, &mut env) {
            Ok(LineResult::Value(v)) => println!("{}", v),
            Ok(LineResult::Symbolic(e)) => println!("{}", e),
            Err(e) => println!("Erreur : {}", e),
        }
    }
}
