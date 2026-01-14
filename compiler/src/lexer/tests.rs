use nom::Parser;

use super::{keywords_parser, symbols_parser};
use super::token::{self, TokenKind};

#[test]
fn test_symbol_parser() {
    assert_eq!(
        symbols_parser().parse(";".into()).map(|t| t.1.kind),
        Ok(TokenKind::Symbol(token::SymbolKind::LineBreak))
    );
}

#[test]
fn test_keyword_parser() {
    assert_eq!(
        keywords_parser().parse("fn main".into()).map(|t| t.1.kind),
        Ok(TokenKind::Keyword(token::KeywordKind::Fn))
    );
}