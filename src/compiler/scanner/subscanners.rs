
use super::token::{Token, Keyword};

struct WordScanner {
    buffer: String,
}
impl WordScanner {
    pub fn consume(&mut self, c: char) -> bool {
        if c.is_alphanumeric() {
            self.buffer.push(c);
            false
        } else {
            true
        }
    }

    pub fn into_token(self) -> Token {
        word_token(self.buffer)
    }
}

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

struct IntScanner {
    buffer: String
}
impl IntScanner {
    pub fn consume(&mut self, c: char) -> bool {
        if c.is_digit(10) {
            self.buffer.push(c);
            false
        } else {
            true
        }
    }

    pub fn into_token(self) -> Token {
        Token::Int(self.buffer.parse().unwrap())
    }
}

pub struct StringScanner {
    buffer: String,
    escaping: bool,
}
impl StringScanner {
    pub fn new() -> StringScanner {
        StringScanner { buffer: String::new(), escaping: false }
    }

    pub fn into_token(self) -> Token {
        Token::String(self.buffer)
    }

    /// Return Some(string) when found full string literal, None otherwise
    pub fn consume(&mut self, c: char) -> bool {
        if self.escaping {
            self.push_escaped(c);
            return false
        }
        if c == '"' {
            true
        } else {
            self.escaping = c == '\\';
            false
        }
    }

    fn push_escaped(&mut self, c: char) {
        match c {
            '\\' => self.buffer.push('\\'),
            '"' => self.buffer.push('"'),
            'n' => self.buffer.push('\n'),
            _ => { self.buffer.push('\\'); self.buffer.push(c) },
        }
        self.escaping = false;
    }
}

enum CommentState { Line, Multiline, MultilineEnding }
struct CommentScanner { state: CommentState }
impl CommentScanner {
    /// Return true if the comment is over
    pub fn consume(&mut self, c: char) -> bool {
        match self.state {
            CommentState::Line if c == '\n' => true,
            CommentState::MultilineEnding if c == '/' => true,
            CommentState::Multiline if c == '*' => {
                self.state = CommentState::MultilineEnding;
                false
            },
            CommentState::MultilineEnding => {
                self.state = CommentState::Multiline;
                false
            },
            _ => false,
        }
    }
}
