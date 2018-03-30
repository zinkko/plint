#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Reserved(Keyword),
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Var,
    For,
    End,
    In,
    Do,
    Read,
    Print,
    Assert,
    Int,
    String,
    Bool,
}
