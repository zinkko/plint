// HELPERS

use super::ast::*;
use super::super::lexer::token::*;

use std::vec::IntoIter;
use std::mem;

pub struct Parser<I: Iterator<Item = Token>> {
    buffer: Option<Token>,
    iterator: I,
}

impl Parser<IntoIter<Token>> {
    pub fn new(tokens: Vec<Token>) -> Parser<IntoIter<Token>> {
        Parser { buffer: None, iterator: tokens.into_iter() }
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {

    fn assume_next(&mut self, expected: Token) {
        let next = self.next();
        if let Some(token) = next {
            if token != expected {
                panic!("Syntax error: Expected {:?}, got {:?}", expected, token);
            }
        } else {
            panic!("Reached end while parsing! Expected {:?}", expected);
        }
    }

    fn assume_end(&mut self) {
        self.assume_next(Token::EndStatement);
    }

    fn next(&mut self) -> Option<Token> {
        if self.buffer == None {
            self.iterator.next()
        } else {
            let mut next = None;
            mem::swap(&mut self.buffer, &mut next);
            next
        }
    }

    // DIFFERENT STATEMENTS

    fn parse_read(&mut self) -> Statement {
        // "read" <identifier> read identifier and return
        let statement = match self.next() {
            Some(Token::Identifier(name)) => Statement::Read(name),
            Some(token) => panic!("Unexpected token {:?}", token),
            None => panic!("Reached end while parsing"),
        };

        self.assume_end();
        statement
    }

    fn parse_assert(&mut self) -> Statement {
        // "assert" ( <expr> )
        self.assume_next(Token::OpenParen);
        let expression = self.parse_expression();
        self.assume_next(Token::CloseParen);

        self.assume_end();
        Statement::Assert(expression)
    }

    fn parse_assignment(&mut self, identifier: String) -> Statement {
        self.assume_next(Token::Assignment);
        let expression = self.parse_expression();

        self.assume_end();
        Statement::Assignment { identifier, expression }
    }

    fn parse_for(&mut self) -> Statement {
        // for <iden> in <expr> .. <expr> do <stmts> end for
        let identifier = match self.next() {
            Some(Token::Identifier(value)) => value,
            _ => panic!("Syntax error"),
        };
        self.assume_next(Token::Reserved(Keyword::In));
        let begin = self.parse_expression();
        self.assume_next(Token::Range);
        let end = self.parse_expression();
        self.assume_next(Token::Reserved(Keyword::Do));

        let mut statements = Vec::new();

        loop {
            match self.next() {
                Some(Token::Reserved(Keyword::End)) => break,
                Some(token) => statements.push(self.parse_statement(token)),
                None => panic!("Reached end while parsing"),
            }
        }

        self.assume_next(Token::Reserved(Keyword::For));
        Statement::For { identifier, begin, end, statements }
    }

    fn parse_declaration(&mut self) -> Statement {
        // var <ident> : <type> [ := <expr> ]
        let identifier = match self.next() {
            Some(Token::Identifier(value)) => value,
            Some(token) => panic!("Wrong token {:?}", token),
            None => panic!("Reached end while parsing"),
        };
        self.assume_next(Token::TypeDecl);
        let mpl_type = match self.next() {
            Some(Token::Reserved(word)) => match word {
                Keyword::Int => MplType::Int,
                Keyword::String => MplType::String,
                Keyword::Bool => MplType::Bool,
                _ => panic!("Not a type {:?}", word),
            },
            Some(token) => panic!("bad token {:?}", token),
            None => panic!("Reached end while parsing"),
        };

        let value = match self.next() {
            Some(Token::EndStatement) => None,
            Some(Token::Assignment) => Some(self.parse_expression()),
            Some(token) => panic!("Bad token {:?}", token),
            None => panic!("Reached end while parsing"),
        };

        match value  {
            None => (),
            _ => self.assume_end(),
        };

        Statement::Declaration { identifier, mpl_type, value }
    }

    fn parse_statement(&mut self, token: Token) -> Statement {
        match token {
            Token::Reserved(Keyword::Var) => self.parse_declaration(), // Declaration
            Token::Identifier(ident) => self.parse_assignment(ident), // Assignment
            Token::Reserved(Keyword::For) => self.parse_for(),
            Token::Reserved(Keyword::Read) => self.parse_read(),
            Token::Reserved(Keyword::Print) => Statement::Print(self.parse_expression()),
            Token::Reserved(Keyword::Assert) => self.parse_assert(),
            Token::EndStatement => Statement::Empty,
            _ => panic!("Bad token! {:?}", token),
        }
    }

    // OTHERS

    fn parse_expression(&mut self) -> Expression {
        match self.next() {
            Some(Token::Operator(operator)) => {
                let operand = match self.next() {
                    Some(token) => self.parse_operand(token),
                    None => panic!("Reached end while parsing"),
                };
                Expression::Unary { operator, operand }
            },
            Some(token) => {
                let left = self.parse_operand(token);
                let operator = match self.next() {
                    Some(Token::Operator(o)) => o,
                    Some(token) => {
                        self.buffer = Some(token);
                        return Expression::Simple(left);
                    }
                    None => panic!("Reached end while parsing"),
                };
                let right = match self.next() {
                     Some(token) => self.parse_operand(token),
                     None => panic!("Reached end while parsing"),
                };

                Expression::Binary { left, operator, right }
            }
            None => panic!("Reached end while parsing"),
        }
    }

    fn parse_operand(&mut self, token: Token) -> Operand {
        // TODO refactor!
        match token {
            Token::Int(i) => Operand::Int(i),
            Token::String(s) => Operand::String(s),
            Token::Identifier(id) => Operand::Identifier(id),
            Token::OpenParen => {
                let expression = Box::new(self.parse_expression());
                self.assume_next(Token::CloseParen);
                Operand::Expr(expression)
            },
            _ => panic!("Bad token {:?}", token),
        }
    }

    pub fn into_ast(mut self) -> Ast {
        let mut statements = Vec::new();

        loop {
            match self.next() {
                Some(token) => statements.push(self.parse_statement(token)),
                None => break,
            };
        };

        Ast { statements }
    }
}
