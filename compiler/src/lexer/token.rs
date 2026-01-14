#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Keyword(KeywordKind),
    Ident(&'a str),
    Number(&'a str),
    Symbol(SymbolKind),
    Op(OperationKind),
    Comp(ComparisonKind),
}

#[derive(Debug, PartialEq, Clone)]
pub enum KeywordKind {
    Let,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolKind {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LineBreak,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonKind {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
