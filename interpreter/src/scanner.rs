use std::{iter::Peekable, str::Chars};

use crate::{
    error::error,
    token::{Token, TokenType},
};

pub struct Scanner {
    tokens: Vec<Token>,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line: 1,
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line));
    }

    pub fn scan_tokens(&mut self, source: &mut Peekable<Chars>) {
        // TODO: Implement this
        while let Some(c) = source.next() {
            match c {
                // Single-Character Lexemes
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RigthParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                // Two-Character Operators
                '!' => {
                    if source.next_if(|&c| c == '=').is_some() {
                        self.add_token(TokenType::BangEqual);
                    } else {
                        self.add_token(TokenType::Bang);
                    }
                }
                '=' => {
                    if source.next_if(|&c| c == '=').is_some() {
                        self.add_token(TokenType::EqualEqual);
                    } else {
                        self.add_token(TokenType::Equal);
                    }
                }
                '<' => {
                    if source.next_if(|&c| c == '=').is_some() {
                        self.add_token(TokenType::LessEqual);
                    } else {
                        self.add_token(TokenType::Less);
                    }
                }
                '>' => {
                    if source.next_if(|&c| c == '=').is_some() {
                        self.add_token(TokenType::GreaterEqual);
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                _ => error(self.line, "Unexpected character."),
            }
        }
    }
}
