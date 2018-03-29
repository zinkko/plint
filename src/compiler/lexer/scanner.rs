
use super::token::Token;
use super::token::Keyword;
use std::mem::replace;

/// The internal state of the scanner.
enum State {
    Empty,
    Comment,
    Unclear,
    ReadingEscape,
    ReadingString,
    ReadingInt,
    ReadingWord,
}

/// The scanner is a state machine, with a buffer for reading multicharacter tokens.
pub struct Scanner {
    tokens: Vec<Token>,
    buffer: String,
    state: State,
}

impl Scanner {
    /// Initialize a scanner.
    pub fn new() -> Scanner {
        Scanner { tokens: Vec::new(), buffer: String::new(), state: State::Empty }
    }

    /// Consume the next character from input. The Scanner expects characters to be fed externally.
    pub fn consume(&mut self, c: char) {
        match self.state {
            State::Unclear => {
                let stored_c = match self.buffer.chars().next() {
                    Some(v) => v,
                    None => panic!("buffer is empty when entering unclear. (scanner)"),
                };
                self.buffer = String::new();
                self.state = State::Empty;
                match (stored_c, c) {
                    (':', '=') => self.tokens.push(Token::Assignment),
                    ('/', '/') => self.state = State::Comment,
                    ('.', '.') => self.tokens.push(Token::Range),
                    (':', _) => {
                        self.tokens.push(Token::TypeDecl);
                        self.consume(c) // re-consume with new state
                    },
                    ('.', _) => {
                        self.tokens.push(Token::Dot);
                        self.consume(c) // re-consume
                    },
                    _ => {
                        self.tokens.push(Token::Operator(stored_c));
                        self.consume(c) // re-consume with new state
                    }
                }
            },
            State::Comment => {
                if c == '\n' {
                    self.state = State::Empty
                }
            },
            State::Empty => {
                match c {
                    '"' => {
                        self.state = State::ReadingString;
                    },
                    _ if is_integral(c) => {
                        self.buffer.push(c);
                        self.state = State::ReadingInt;
                    },
                    _ if is_alphanumeric(c) => {
                        self.buffer.push(c);
                        self.state = State::ReadingWord;
                    },
                    _ if is_unclear(c) => {
                        self.buffer.push(c);
                        self.state = State::Unclear;
                    },
                    _ if is_operator(c) => self.tokens.push(Token::Operator(c)),
                    ')' => self.tokens.push(Token::CloseParen),
                    '(' => self.tokens.push(Token::OpenParen),
                    ';' => self.tokens.push(Token::EndStatement),
                    _ => (),
                }
            },
            State::ReadingEscape => {
                    match c {
                        '"' | '\\' => self.buffer.push(c),
                        'n' => self.buffer.push('\n'),
                        _ => { self.buffer.push('\\'); self.buffer.push(c) },
                    }
                    self.state = State::ReadingString;
            },
            State::ReadingString if c == '\\' => self.state = State::ReadingEscape,
            _ => {
                if read_end(&self.state, c) {
                    self.add_token();
                    self.state = State::Empty;
                    match c {
                        _ if is_unclear(c) => {
                            self.buffer.push(c);
                            self.state = State::Unclear;
                        },
                        _ if is_operator(c) => self.tokens.push(Token::Operator(c)),
                        ';' => self.tokens.push(Token::EndStatement),
                        ')' => self.tokens.push(Token::CloseParen),
                        '(' => self.tokens.push(Token::OpenParen),
                        _ => (),
                    }
                } else {
                    self.buffer.push(c);
                }
            }
        }
    }

    /// Extract tokens from the scanner. If the scanner is still reading, return error.
    pub fn into_tokens(self) -> Result<Vec<Token>, String> {
        let end = "Reached end while scanning";
        match self.state {
            State::Empty | State::Comment => Ok(self.tokens),
            State::Unclear => Err(format!("{}. {} expected continuation", end, self.buffer)),
            State::ReadingInt => Err(format!("{} integer.", end)),
            State::ReadingString | State::ReadingEscape => Err(format!("{} string.", end)),
            State::ReadingWord =>  Err(format!("{} word.", end)),
        }
    }

    /// Helper for creating long tokens, i.e. integer, string, or word.
    fn add_token(&mut self) {
        let new_token = match self.state {
            State::ReadingInt => {
                let literal = replace(&mut self.buffer, String::new());
                Token::Int(literal.parse().unwrap()) // parse failure should be impossible, so unwrap
            },
            State::ReadingString => Token::String(replace(&mut self.buffer, String::new())),
            State::ReadingWord => word_token(replace(&mut self.buffer, String::new())),
            _ => unreachable!("add_token called on non-reading state (scanner)"),
        };
        self.tokens.push(new_token);
    }
}

/// Helper. Accepts characters 0 ... 9 as integral.
fn is_integral(c: char) -> bool {
    match c {
        '0' ... '9' => true,
        _ => false,
    }
}

/// Helper. Accepts characters 0 ... 9 as numeric, and a...z, A...Z as alphabetic.
fn is_alphanumeric(c: char) -> bool {
    match c {
        _ if is_integral(c) => true,
        'a'...'z' | 'A'...'Z' => true,
        '_' => true,
        _ => false,
    }
}

/// Helper. Characters that could be part of a single character token, or a different longer token
/// are considered unclear. They are ":", "/", ".". For example : could be either a type-declaration
/// token, or the beginning of an assignment token (:=).
fn is_unclear(c: char) -> bool {
    match c {
        // * not included, it is a special case for comment
        ':' | '/' | '.'  => true,
        _ => false,
    }
}

/// Helper. Return true if the character is an operator in MiniPl.
fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '&' | '!' | '=' | '<' => true,
        _ => false,
    }
}

/// Check if reading a longer token should end. For example if the scanner has been reading a string
/// and encounters a (unescaped) quote (") return true.
fn read_end(state: &State, c: char) -> bool {
    match *state {
        State::ReadingInt => !is_integral(c),
        State::ReadingString => c == '"',
        State::ReadingWord => !is_alphanumeric(c),
        _ => false, // calling this is useless on other states
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
