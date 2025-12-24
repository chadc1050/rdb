use crate::parser::token::*;
use std::cell::Cell;

pub struct Lexer<'a> {
    data: &'a [u8],
    cursor: Cell<usize>,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub pos: usize,
}

impl LexerError {
    fn new(message: String, pos: usize) -> Self {
        LexerError {
            message,
            pos: pos.into(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            cursor: Cell::new(0),
        }
    }

    pub fn next(&self) -> Result<Token<'a>, LexerError> {
        let pos = self.cursor.get();
        if pos >= self.data.len() {
            return Ok(Token::new(TokenKind::Eof, pos));
        }

        let peek = self.data[pos];

        match peek {
            b'\'' => Ok(Token::new(self.lex_string_literal(), pos)),
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Token::new(self.lex_identifier_or_kw(), pos)),
            b'0'..=b'9' => Ok(Token::new(self.lex_numerical_literal(), pos)),
            _ => self.lex_single_chars(),
        }
    }

    pub fn peek(&self) -> Result<Token<'a>, LexerError> {
        let start = self.cursor.get();
        let next = self.next();
        self.cursor.set(start);
        next
    }

    pub fn expect(&self, expected: TokenKind) -> Result<(), LexerError> {
        let pos = self.cursor.get();
        match self.next() {
            Ok(t) => {
                if t.kind == expected {
                    Ok(())
                } else {
                    Err(LexerError::new(format!("Unexpected token: {0}", t.kind), pos))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn lex_single_chars(&self) -> Result<Token<'a>, LexerError> {
        let pos = self.cursor.get();
        let take = self.data[pos];
        self.cursor.set(pos + 1);

        match take {
            b'\t' => Ok(Token::new(TokenKind::Whitespace(WhitespaceKind::HorizontalTab), pos)),
            b' ' => Ok(Token::new(TokenKind::Whitespace(WhitespaceKind::Space), pos)),
            b'\n' => Ok(Token::new(TokenKind::LineTerminator(LineTerminatorKind::LineFeed), pos)),
            b'\r' => Ok(Token::new(TokenKind::LineTerminator(LineTerminatorKind::CarridgeReturn), pos)),
            b'(' => Ok(Token::new(TokenKind::Punc(PuncKind::LParen), pos)),
            b')' => Ok(Token::new(TokenKind::Punc(PuncKind::RParen), pos)),
            b'[' => Ok(Token::new(TokenKind::Punc(PuncKind::LBracket), pos)),
            b']' => Ok(Token::new(TokenKind::Punc(PuncKind::RBracket), pos)),
            b',' => Ok(Token::new(TokenKind::Punc(PuncKind::Comma), pos)),
            b';' => Ok(Token::new(TokenKind::Punc(PuncKind::SemiColon), pos)),
            b':' => Ok(Token::new(TokenKind::Punc(PuncKind::Colon), pos)),
            b'=' => Ok(Token::new(TokenKind::Punc(PuncKind::Equal), pos)),
            _ => Err(LexerError {
                message: "unknown artifact".to_string(),
                pos: pos.into(),
            }),
        }
    }

    fn lex_string_literal(&self) -> TokenKind<'a> {
        let start = self.cursor.get() + 1;
        let mut pos = start;
        while self.data[pos] != b'\'' || self.data[pos - 1] == b'\\' {
            pos += 1;
        }

        let slice = &self.data[start..pos];
        pos += 1;

        self.cursor.set(pos);

        let word = std::str::from_utf8(slice).expect("word should be utf8");
        TokenKind::Literal(LiteralKind::String(word))
    }

    fn lex_identifier_or_kw(&self) -> TokenKind<'a> {
        let start = self.cursor.get();
        let mut pos = start;
        while pos < self.data.len() && self.data[pos].is_ascii_alphabetic() {
            pos += 1;
        }

        self.cursor.set(pos);

        let slice = &self.data[start..pos];
        let word = std::str::from_utf8(slice).expect("word should be utf-8");

        match match_kw(word) {
            Some(kw) => TokenKind::Keyword(kw),
            None => TokenKind::Identifier(word),
        }
    }

    fn lex_numerical_literal(&self) -> TokenKind<'a> {
        todo!();
    }
}

fn match_data(word: &str) -> Option<DataKind> {
    match word.to_lowercase().as_str() {
        _ => None,
    }
}

fn match_kw(word: &str) -> Option<KeywordKind> {
    match word.to_lowercase().as_str() {
        "add" => Some(KeywordKind::Add),
        "all" => Some(KeywordKind::All),
        "alter" => Some(KeywordKind::Alter),
        "and" => Some(KeywordKind::And),
        "any" => Some(KeywordKind::Any),
        "as" => Some(KeywordKind::As),
        "asc" => Some(KeywordKind::Asc),
        "backup" => Some(KeywordKind::Backup),
        "begin" => Some(KeywordKind::Begin),
        "by" => Some(KeywordKind::By),
        "case" => Some(KeywordKind::Case),
        "check" => Some(KeywordKind::Check),
        "column" => Some(KeywordKind::Column),
        "commit" => Some(KeywordKind::Commit),
        "constraint" => Some(KeywordKind::Constraint),
        "create" => Some(KeywordKind::Create),
        "database" => Some(KeywordKind::Database),
        "default" => Some(KeywordKind::Default),
        "delete" => Some(KeywordKind::Delete),
        "desc" => Some(KeywordKind::Desc),
        "distinct" => Some(KeywordKind::Distinct),
        "drop" => Some(KeywordKind::Drop),
        "else" => Some(KeywordKind::Else),
        "exec" => Some(KeywordKind::Exec),
        "exists" => Some(KeywordKind::Exists),
        "foreign" => Some(KeywordKind::Foreign),
        "from" => Some(KeywordKind::From),
        "full" => Some(KeywordKind::Full),
        "group" => Some(KeywordKind::Group),
        "having" => Some(KeywordKind::Having),
        "if" => Some(KeywordKind::If),
        "in" => Some(KeywordKind::In),
        "into" => Some(KeywordKind::Into),
        "index" => Some(KeywordKind::Index),
        "inner" => Some(KeywordKind::Inner),
        "insert" => Some(KeywordKind::Insert),
        "is" => Some(KeywordKind::Is),
        "join" => Some(KeywordKind::Join),
        "key" => Some(KeywordKind::Key),
        "left" => Some(KeywordKind::Left),
        "like" => Some(KeywordKind::Like),
        "limit" => Some(KeywordKind::Limit),
        "not" => Some(KeywordKind::Not),
        "null" => Some(KeywordKind::Null),
        "or" => Some(KeywordKind::Or),
        "order" => Some(KeywordKind::Order),
        "outer" => Some(KeywordKind::Outer),
        "primary" => Some(KeywordKind::Primary),
        "procedure" => Some(KeywordKind::Procedure),
        "right" => Some(KeywordKind::Right),
        "rownum" => Some(KeywordKind::Rownum),
        "select" => Some(KeywordKind::Select),
        "set" => Some(KeywordKind::Set),
        "some" => Some(KeywordKind::Some),
        "table" => Some(KeywordKind::Table),
        "then" => Some(KeywordKind::Then),
        "top" => Some(KeywordKind::Top),
        "transaction" => Some(KeywordKind::Transaction),
        "truncate" => Some(KeywordKind::Truncate),
        "union" => Some(KeywordKind::Union),
        "unique" => Some(KeywordKind::Unique),
        "update" => Some(KeywordKind::Update),
        "values" => Some(KeywordKind::Values),
        "view" => Some(KeywordKind::View),
        "when" => Some(KeywordKind::When),
        "where" => Some(KeywordKind::Where),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    use crate::parser::token::*;

    #[test]
    fn test_punc() {
        let l = Lexer::new(b",:;(][)");

        assert_eq!(TokenKind::Punc(PuncKind::Comma), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::Colon), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::LParen), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::RBracket), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::LBracket), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::RParen), l.next().unwrap().kind);
        assert_eq!(TokenKind::Eof, l.next().unwrap().kind);
    }

    #[test]
    fn test_kw() {
        let l = Lexer::new(b"select weight from dog;");

        assert_eq!(TokenKind::Keyword(KeywordKind::Select), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Identifier("weight"), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::From), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Identifier("dog"), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), l.next().unwrap().kind);
        assert_eq!(TokenKind::Eof, l.next().unwrap().kind);
    }

    #[test]
    fn test_string_literal() {
        let l = Lexer::new(b"UPDATE dog SET color = 'golden' WHERE breed = 'golden retriever';");

        assert_eq!(TokenKind::Keyword(KeywordKind::Update), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Identifier("dog"), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::Set), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Identifier("color"), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::Equal), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Literal(LiteralKind::String("golden")), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::Where), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Identifier("breed"), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(TokenKind::Punc(PuncKind::Equal), l.next().unwrap().kind);
        assert_eq!(TokenKind::Whitespace(WhitespaceKind::Space), l.next().unwrap().kind);
        assert_eq!(
            TokenKind::Literal(LiteralKind::String("golden retriever")),
            l.next().unwrap().kind
        );
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), l.next().unwrap().kind);
        assert_eq!(TokenKind::Eof, l.next().unwrap().kind);
    }
}
