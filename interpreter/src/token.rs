#[derive(Debug)]
pub enum TokenType {
    // TODO:  Add enough TokenType variants to compile. Implement in 4.2.
    LeftParen,
    RigthParen,
}

#[derive(Debug)]
pub struct Token {
    // TODO: Add enough Token variants to compile. Implement in 4.2.
    pub tpe: TokenType,
    pub lexeme: String,
    pub line: i32,
}
