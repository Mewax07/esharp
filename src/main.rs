use std::io::{Write, stdin, stdout};

use crate::parser::{ast::Expr, parser::{Stmt, parse_stmt}};

pub(crate) mod parser;
pub(crate) mod utils;

fn main() {
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

        if let Err(e) = handle_line(line) {
            println!("Error: {}", e)
        }
    }
}

fn handle_line(line: &str) -> Result<(), String> {
    let stmt = parse_stmt(line)?;

    match stmt {
        Stmt::Eval(expr) => handle_expr(&expr)?,
    }

    Ok(())
}

fn handle_expr(expr: &Expr) -> Result<(), String> {
	println!("{:?}", expr);
	Ok(())
}
