use crate::parser::token::*;

pub struct Lexer<'a> {
    data: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        if self.cursor >= self.data.len() {
            return Some(Token::new(TokenKind::Eof, self.cursor));
        }

        let pos = self.cursor;
        let peek = self.data[self.cursor];

        match peek {
            b'a'..=b'z' | b'A'..=b'Z' => {
                return Some(Token::new(self.lex_identifier_or_kw(), pos));
            }
            b'0'..=b'9' => return Some(Token::new(self.lex_numerical_literal(), pos)),
            _ => return self.lex_other(),
        };
    }

    fn lex_other(&mut self) -> Option<Token<'a>> {
        let pos = self.cursor;
        let take = self.data[self.cursor];
        self.cursor += 1;

        match take {
            b'(' => Some(Token::new(TokenKind::Punc(PuncKind::LParen), pos)),
            b')' => Some(Token::new(TokenKind::Punc(PuncKind::RParen), pos)),
            b'[' => Some(Token::new(TokenKind::Punc(PuncKind::LBracket), pos)),
            b']' => Some(Token::new(TokenKind::Punc(PuncKind::RBracket), pos)),
            b',' => Some(Token::new(TokenKind::Punc(PuncKind::Comma), pos)),
            b';' => Some(Token::new(TokenKind::Punc(PuncKind::SemiColon), pos)),
            b':' => Some(Token::new(TokenKind::Punc(PuncKind::Colon), pos)),
            _ => None,
        }
    }

    fn lex_identifier_or_kw(&mut self) -> TokenKind<'a> {
        let start = self.cursor;
        while self.data[self.cursor].is_ascii_alphabetic() {
            self.cursor += 1;
        }

        let slice = &self.data[start..self.cursor];
        let word = std::str::from_utf8(slice).expect("word should be utf-8");

        return match match_kw(word) {
            Some(kw) => TokenKind::Keyword(kw),
            None => TokenKind::Identifier(word),
        };
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
    }
}
