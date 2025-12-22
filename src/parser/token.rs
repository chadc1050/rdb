#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub pos: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, pos: usize) -> Self {
        Token { kind, pos }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind<'a> {
    Identifier(&'a str),
    Data(DataKind),
    Keyword(KeywordKind),
    Literal(LiteralKind<'a>),
    Punc(PuncKind),
    Comment(CommentKind<'a>),
    LineTerminator(LineTerminatorKind),
    Whitespace(WhitespaceKind),
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataKind {
    Char(Option<u8>),
    VarChar(Option<u16>),
    Binary(Option<u16>),
    VarBinary(Option<u16>),
    TinyBlob,
    TinyText,
    Text(Option<u16>),
    Blob(Option<u16>),
    MediumText(Option<u32>),
    MediumBlob(Option<u32>),
    LongText(Option<u64>),
    LongBlob(Option<u64>),
    Bit(Option<u8>),
    TinyInt(Option<u8>),
    Bool,
    SmallInt(Option<u8>),
    MediumInt(Option<u8>),
    Integer(Option<u8>),
    BigInt(Option<u8>),
    Float(Option<u8>, Option<u8>),
    Double(Option<u8>, Option<u8>),
    Decimal(Option<u8>, Option<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind<'a> {
    String(&'a str),
    Numeric(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeywordKind {
    Add,
    All,
    Alter,
    And,
    Any,
    As,
    Asc,
    Backup,
    Begin,
    By,
    Case,
    Check,
    Column,
    Commit,
    Constraint,
    Create,
    Database,
    Default,
    Delete,
    Desc,
    Distinct,
    Drop,
    Else,
    Exec,
    Exists,
    Foreign,
    From,
    Full,
    Group,
    Having,
    If,
    In,
    Into,
    Index,
    Inner,
    Insert,
    Is,
    Join,
    Key,
    Left,
    Like,
    Limit,
    Not,
    Null,
    Or,
    Order,
    Outer,
    Primary,
    Procedure,
    Right,
    Rownum,
    Select,
    Set,
    Some,
    Table,
    Then,
    Top,
    Transaction,
    Truncate,
    Union,
    Unique,
    Update,
    Values,
    View,
    When,
    Where,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PuncKind {
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBracket,
    /// ]
    RBracket,
    /// ,
    Comma,
    /// ;
    SemiColon,
    /// :
    Colon,
    /// *
    Star,
    /// *=
    MultiplyAssign,
    /// .
    Period,
    /// +
    Add,
    /// +=
    AddAssign,
    /// -
    Subtract,
    /// -=
    SubtractAssign,
    /// /
    Divide,
    /// /=
    DivideAssign,
    /// <
    LessThan,
    /// <=
    LessThanEq,
    /// >
    GreaterThan,
    /// >=
    GreaterThanEq,
    /// =
    Equal,
    /// <>
    NotEqual,
    /// %
    Modulo,
    /// %=
    ModuloAssign,
    /// &
    BitwiseAnd,
    /// &=,
    BitwiseAndAssign,
    /// |
    BitwiseOr,
    /// |=
    BitwiseOrAssign,
    /// ^
    BitwiseXor,
    /// ^=
    BitwiseXorAssign,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommentKind<'a> {
    /// --
    Single(&'a str),
    /// /* * */
    Multi(&'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineTerminatorKind {
    /// \n
    LineFeed,
    /// \r
    CarridgeReturn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WhitespaceKind {
    ///  
    Space,
    /// \t
    HorizontalTab,
}
