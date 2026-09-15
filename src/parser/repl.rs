use crate::{parser::{ast::Expr, eval::{Environment, eval}, parser::{Stmt, parse_stmt}}, utils::decimal::Dec};

#[derive(Debug, Clone)]
pub enum LineResult {
    Value(Dec),
    Symbolic(Expr),
}

pub fn process_line(line: &str, env: &mut Environment) -> Result<LineResult, String> {
    let stmt = parse_stmt(line)?;

    match stmt {
        Stmt::Eval(expr) => process_expr(&expr, env),
    }
}

fn process_expr(expr: &Expr, env: &Environment) -> Result<LineResult, String> {
	let value = eval(expr, env)?;
    Ok(LineResult::Value(value))
}
