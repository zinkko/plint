//! Scanner module. Contains public Token module
//! Use scanner::scan(sourceString) to scan source code to tokens.

pub mod token;
mod subscanners;

use self::token::Token;
use self::subscanners::*;

use std::mem;
use std::str::Chars;
use std::vec::Vec;

pub fn scan(source: String) -> Result<Vec<Token>, String> {
    let scanner = Scanner { tokens: Vec::new(), buffer: None, input: source.chars() };
    scanner.scan()
}


struct Scanner<'scanner> {
    tokens: Vec<Token>,
    input: Chars<'scanner>,
    buffer: Option<char>,
}

impl<'scanner> Scanner<'scanner> {
    /// Scan the input string, return a vector of tokens (scanner::token::Token) or an error.
    fn scan(mut self) -> Result<Vec<Token>, String> {

        while let Some(chr) = self.next() {
            let next_token = match chr {
                _ if is_letter(chr) => { self.buffer = Some(chr); self.scan_word()? },
                _ if chr.is_digit(10) => { self.buffer = Some(chr); self.scan_int()? },
                _ if is_operator(chr) => Token::Operator(chr),
                '/' | ':' => { self.buffer = Some(chr); self.lookahead() },
                '"' => self.scan_string()?,
                ';' => Token::EndStatement,
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '.' => self.range_or_err(self.expect_next()?)?,
                _ => return Err(format!("Character '{}' not allowed at this point", chr)),
            };
            self.tokens.push(next_token);
        }

        Ok(self.tokens)
    }

    fn scan_string(&mut self) -> Result<Token, String> {
        Err("".to_string())
    }

    fn scan_int(&mut self) -> Result<Token, String> {
        Err("".to_string())
    }

    fn scan_word(&mut self) -> Result<Token, String> {
        Err("".to_string())
    }

    fn lookahead(&mut self) -> Token {

    }

    fn range_or_err(&mut self, next: char) -> Result<Token, String> {
        match next {
            '.' => Ok(Token::Range),
            chr => Err(format!("Expected range (..) got: .{}", chr)),
        }
    }

    /// Returns next char. Automatically takes the token from the internal buffer
    /// or the iterator, whichever is appropriate. Use this instead of self.expect_next() when
    /// not requiring another token.
    fn next(&mut self) -> Option<char> {
        if self.buffer == None {
            self.input.next()
        } else {
            // swap out self.buffer with None, returning the value the buffer used to have
            mem::replace(&mut self.buffer, None)
        }
    }

    /// Returns next char, or an error if there is no next char. Use this instead of self.next()
    /// when you are expecting another token.
    fn expect_next(&mut self) -> Result<char, String> {
        match self.next() {
            Some(c) => Ok(c),
            None => Err("Reached end while scanning".to_string()),
        }
    }
}

fn is_letter(c: char) -> bool {
    match c {
        'a' ... 'z' => true,
        'A' ... 'Z' => true,
        _ => false,
    }
}

/// returns false for /, it is handled separately
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '&' | '!' => true,
        _ => false,
    }
}
