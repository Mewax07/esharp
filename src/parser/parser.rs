use crate::parser::{
    ast::Expr,
    lexer::{Token, tokenize},
};

#[derive(Debug, Clone)]
pub enum Stmt {
    Eval(Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn peek_at(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset)
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, tok: &Token) -> Result<(), String> {
        if self.peek() == tok {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", tok, self.peek()))
        }
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let e = self.parse_expr()?;
        self.expect(&Token::Eof)?;
        Ok(Stmt::Eval(e))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let e = self.advance();
        eprintln!("{:?}", e);
        // match self.advance() {
        match e {
            Token::Num(n) => Ok(Expr::Num(n)),
            other => Err(format!("Token unexpected : {:?}", other)),
        }
    }
}

pub fn parse_stmt(input: &str) -> Result<Stmt, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse_stmt()
}
