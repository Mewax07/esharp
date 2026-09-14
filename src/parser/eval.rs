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
        _ => Ok(Dec::zero()),
    }
}
