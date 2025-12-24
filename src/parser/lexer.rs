use crate::parser::token::*;

pub struct Lexer<'a> {
    data: &'a [u8],
    cursor: usize,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn next(&mut self) -> Result<Token<'a>, LexerError> {
        if self.cursor >= self.data.len() {
            return Ok(Token::new(TokenKind::Eof, self.cursor));
        }

        let pos = self.cursor;
        let peek = self.data[self.cursor];

        match peek {
            b'\'' => Ok(Token::new(self.lex_string_literal(), pos)),
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Token::new(self.lex_identifier_or_kw(), pos)),
            b'0'..=b'9' => Ok(Token::new(self.lex_numerical_literal(), pos)),
            _ => self.lex_single_chars(),
        }
    }

    fn lex_single_chars(&mut self) -> Result<Token<'a>, LexerError> {
        let pos = self.cursor;
        let take = self.data[self.cursor];
        self.cursor += 1;

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
                pos,
            }),
        }
    }

    fn lex_string_literal(&mut self) -> TokenKind<'a> {
        self.cursor += 1;
        let start = self.cursor;
        while self.data[self.cursor] != b'\'' || self.data[self.cursor - 1] == b'\\' {
            self.cursor += 1;
        }

        let slice = &self.data[start..self.cursor];
        self.cursor += 1;

        let word = std::str::from_utf8(slice).expect("word should be utf8");
        TokenKind::Literal(LiteralKind::String(word))
    }

    fn lex_identifier_or_kw(&mut self) -> TokenKind<'a> {
        let start = self.cursor;
        while self.cursor < self.data.len() && self.data[self.cursor].is_ascii_alphabetic() {
            self.cursor += 1;
        }

        let slice = &self.data[start..self.cursor];
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
        let mut l = Lexer::new(b",:;(][)");

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
        let mut l = Lexer::new(b"select weight from dog;");

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
        let mut l = Lexer::new(b"UPDATE dog SET color = 'golden' WHERE breed = 'golden retriever';");

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
