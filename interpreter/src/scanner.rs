use anyhow::Result;
use std::{iter::Peekable, str::Chars};
use thiserror::Error;

use crate::token::{reserved_to_tokentype, LiteralType, Token, TokenType};

pub struct Scanner {
    tokens: Vec<Token>,
    line: usize,
}

#[derive(Error, Debug, PartialEq)]
enum ScanError {
    #[error("Unexpected token {found:?} found on line {line:?}.")]
    UnexpectedToken { found: char, line: usize },
    #[error("Unterminated string.")]
    UnterminatedString,
    #[error("Invalid number format.")]
    InvalidNumberFormat,
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

    pub fn scan_tokens(&mut self, source: &mut Peekable<Chars>) -> Result<()> {
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
                '/' => {
                    // Deref coersion on &&'/'
                    if let Some(&'/') = source.peek() {
                        while let Some(c) = source.next() {
                            if c == '\n' {
                                self.line += 1;
                                break;
                            }
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                // String literals
                '"' => {
                    let mut s = String::new();
                    loop {
                        if let Some(c) = source.next() {
                            if c == '"' {
                                break;
                            }
                            if c != '\n' {
                                s.push(c);
                            } else {
                                self.line += 1;
                            }
                        } else {
                            anyhow::bail!(ScanError::UnterminatedString);
                        }
                    }
                }
                // Numeric literals
                '0'..='9' => {
                    let mut number_literal = String::new();
                    while let Some(c) = source.peek() {
                        if c.is_ascii_digit() {
                            number_literal.push(*c);
                            source.next();
                        } else if *c == '.' {
                            number_literal.push(*c);
                            source.next();

                            if let Some(digit) = source.peek() {
                                if !digit.is_ascii_digit() {
                                    anyhow::bail!(ScanError::InvalidNumberFormat);
                                }
                            } else {
                                anyhow::bail!(ScanError::InvalidNumberFormat);
                            }
                        } else {
                            break;
                        }
                    }

                    let number: f32 = number_literal.parse()?;
                    self.add_token(TokenType::Literal(LiteralType::Number(number)));
                }
                // Identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut id = String::new();
                    id.push(c);

                    while let Some(c) = source.peek() {
                        if c.is_ascii_alphanumeric() || *c == '_' {
                            id.push(*c);
                            source.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(keyword) = reserved_to_tokentype(&id) {
                        self.add_token(keyword);
                    } else {
                        self.add_token(TokenType::Literal(LiteralType::Identifier(id)));
                    }
                }
                // Ignore whitespaces
                ' ' | '\r' | '\t' => {}
                '\n' => self.line += 1,
                _ => anyhow::bail!(ScanError::UnexpectedToken {
                    found: c,
                    line: self.line
                }),
            }
        }

        self.add_token(TokenType::EOF);
        Ok(())
    }
}
