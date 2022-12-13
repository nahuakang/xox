use crate::token::Token;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        // TODO: Implement this
        vec![]
    }
}
