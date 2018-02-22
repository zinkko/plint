#[derive(Debug, Clone)]
pub enum Token {
    Reserved(String),
    Identifier(String),
    Int(i32),
    String(String),
    Bool(bool),
    Operator(char),
    OpenParen,
    CloseParen,
    Assignment,
    TypeDecl,
    Range,
    Dot,
    EndStatement,
    InvalidToken(String),
}
