use crate::math::ast::{Token, TokenType};

pub struct Lexer {
    input: String,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
            line: 1,
        }
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.pos).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.input.chars().nth(self.pos + 1).unwrap_or('\0')
    }

    fn advance(&mut self) {
        if let Some(c) = self.input.chars().nth(self.pos) {
            if c == '\n' {
                self.line += 1;
            }
            self.pos += 1;
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            let c = self.peek();

            match c {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::new(TokenType::Plus, "+ ".to_string(), self.line));
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::new(TokenType::Minus, "-".to_string(), self.line));
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::new(TokenType::Star, "* ".to_string(), self.line));
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::new(TokenType::Slash, "/ ".to_string(), self.line));
                    self.advance();
                }
                '^' => {
                    tokens.push(Token::new(TokenType::Power, "^ ".to_string(), self.line));
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::new(TokenType::Equals, "=".to_string(), self.line));
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::new(TokenType::LParen, "(".to_string(), self.line));
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::new(TokenType::RParen, ") ".to_string(), self.line));
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::new(TokenType::Comma, ", ".to_string(), self.line));
                    self.advance();
                }
                c if c.is_ascii_digit() || c == '.' => {
                    tokens.push(self.read_number());
                }
                c if c.is_ascii_alphabetic() => {
                    tokens.push(self.read_identifier());
                }
                _ => {
                    // ignore other
                    self.advance();
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, "EOF".to_string(), self.line));
        tokens
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        let mut has_dot = false;

        while self.pos < self.input.len() {
            let c = self.peek();

            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.input[start..self.pos];
        let value = text.parse().unwrap_or(0.0);

        Token::new(TokenType::Number(value), text.to_string(), self.line)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;

        while self.pos < self.input.len() {
            let c = self.peek();

            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.input[start..self.pos];

        Token::new(
            TokenType::Identifier(text.to_string()),
            text.to_string(),
            self.line,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokenizer() {
        let mut lexer = Lexer::new("2 + 5");
        let tokens = lexer.tokenize();

        assert_eq!(tokens.len(), 4); // 2, +, 5, EOF
        assert!(matches!(tokens[0].token_type, TokenType::Number(2.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Plus));
        assert!(matches!(tokens[2].token_type, TokenType::Number(5.0)));
    }
}
