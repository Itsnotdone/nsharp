#[derive(Debug, PartialEq)]
pub struct Token {
    token_kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { token_kind: kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Separator(SeparatorKind),
    Operator(OperatorKind),
    Literal(LiteralKind),
    Identifier(String),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Public,
    Private,
    Int,
    Void,
    String,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum SeparatorKind {
    OpenParent,
    CloseParent,
    OpenBracet,
    CloseBracket,
    Semicolon
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    String(String),
    Int(i32),
    Float(f32),
}
