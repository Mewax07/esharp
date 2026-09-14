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
    Call(String, Vec<Expr>),
}

impl Expr {
    pub fn num(n: i64) -> Expr {
        Expr::Num(Dec::from_i64(n))
    }

    pub fn add(a: Expr, b: Expr) -> Expr {
        Expr::Add(Box::new(a), Box::new(b))
    }

    pub fn sub(a: Expr, b: Expr) -> Expr {
        Expr::Sub(Box::new(a), Box::new(b))
    }

    pub fn neg(a: Expr) -> Expr {
        Expr::Neg(Box::new(a))
    }
}
