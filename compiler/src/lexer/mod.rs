use nom::{
    Parser, branch::alt, bytes::complete::{tag, take_until}, character::complete::{char, one_of}, combinator::{opt, map, recognize, value}, error::Error, multi::{many0, many1}, sequence::{pair, terminated}
};

pub mod token;
mod utils;

use token::{TokenKind, Token, TokenLocation};
use utils::{Span, LexResult};

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

fn whitespace_parser<'a>() -> impl Parser<Span<'a>, Output = (), Error = Error<Span<'a>>> {
    value(
        (),
        many1(one_of(" \t\r\n"))
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

fn identifier_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    map(
        recognize(
            pair(
                opt(char('$')), 
                terminated(
                    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"),
                    many0(one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"))
                )
            )
        ),
        |lexeme: Span| {
            Token {
                kind: TokenKind::Ident(lexeme.fragment()),
                location: TokenLocation::new(&lexeme)
            }
        }
    )
}

fn token_parser<'a>() -> impl Parser<Span<'a>, Output = Token<'a>, Error = Error<Span<'a>>> {
    alt((
        keywords_parser(),
        comparison_operators_parser(),
        arithmetic_operators_parser(),
        symbols_parser(),
        literals_parser(),
        identifier_parser()
    ))
}

fn skip_ignorable<'a>(input: Span<'a>) -> Span<'a> {
    let mut current_input = input;

    loop {
        let next_input = match whitespace_parser().parse(current_input) {
            Ok(result) => result.0,
            Err(_) => current_input,
        };

        let next_input = match comments_parser().parse(next_input) {
            Ok(result) => result.0,
            Err(_) => next_input,
        };

        if next_input.fragment() == current_input.fragment() {
            break;
        } else {
            current_input = next_input;
        }
    }

    current_input
}

pub fn parse_source<'a>(source: &'a str) -> LexResult<'a> {
    let mut input = Span::new(source);
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    loop {
        input = skip_ignorable(input);

        if input.fragment().is_empty() {
            break;
        }

        match token_parser().parse(input) {
            Ok((remaining, token)) => {
                tokens.push(token);
                input = remaining;
            },
            Err(e) => {
                errors.push(utils::LexerError {
                    message: format!("Failed to parse token: {:?}", e),
                    location: TokenLocation::new(&input),
                });
                
                input = Span::new(&input.fragment()[1..]);
            }
        }
    }

    LexResult {
        tokens,
        errors,
    }
}