use std::mem::discriminant;

use crate::math::ast::{Expr, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match token_type {
            TokenType::Number(_) => matches!(self.peek().token_type, TokenType::Number(_)),
            TokenType::Identifier(_) => matches!(self.peek().token_type, TokenType::Identifier(_)),
            _ => discriminant(&self.peek().token_type) == discriminant(token_type),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<&Token> {
        if self.check(&token_type) {
            self.advance();
            Some(self.previous())
        } else {
            eprintln!("Error at line {}: {}", self.peek().line, message);
            None
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn function_call(&mut self, name: String) -> Option<Expr> {
        let mut arguments = Vec::new();

        if !self.match_token(&[TokenType::RParen]) {
            arguments.push(self.expression()?);

            while self.match_token(&[TokenType::Comma]) {
                arguments.push(self.expression()?);
            }

            self.consume(TokenType::RParen, "Expect ')' after arguments")?;
        }

        Some(Expr::Call { name, arguments })
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.addition()
    }

    fn addition(&mut self) -> Option<Expr> {
        let mut expr = self.multiplication()?;

        Some(expr)
    }

    fn multiplication(&mut self) -> Option<Expr> {
        let mut expr = self.power()?;

        Some(expr)
    }

    fn power(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;

        Some(expr)
    }

    fn unary(&mut self) -> Option<Expr> {
        self.primary()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[TokenType::Number(0.0)]) {
            if let TokenType::Number(value) = self.previous().token_type {
                return Some(Expr::Num(value));
            }
        }

        if self.match_token(&[TokenType::Identifier(String::new())]) {
            if let TokenType::Identifier(name) = &self.previous().token_type {
                let name = name.clone();

                if self.match_token(&[TokenType::LParen]) {
                    return self.function_call(name);
                }

                return Some(Expr::Var(name));
            }
        }

        if self.match_token(&[TokenType::LParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RParen, "Expect ')' after expression")?;
            return Some(expr);
        }

        None
    }
}
