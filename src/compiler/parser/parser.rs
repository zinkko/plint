// HELPERS

use super::ast::*;
use super::super::scanner::token::*;

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

    fn assume_next(&mut self, expected: Token) -> Result<(), String> {
        match self.next() {
            Some(ref token) if *token == expected => Ok(()),
            Some(wrong) => Err(format!("Syntax error: Expected {:?}, got {:?}", expected, wrong)),
            _ => Err(format!("Reached end while parsing! Expected {:?}", expected)),
        }
    }

    fn assume_end(&mut self) -> Result<(), String> {
        self.assume_next(Token::EndStatement)
    }

    /// Returns next token. Automatically takes the token from the internal buffer
    /// or the iterator, whichever is appropriate. Use this instead of self.expect_next() when
    /// not requiring another token.
    fn next(&mut self) -> Option<Token> {
        if self.buffer == None {
            self.iterator.next()
        } else {
            let mut next = None;
            mem::swap(&mut self.buffer, &mut next);
            next
        }
    }

    /// Returns next token, or an error if there is no next token. Use this instead of self.next()
    /// when you are expecting another token.
    fn expect_next(&mut self) -> Result<Token, String> {
        match self.next() {
            Some(token) => Ok(token),
            None => Err("Reached end while parsing".to_string()),
        }
    }

    // DIFFERENT STATEMENTS

    fn parse_read(&mut self) -> Result<Statement, String> {
        // "read" <identifier> read identifier and return
        let stmt_res = match self.next() {
            Some(Token::Identifier(name)) => Ok(Statement::Read(name)),
            Some(token) => Err(format!("Unexpected token {:?}", token)),
            None => Err(format!("Reached end while parsing")),
        };
        stmt_res.and_then(|statement| self.assume_end().and(Ok(statement)))
    }

    fn parse_assert(&mut self) -> Result<Statement, String> {
        self.assume_next(Token::OpenParen)
            .and(self.parse_expression())
            .and_then(|expr| self.assume_next(Token::CloseParen)
                .and(self.assume_end())
                .and(Ok(Statement::Assert(expr)))
            )
    }

    fn parse_assignment(&mut self, identifier: String) -> Result<Statement, String> {
        self.assume_next(Token::Assignment)
            .and(self.parse_expression())
            .and_then(|expression| self.assume_end()
                .and(Ok(Statement::Assignment { identifier, expression }))
            )
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        // for <iden> in <expr> .. <expr> do <stmts> end for
        let res_identifier = match self.next() {
            Some(Token::Identifier(value)) => Ok(value),
            Some(wrong) => Err(format!("Bad token {:?}", wrong)),
            _ => Err("Reached end while parsing".to_string()),
        }.and_then(|identifier|
            self.assume_next(Token::Reserved(Keyword::In)).and(Ok(identifier))
        );

        let identifier = res_identifier?;
        let begin = self.parse_expression().and_then(
            |expr| self.assume_next(Token::Range).and(Ok(expr))
        )?;
        let end = self.parse_expression().and_then(
            |expr| self.assume_next(Token::Reserved(Keyword::Do)).and(Ok(expr))
        )?;


        let mut stmt_results = Vec::new();

        loop {
            match self.next() {
                Some(Token::Reserved(Keyword::End)) => break,
                Some(token) => stmt_results.push(self.parse_statement(token)),
                None => return Err("Reached end while parsing".to_string()),
            }
        }

        let res_stmts : Result<_, _> = stmt_results.iter().cloned().collect();
        let statements = res_stmts?;

        self.assume_next(Token::Reserved(Keyword::For))
            .and(self.assume_end())?;
        Ok(Statement::For { identifier, begin, end, statements })
    }

    fn parse_declaration(&mut self) -> Result<Statement, String> {
        let identifier = match self.expect_next()? {
            Token::Identifier(value) => Ok(value),
            token => Err(format!("Wrong token {:?}", token)),
        }?;
        self.assume_next(Token::TypeDecl)?;

        let mpl_type = match self.next() {
            Some(Token::Reserved(word)) => match word {
                Keyword::Int => Ok(MplType::Int),
                Keyword::String => Ok(MplType::String),
                Keyword::Bool => Ok(MplType::Bool),
                _ => Err(format!("Not a type {:?}", word)),
            },
            Some(token) => Err(format!("bad token {:?}", token)),
            None => Err("Reached end while parsing".to_string()),
        }?;

        let value = match self.next() {
            Some(Token::EndStatement) => Ok(None),
            Some(Token::Assignment) => self.parse_expression()
                .and_then(|expr| self.assume_end().and(Ok(Some(expr)))),
            Some(token) => Err(format!("Bad token {:?}", token)),
            None => Err(format!("Reached end while parsing")),
        }?;

        Ok(Statement::Declaration { identifier, mpl_type, value })
    }

    fn parse_statement(&mut self, token: Token) -> Result<Statement, String> {
        match token {
            Token::Reserved(Keyword::Var) => self.parse_declaration(), // Declaration
            Token::Identifier(ident) => self.parse_assignment(ident), // Assignment
            Token::Reserved(Keyword::For) => self.parse_for(),
            Token::Reserved(Keyword::Read) => self.parse_read(),
            Token::Reserved(Keyword::Print) => self.parse_expression()
                .and_then(|expr| self.assume_end().map(|_| expr))
                .map(Statement::Print),
            Token::Reserved(Keyword::Assert) => self.parse_assert(),
            Token::EndStatement => Ok(Statement::Empty),
            _ => Err(format!("Bad token! {:?}", token)),
        }
    }

    // Expression and Operand

    fn parse_expression(&mut self) -> Result<Expression, String> {
        match self.next() {
            Some(Token::Operator(operator)) => self.expect_next()
                .and_then(|token| self.parse_operand(token))
                .map(|operand| Expression::Unary { operator, operand }),
            Some(token) => {
                self.parse_operand(token).and_then(|left| {
                    match self.next() {
                        Some(Token::Operator(operator)) => {
                            self.expect_next()
                                .and_then(|token| self.parse_operand(token))
                                .map(|right| Expression::Binary { left, operator, right })
                        },
                        Some(token) => {
                            self.buffer = Some(token);
                            Ok(Expression::Simple(left))
                        },
                        None => Err("Reached end while parsing".to_string()),
                    }
                })
            },
            None => Err("Reached end while parsing".to_string()),
        }
    }

    fn parse_operand(&mut self, token: Token) -> Result<Operand, String> {
        // TODO refactor!
        match token {
            Token::Int(i) => Ok(Operand::Int(i)),
            Token::String(s) => Ok(Operand::String(s)),
            Token::Identifier(id) => Ok(Operand::Identifier(id)),
            Token::OpenParen => self.parse_expression()
                .and_then(|expr| self.assume_next(Token::CloseParen)
                    .and(Ok(Operand::Expr(Box::new(expr))))
                ),
            _ => Err(format!("Bad token {:?}", token)),
        }
    }

    /// Extract the AST from the parser. If parsing is not complete, return an error.
    pub fn into_ast(mut self) -> Result<Ast, String> {
        let mut statements = Vec::new();

        loop {
            match self.next() {
                Some(token) => match self.parse_statement(token) {
                    Ok(stmt) => statements.push(stmt),
                    Err(e) => return Err(e),
                },
                None => break,
            };
        };

        Ok(Ast { statements })
    }
}
