
use super::token::Token;
use super::token::Keyword;
use std::mem::replace;


/// Filter a word into either an identifier, reserved keyword, or a boolean.
fn word_token(word: String) -> Token {
    match word.as_ref() {
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "var" => Token::Reserved(Keyword::Var),
        "for" => Token::Reserved(Keyword::For),
        "end" => Token::Reserved(Keyword::End),
        "in" => Token::Reserved(Keyword::In),
        "do" => Token::Reserved(Keyword::Do),
        "read" => Token::Reserved(Keyword::Read),
        "print" => Token::Reserved(Keyword::Print),
        "int" => Token::Reserved(Keyword::Int),
        "string" => Token::Reserved(Keyword::String),
        "bool" => Token::Reserved(Keyword::Bool),
        "assert" => Token::Reserved(Keyword::Assert),
        _ => Token::Identifier(word),
    }
}
