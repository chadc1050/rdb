use crate::parser::ast::{
    AST, DatasetReference, ExprKind, FromClause, FromItemKind, SelectClause, SelectItemKind, SelectStmt, StatementKind,
};
use crate::parser::lexer::{Lexer, LexerError};
use crate::parser::token::{KeywordKind, PuncKind, TokenKind, TokenKind::Keyword};
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

    pub fn parse(&'a mut self) -> Result<AST<'a>, ParseError> {
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

    fn parse_stmt(&'a self) -> Result<Option<StatementKind<'a>>, ParseError> {
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

    fn parse_select_stmt(&'a self) -> Result<Option<StatementKind<'a>>, ParseError> {
        let l = self.lexer.borrow();

        // Select clause
        let select_clause = self.parse_select_clause()?;

        // From clause
        l.expect(TokenKind::Keyword(KeywordKind::From))?;
        let from_clause = self.parse_from_clause()?;

        let select = SelectStmt::new(select_clause, from_clause);

        // Where clause

        // GroupBy clause

        // Having clause

        // OrderBy clause

        // Limit clause

        Ok(Some(StatementKind::Select(select)))
    }

    fn parse_select_clause(&'a self) -> Result<SelectClause<'a>, ParseError> {
        let l = self.lexer.borrow();

        match l.next() {
            Ok(t) => match t.kind {
                TokenKind::Punc(PuncKind::Star) => Ok(SelectClause::all()),
                TokenKind::Punc(PuncKind::LParen) => {
                    let expr = self.parse_select_list();
                    Ok(SelectClause { selected: expr })
                }
                _ => Err(ParseError::new("Unexpected token".to_string(), t.pos)),
            },
            Err(err) => Err(err.into()),
        }
    }

    fn parse_select_list(&'a self) -> Vec<SelectItemKind<'a>> {
        todo!();
    }

    fn parse_from_clause(&'a self) -> Result<FromClause<'a>, ParseError> {
        let l = self.lexer.borrow();

        let mut from_clause = FromClause::new();

        loop {
            match l.next() {
                Ok(t) => match t.kind {
                    TokenKind::Identifier(id) => from_clause.from.push(FromItemKind::Dataset(DatasetReference::new(id))),
                    TokenKind::Keyword(KeywordKind::Join) => {
                        // There will be left, right, inner etc...
                        todo!();
                    }
                    _ => break,
                },
                Err(err) => return Err(err.into()),
            };
        }

        if from_clause.is_empty() {
            return Err(ParseError::new("Missing from clause".to_string(), l.position()));
        }

        Ok(from_clause)
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

    fn parse_list_literal(&self) -> Result<Option<ExprKind>, ParseError> {
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
    fn test_commit() {
        let mut p = Parser::new(b"commit");
        let ast = p.parse();

        let ast = ast.unwrap();
        assert!(ast.stmts.len() == 1);
        assert_eq!(ast.stmts[0], StatementKind::Commit);
    }

    #[test]
    fn test_rollback() {
        let mut p = Parser::new(b"rollback");
        let ast = p.parse();

        let ast = ast.unwrap();
        assert!(ast.stmts.len() == 1);
        assert_eq!(ast.stmts[0], StatementKind::Rollback);
    }

    #[test]
    fn test_select() {
        let mut p = Parser::new(b"SELECT * from cats");
        let ast = p.parse();

        let ast = ast.unwrap();
        assert!(ast.stmts.len() == 1);

        let select = SelectStmt::new(SelectClause::all(), FromClause::table("cats"));

        assert_eq!(ast.stmts[0], StatementKind::Select(select));
    }
}
