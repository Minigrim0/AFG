use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    multi::{many0, many1},
    sequence::terminated,
    character::complete::{char, one_of},
    combinator::{map, value, recognize},
    error::Error
};

pub mod token;
mod utils;

use token::{TokenKind, Token, TokenLocation};
use utils::Span;

#[cfg(test)]
mod tests;

fn symbols_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(alt((tag(";"), tag("("), tag(")"), tag("["), tag("]"), tag("{"), tag("}"))), |lexeme: Span| {
        Token {
            kind: TokenKind::Symbol(match *lexeme.fragment() {
                ";" => token::SymbolKind::LineBreak,
                "(" => token::SymbolKind::LeftParen,
                ")" => token::SymbolKind::RightParen,
                "[" => token::SymbolKind::LeftBracket,
                "]" => token::SymbolKind::RightBracket,
                "{" => token::SymbolKind::LeftBrace,
                "}" => token::SymbolKind::RightBrace,
                _ => unreachable!(),
            }),
            location: TokenLocation::new(&lexeme)

        }
    })
}

fn keywords_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
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
        |lexeme: Span| {
            Token {
                kind: TokenKind::Keyword(match *lexeme.fragment() {
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
                location: TokenLocation::new(&lexeme)
            }
        },
    )
}

/// Parses comparator tokens
fn comparison_operators_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
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
        |lexeme: Span| {
            Token {
                kind: TokenKind::Comp(match *lexeme.fragment() {
                    "==" => token::ComparisonKind::Equal,
                    "!=" => token::ComparisonKind::NotEqual,
                    "<=" => token::ComparisonKind::LessThanOrEqual,
                    ">=" => token::ComparisonKind::GreaterThanOrEqual,
                    "<" => token::ComparisonKind::LessThan,
                    ">" => token::ComparisonKind::GreaterThan,
                    _ => unreachable!()
                }),
                location: TokenLocation::new(&lexeme)
            }
        }
    )
}

fn arithmetic_operators_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(
        alt((tag("+"), tag("-"), tag("*"), tag("/"), tag("%"), tag("="))),
        |lexeme: Span| {
            Token {
                kind: TokenKind::Op(match *lexeme.fragment() {
                    "+" => token::OperationKind::Add,
                    "-" => token::OperationKind::Subtract,
                    "*" => token::OperationKind::Multiply,
                    "/" => token::OperationKind::Divide,
                    "%" => token::OperationKind::Modulo,
                    "=" => token::OperationKind::Assign,
                    _ => unreachable!(),
                }),
                location: TokenLocation::new(&lexeme)
            }
        },
    )
}

/// Remove comments from the source code
fn comments_parser<'a>() -> impl Parser<Span<'a>, Output = (), Error = Error<Span<'a>>> {
    value(
        (),
        (
            tag("//"),
            take_until("\n"),
            tag("\n"),
        )
    )
}

fn literals_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(
        recognize(
            many1(terminated(one_of("0123456789"), many0(char('_'))))
        ),
        |lexeme: Span| {
            Token {
                kind: TokenKind::Literal(lexeme.fragment()),
                location: TokenLocation::new(&lexeme)
            }
        }
    )
}

pub fn parse_source<'a>(source: &'a str) -> Vec<Token<'a>> {

    vec![]
}