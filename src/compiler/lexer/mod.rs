//! Scanner module. Contains public Token module
//! Use lexer::scan(sourceString) to scan source code to tokens.

pub mod token;
mod scanner;

use std::vec::Vec;

/// Scan the input string, return a vector of tokens (lexer::token::Token) or an error.
pub fn scan(input: &String) -> Result<Vec<token::Token>, String> {
    let mut scanner = scanner::Scanner::new();

    for c in (*input).chars() {
        scanner.consume(c);
    };

    scanner.into_tokens()
}
