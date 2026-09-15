use std::fmt::{self, Display, Formatter};

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

    pub fn mul(a: Expr, b: Expr) -> Expr {
        Expr::Mul(Box::new(a), Box::new(b))
    }

    pub fn div(a: Expr, b: Expr) -> Expr {
        Expr::Div(Box::new(a), Box::new(b))
    }

    pub fn neg(a: Expr) -> Expr {
        Expr::Neg(Box::new(a))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Add(a, b) => write!(f, "({} + {})", a, b),
            Expr::Sub(a, b) => write!(f, "({} - {})", a, b),
            Expr::Mul(a, b) => write!(f, "({} * {})", a, b),
            Expr::Div(a, b) => write!(f, "({} / {})", a, b),
            Expr::Pow(a, b) => write!(f, "({} ^ {})", a, b),
            Expr::Neg(a) => write!(f, "-{}", a),
            Expr::Call(name, args) => {
                write!(f, "{}(", name)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", a)?;
                }
                write!(f, ")")
            }
        }
    }
}
