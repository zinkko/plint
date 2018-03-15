pub struct Ast {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration { identifier: String, mpl_type: MplType, value: Option<Expression> },
    Assignment { identifier: String, expression: Expression },
    For { identifier: String, begin: Expression, end: Expression, statements: Vec<Statement> },
    Read(String), // Identifier
    Print(Expression),
    Assert(Expression),
    Empty, // TODO remove!
}

#[derive(Debug)]
pub enum Expression {
    Simple(Operand),
    Binary { left: Operand, operator: char, right: Operand },
    Unary { operator: char, operand: Operand },
}

#[derive(Debug)]
pub enum Operand {
    Int(i32),
    String(String),
    Identifier(String),
    Expr(Box<Expression>),
}

#[derive(Debug)]
pub enum MplType {
    Int,
    String,
    Bool,
}
