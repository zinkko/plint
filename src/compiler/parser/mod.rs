pub mod ast;
mod parser;
use super::scanner::token::*;

/// Parse a vector of tokens into an AST. Returns possible parsing errors.
pub fn parse(input: Vec<Token>) -> Result<ast::Ast, String> {
    parser::Parser::new(input).into_ast()
}
