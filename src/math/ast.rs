use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Equals,
    LParen,
    RParen,
    Comma,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Call { name: String, arguments: Vec<Expr> },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Add(left, right) => write!(f, "({} + {})", left, right),
            Expr::Sub(left, right) => write!(f, "({} - {})", left, right),
            Expr::Mul(left, right) => write!(f, "({} * {})", left, right),
            Expr::Div(left, right) => write!(f, "({} / {})", left, right),
            Expr::Pow(left, right) => write!(f, "{}^({})", left, right),
            Expr::Neg(expr) => write!(f, "-({})", expr),
            Expr::Call { name, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", name, args.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Calculation {
    pub expression: String,
    pub ast: Option<Expr>,
    pub result: Option<f64>,
    pub steps: Vec<CalculationStep>,
}

impl Calculation {
    pub fn new(expression: &str) -> Self {
        Self {
            expression: expression.to_string(),
            ast: None,
            result: None,
            steps: Vec::new(),
        }
    }

    pub fn with_ast(mut self, ast: Expr) -> Self {
        self.ast = Some(ast);
        self
    }

    pub fn with_result(mut self, result: f64) -> Self {
        self.result = Some(result);
        self
    }

    pub fn add_step(mut self, step: CalculationStep) -> Self {
        self.steps.push(step);
        self
    }
}

#[derive(Debug, Clone)]
pub struct CalculationStep {
    pub description: String,
    pub expression: String,
    pub value: Option<f64>,
}

impl CalculationStep {
    pub fn new(description: &str, expression: &str) -> Self {
        Self {
            description: description.to_string(),
            expression: expression.to_string(),
            value: None,
        }
    }

    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: Vec::new(),
            body: Expr::Num(0.0),
        }
    }

    pub fn param(mut self, param: &str) -> Self {
        self.params.push(param.to_string());
        self
    }

    pub fn body(mut self, body: Expr) -> Self {
        self.body = body;
        self
    }
}
