mod token;
mod scanner;

use std::vec::Vec;

pub fn scan(input: &String) -> Vec<token::Token> {
    let mut scanner = scanner::Scanner::new();

    for c in (*input).chars() {
        scanner.consume(c);
    };

    scanner.into_tokens()
}
