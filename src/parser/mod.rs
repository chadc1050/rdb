use crate::parser::ast::{AST, StatementKind};
use crate::parser::lexer::{Lexer, LexerError};
use crate::parser::token::{KeywordKind, LineTerminatorKind, PuncKind, TokenKind, TokenKind::Keyword};
use std::cell::RefCell;
use std::rc::Rc;

pub mod ast;
mod lexer;
pub mod token;

pub struct Parser<'a> {
    lexer: Rc<RefCell<Lexer<'a>>>,
}

#[derive(Clone, Debug)]
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
        let l = self.lexer.borrow();

        match l.next() {
            Ok(token) => match token.kind.clone() {
                Keyword(kw) => match kw {
                    KeywordKind::Commit => self.parse_commit_stmt(),
                    KeywordKind::Create => self.parse_create_stmt(),
                    KeywordKind::Delete => self.parse_delete_stmt(),
                    KeywordKind::Rollback => self.parse_rollback_stmt(),
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
            Err(err) => Err(err.into()),
        }
    }

    fn parse_select_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        todo!();
    }

    fn parse_commit_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        let l = self.lexer.borrow();

        l.eat(TokenKind::Keyword(KeywordKind::Work));
        self.parse_eol()?;

        Ok(Some(StatementKind::Commit))
    }

    fn parse_rollback_stmt(&self) -> Result<Option<StatementKind<'a>>, ParseError> {
        let l = self.lexer.borrow();

        l.eat(TokenKind::Keyword(KeywordKind::Work));
        self.parse_eol()?;

        Ok(Some(StatementKind::Rollback))
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

    fn parse_eol(&self) -> Result<(), ParseError> {
        let l = self.lexer.borrow();

        if l.eat(TokenKind::Punc(PuncKind::SemiColon)) {
            return Ok(());
        }
        match l.peek() {
            Ok(t) => {
                if t.kind == TokenKind::Eof {
                    return Ok(());
                }

                Err(ParseError::new("missing ';'".to_string(), t.pos))
            }
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_commit_rollback() {
        let mut p = Parser::new(b"commit");
        let ast = p.parse();

        assert!(ast.is_ok());
        let ast = ast.unwrap();
        assert!(ast.stmts.len() == 1);
        assert_eq!(ast.stmts[0], StatementKind::Commit);
    }
}
