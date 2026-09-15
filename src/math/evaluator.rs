use std::collections::HashMap;

use crate::math::{Calculation, Expr};

pub struct Evaluator {
    vars: HashMap<String, f64>,
    funcs: HashMap<String, fn(&[f64]) -> f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn with_var(mut self, name: &str, value: f64) -> Self {
        self.vars.insert(name.to_string(), value);
        self
    }

    pub fn with_func(mut self, name: &str, value: fn(&[f64]) -> f64) -> Self {
        self.funcs.insert(name.to_string(), value);
        self
    }

    pub fn eval(&self, expr: &Expr) -> Result<f64, String> {
        self.eval_expr(expr)
    }

    fn eval_expr(&self, expr: &Expr) -> Result<f64, String> {
        match expr {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => self
                .vars
                .get(name)
                .copied()
                .ok_or_else(|| format!("Variable '{}' not defined", name)),
            Expr::Add(left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                Ok(left_val + right_val)
            }
            Expr::Sub(left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                Ok(left_val - right_val)
            }
            Expr::Mul(left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                Ok(left_val * right_val)
            }
            Expr::Div(left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                if right_val == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left_val / right_val)
                }
            }
            Expr::Pow(left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                Ok(left_val.powf(right_val))
            }
            Expr::Neg(expr) => {
                let val = self.eval_expr(expr)?;
                Ok(-val)
            }
            Expr::Call { name, arguments } => {
                let args: Result<Vec<f64>, String> =
                    arguments.iter().map(|arg| self.eval_expr(arg)).collect();
                let args = args?;

                if let Some(func) = self.funcs.get(name) {
                    Ok(func(&args))
                } else {
                    Err(format!("Function '{}' not defined", name))
                }
            }
        }
    }

    pub fn eval_calculation(&self, calc: &mut Calculation) -> Result<(), String> {
        if let Some(ref ast) = calc.ast {
            let result = self.eval_expr(ast)?;
            calc.result = Some(result);

            // self.generate_steps(calc)?;

            Ok(())
        } else {
            Err("No AST to evaluate".to_string())
        }
    }
}
