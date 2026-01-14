use super::utils::Span;

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub location: TokenLocation
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl TokenLocation {
    pub fn new(span: &Span) -> Self {
        Self {
            start: span.location_offset(),
            end: span.location_offset() + span.fragment().len(),
            line: span.location_line() as usize,
            column: span.get_utf8_column(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind<'a> {
    Keyword(KeywordKind),
    Ident(&'a str),
    Literal(&'a str),  // Optional extension with literal kind
    Symbol(SymbolKind),
    Op(OperationKind),
    Comp(ComparisonKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Fn,
    While,
    Set,
    If,
    Else,
    Return,
    Loop,
    Call,
    Print,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    LineBreak,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonKind {
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LessThan,
    GreaterThan,
}
