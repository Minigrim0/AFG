use nom::Parser;

use crate::lexer::comments_parser;

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


#[test]
fn test_comment_parser() {
    // Comment should be removed entirely
    assert_eq!(
        comments_parser().parse("// Hello world\n".into())
            .map(|t| t.0.len()),
        Ok(0)
    )
}

#[test]
fn test_lexer() {
    let source = "fn main {\n\tprint $Velocity;\n}";
    let lex_result = super::parse_source(source);

    assert!(lex_result.is_ok(), "Lexer encountered errors: {:?}", lex_result.errors);
    assert_eq!(lex_result.tokens.len(), 7); // fn, main, {, print, $Velocity, ;, }


    assert_eq!(lex_result.tokens[0].kind, TokenKind::Keyword(token::KeywordKind::Fn));
    assert_eq!(lex_result.tokens[1].kind, TokenKind::Ident("main"));
    assert_eq!(lex_result.tokens[2].kind, TokenKind::Symbol(token::SymbolKind::LeftBrace));
    assert_eq!(lex_result.tokens[3].kind, TokenKind::Keyword(token::KeywordKind::Print));
    assert_eq!(lex_result.tokens[4].kind, TokenKind::Ident("$Velocity"));
    assert_eq!(lex_result.tokens[5].kind, TokenKind::Symbol(token::SymbolKind::LineBreak));
    assert_eq!(lex_result.tokens[6].kind, TokenKind::Symbol(token::SymbolKind::RightBrace));
}