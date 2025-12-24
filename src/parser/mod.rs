use crate::parser::ast::{AST, StatementKind};
use crate::parser::lexer::{Lexer, LexerError};
use crate::parser::token::{KeywordKind, TokenKind, TokenKind::Keyword};
use std::cell::RefCell;
use std::rc::Rc;

pub mod ast;
mod lexer;
pub mod token;

pub struct Parser<'a> {
    lexer: Rc<RefCell<Lexer<'a>>>,
}

pub struct ParseError {
    pub pos: usize,
    pub message: String,
}

impl ParseError {
    fn new(message: String, pos: usize) -> Self {
        ParseError { message, pos }
    }
}

impl Into<String> for ParseError {
    fn into(self) -> String {
        format!("ERROR[Position:{0}]: '{1}'", self.pos, self.message)
    }
}

impl From<LexerError> for ParseError {
    fn from(err: LexerError) -> Self {
        ParseError {
            message: err.message,
            pos: err.pos,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Parser {
            lexer: Rc::new(RefCell::new(Lexer::new(data))),
        }
    }

    pub fn parse(&mut self) -> Result<AST<'a>, ParseError> {
        let mut ast = AST::new();
        loop {
            match self.parse_stmt() {
                Ok(Some(stmt)) => ast.append_stmt(stmt),
                Ok(None) => break,
                Err(err) => return Err(err),
            }
        }
        Ok(ast)
    }

    fn parse_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        let mut l = self.lexer.borrow_mut();
        let next = l.next();

        match next {
            Ok(token) => match token.kind.clone() {
                Keyword(kw) => match kw {
                    KeywordKind::Commit => self.parse_commit_stmt(),
                    KeywordKind::Create => self.parse_create_stmt(),
                    KeywordKind::Delete => self.parse_delete_stmt(),
                    KeywordKind::Select => self.parse_select_stmt(),
                    KeywordKind::Update => self.parse_update_stmt(),
                    _ => Err(ParseError::new(
                        format!("Unexpected keyword token: {0}", token.kind),
                        token.pos,
                    )),
                },
                TokenKind::Eof => Ok(None),
                _ => Err(ParseError::new(format!("Unexpected token: {0}", token.kind), token.pos)),
            },
            Err(err) => Err(ParseError::from(err)),
        }
    }

    fn parse_select_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }

    fn parse_commit_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }

    fn parse_delete_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }

    fn parse_update_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }

    fn parse_create_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }
}
