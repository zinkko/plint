use std::fmt;

/// Abstract Syntax Tree for Mini PL. There is no Node struct/enum, but the nodes are different
/// enums depending on the type of node. For example the nodes can be Operand enums or Expression
/// enums.
pub struct Ast {
    pub statements: Vec<Statement>,
}

/// The statement enum. One of the AST node types.
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Declaration { identifier: String, mpl_type: MplType, value: Option<Expression> },
    Assignment { identifier: String, expression: Expression },
    For { identifier: String, begin: Expression, end: Expression, statements: Vec<Statement> },
    Read(String), // Identifier
    Print(Expression),
    Assert(Expression),
    Empty, // TODO remove!
}

/// The statement enum. One of the AST node types.
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Simple(Operand),
    Binary { left: Operand, operator: char, right: Operand },
    Unary { operator: char, operand: Operand },
}

/// An operand of an expression. One of the AST Node types.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Int(i32),
    String(String),
    Identifier(String),
    Expr(Box<Expression>),
}

/// MplType represents a type in mpl. This enum is used both by the AST to mark types, and the
/// interpreter to reason about types more cleanly.
#[derive(Clone, Debug, PartialEq)]
pub enum MplType {
    Int,
    String,
    Bool,
}

impl fmt::Display for MplType {
    // Implement the display trait for printing types nicely in errormessages
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MplType::Int => write!(f, "Integer"),
            &MplType::String => write!(f, "String"),
            &MplType::Bool => write!(f, "Boolean"),
        }
    }
}
