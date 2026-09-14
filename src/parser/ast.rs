use crate::utils::decimal::Dec;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(Dec),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    pub fn num(n: i64) -> Expr {
        Expr::Num(Dec::from_i64(n))
    }
}
