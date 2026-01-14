use nom::{branch::alt, bytes::complete::tag, combinator::map, error::Error, Parser};

pub mod token;

use token::Token;

fn symbol_parser<'a>() -> impl Parser<&'a str, Output = Token<'a>, Error = Error<&'a str>> {
    map(alt((tag(";"), tag("("), tag(")"))), |lexeme: &'a str| {
        Token::Symbol(match lexeme {
            ";" => token::SymbolKind::LineBreak,
            "(" => token::SymbolKind::LeftParen,
            ")" => token::SymbolKind::RightParen,
            _ => unreachable!(),
        })
    })
}

#[test]
fn test_symbol_parser() {
    assert_eq!(
        symbol_parser().parse(";"),
        Ok(("", Token::Symbol(token::SymbolKind::LineBreak)))
    );

    assert_eq!(
        symbol_parser().parse("(;"),
        Ok((";", Token::Symbol(token::SymbolKind::LeftParen)))
    );

    assert_eq!(
        symbol_parser().parse(")"),
        Ok(("", Token::Symbol(token::SymbolKind::RightParen)))
    )
}
