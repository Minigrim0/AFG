use nom::Parser;

use super::{parse_keywords, parse_symbols};
use super::token::{self, TokenKind};

#[test]
fn test_symbol_parser() {
    assert_eq!(
        parse_symbols().parse(";".into()).map(|t| t.1.kind),
        Ok(TokenKind::Symbol(token::SymbolKind::LineBreak))
    );
}

#[test]
fn test_keyword_parser() {
    assert_eq!(
        parse_keywords().parse("fn main".into()).map(|t| t.1.kind),
        Ok(TokenKind::Keyword(token::KeywordKind::Fn))
    );
}