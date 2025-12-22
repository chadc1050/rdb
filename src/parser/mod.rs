use crate::parser::lexer::Lexer;

mod lexer;
pub mod token;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

pub struct AST;

pub struct ParseError;

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Parser {
            lexer: Lexer::new(data),
        }
    }

    pub fn parse(&self) -> Result<&'a AST, ParseError> {
        todo!();
    }
}
