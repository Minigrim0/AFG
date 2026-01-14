use nom::{branch::alt, bytes::complete::tag, combinator::map, error::Error, Parser};

pub mod token;
mod utils;

use token::{TokenKind, Token, TokenLocation};
use utils::Span;

#[cfg(test)]
mod tests;

fn parse_symbols<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(alt((tag(";"), tag("("), tag(")"), tag("["), tag("]"), tag("{"), tag("}"))), |span: Span| {
        Token {
            kind: TokenKind::Symbol(match *span.fragment() {
                ";" => token::SymbolKind::LineBreak,
                "(" => token::SymbolKind::LeftParen,
                ")" => token::SymbolKind::RightParen,
                "[" => token::SymbolKind::LeftBracket,
                "]" => token::SymbolKind::RightBracket,
                "{" => token::SymbolKind::LeftBrace,
                "}" => token::SymbolKind::RightBrace,
                _ => unreachable!(),
            }),
            location: TokenLocation::new(&span)

        }
    })
}

fn parse_keywords<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(
        alt((
            tag("fn"),
            tag("while"),
            tag("set"),
            tag("if"),
            tag("else"),
            tag("return"),
            tag("loop"),
            tag("call"),
            tag("print"),
        )),
        |span: Span| {
            Token {
                kind: TokenKind::Keyword(match *span.fragment() {
                    "fn" => token::KeywordKind::Fn,
                    "while" => token::KeywordKind::While,
                    "set" => token::KeywordKind::Set,
                    "if" => token::KeywordKind::If,
                    "else" => token::KeywordKind::Else,
                    "return" => token::KeywordKind::Return,
                    "loop" => token::KeywordKind::Loop,
                    "call" => token::KeywordKind::Call,
                    "print" => token::KeywordKind::Print,
                    _ => unreachable!(),
                }),
                location: TokenLocation::new(&span)
            }
        },
    )
}

/// Parses operators consisting of two distinct characters (e.g. "==")
fn parse_operators<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(
        alt(
            (
                tag(">="),
                tag("<="),
                tag("=="),
                tag("!="),
                tag("<"),
                tag(">")
            )
        ),
        |span: Span| {
            Token {
                kind: TokenKind::Comp(match *span.fragment() {
                    "==" => token::ComparisonKind::Equal,
                    "!=" => token::ComparisonKind::NotEqual,
                    "<=" => token::ComparisonKind::LessThanOrEqual,
                    ">=" => token::ComparisonKind::GreaterThanOrEqual,
                    "<" => token::ComparisonKind::LessThan,
                    ">" => token::ComparisonKind::GreaterThan,
                    _ => unreachable!()
                }),
                location: TokenLocation::new(&span)
            }
        }
    )
}