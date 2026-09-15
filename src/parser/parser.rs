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
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul_div()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Expr::add(left, right);
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_mul_div()?;
                    left = Expr::sub(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::mul(left, right);
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::div(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.peek() == &Token::Minus {
            self.advance();
            let e = self.parse_unary()?;
            return Ok(Expr::neg(e));
        }
        if self.peek() == &Token::Plus {
            self.advance();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Num(n) => Ok(Expr::Num(n)),
            Token::Ident(name) => {
                if self.peek() == &Token::LParen {
                    self.advance(); // '('
                    let mut args = Vec::new();
                    if self.peek() != &Token::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.peek() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::LParen => {
                let e = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(e)
            }
            other => Err(format!("Token unexpected : {:?}", other)),
        }
    }
}

pub fn parse_stmt(input: &str) -> Result<Stmt, String> {
    let tokens = tokenize(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse_stmt()
}
