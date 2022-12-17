#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RigthParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Literal(LiteralType),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralType {
    Identifier(String),
    String(String),
    Number(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}
