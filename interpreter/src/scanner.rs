use std::{iter::Peekable, str::Chars};

use crate::token::Token;

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

    pub fn scan_tokens(&mut self, source: &mut Peekable<Chars>) {
        // TODO: Implement this
    }
}
