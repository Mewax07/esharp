use std::collections::HashMap;

use crate::{parser::ast::Expr, utils::decimal::Dec};

#[derive(Default, Clone)]
pub struct Environment {
    pub vars: HashMap<String, Dec>,
    pub funcs: HashMap<String, (Vec<String>, Expr)>,
}

impl Environment {
    pub fn new() -> Self {
        Environment::default()
    }
}

pub fn eval(expr: &Expr, env: &Environment) -> Result<Dec, String> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Add(a, b) => eval(a, env)?.checked_add(eval(b, env)?).ok_or_else(|| "Overflow in addition".to_string()),
        Expr::Sub(a, b) => eval(a, env)?.checked_sub(eval(b, env)?).ok_or_else(|| "Overflow in substraction".to_string()),
        Expr::Neg(a) => Ok(-eval(a, env)?),
        _ => Ok(Dec::zero()),
    }
}
