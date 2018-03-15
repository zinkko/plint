pub mod ast;
mod parser;
use super::lexer::token::*;

pub fn parse(input: Vec<Token>) -> ast::Ast {
    parser::Parser::new(input).into_ast()
}
