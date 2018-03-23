pub mod ast;
mod parser;
use super::lexer::token::*;

pub fn parse(input: Vec<Token>) -> Result<ast::Ast, String> {
    parser::Parser::new(input).into_ast()
}
