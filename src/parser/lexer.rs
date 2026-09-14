use crate::{parser::{ast::Expr, lexer::Token::{Eof, Num}}, utils::decimal::Dec};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(Dec),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    Equal,
    Eof,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
	let chars: Vec<char> = input.chars().collect();
	let mut i = 0;
	let mut tokens = Vec::new();

	while i < chars.len() {
		let c = chars[i];

		if c.is_whitespace() {
			i += 1;
			continue;
		}

		match c {
    		_ => i += 1
		}
	}

	tokens.push(Token::Eof);
	Ok(tokens)
}
