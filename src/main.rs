use std::io::{Write, stdin, stdout};

use crate::parser::{
    ast::Expr,
    eval::{Environment, eval},
    parser::{Stmt, parse_stmt},
};

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

        if let Err(e) = handle_line(line, &mut env) {
            println!("Error: {}", e)
        }
    }
}

fn handle_line(line: &str, env: &mut Environment) -> Result<(), String> {
    let stmt = parse_stmt(line)?;

    match stmt {
        Stmt::Eval(expr) => handle_expr(&expr, env)?,
    }

    Ok(())
}

fn handle_expr(expr: &Expr, env: &Environment) -> Result<(), String> {
    let value = eval(expr, env)?;
    println!("{:?}", value);
    Ok(())
}
