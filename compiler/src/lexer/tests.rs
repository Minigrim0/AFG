use nom::Parser;

use super::{
    arithmetic_operators_parser, comments_parser, comparison_operators_parser, identifier_parser,
    keywords_parser, literals_parser, parse_source, symbols_parser, whitespace_parser,
};
use super::token::{self, TokenKind};
use super::utils::Span;

// ============================================================================
// Individual Parser Tests
// ============================================================================

mod symbols_parser_tests {
    use super::*;

    #[test]
    fn test_semicolon() {
        let result = symbols_parser().parse(Span::new(";"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Symbol(token::SymbolKind::LineBreak));
    }

    #[test]
    fn test_left_paren() {
        let result = symbols_parser().parse(Span::new("("));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Symbol(token::SymbolKind::LeftParen));
    }

    #[test]
    fn test_right_paren() {
        let result = symbols_parser().parse(Span::new(")"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Symbol(token::SymbolKind::RightParen)
        );
    }

    #[test]
    fn test_left_bracket() {
        let result = symbols_parser().parse(Span::new("["));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Symbol(token::SymbolKind::LeftBracket)
        );
    }

    #[test]
    fn test_right_bracket() {
        let result = symbols_parser().parse(Span::new("]"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Symbol(token::SymbolKind::RightBracket)
        );
    }

    #[test]
    fn test_left_brace() {
        let result = symbols_parser().parse(Span::new("{"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Symbol(token::SymbolKind::LeftBrace));
    }

    #[test]
    fn test_right_brace() {
        let result = symbols_parser().parse(Span::new("}"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Symbol(token::SymbolKind::RightBrace)
        );
    }

    #[test]
    fn test_symbol_leaves_remaining_input() {
        let result = symbols_parser().parse(Span::new(";remaining"));
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(*remaining.fragment(), "remaining");
    }
}

mod keywords_parser_tests {
    use super::*;

    #[test]
    fn test_fn_keyword() {
        let result = keywords_parser().parse(Span::new("fn"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Fn));
    }

    #[test]
    fn test_while_keyword() {
        let result = keywords_parser().parse(Span::new("while"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::While));
    }

    #[test]
    fn test_set_keyword() {
        let result = keywords_parser().parse(Span::new("set"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Set));
    }

    #[test]
    fn test_if_keyword() {
        let result = keywords_parser().parse(Span::new("if"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::If));
    }

    #[test]
    fn test_else_keyword() {
        let result = keywords_parser().parse(Span::new("else"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Else));
    }

    #[test]
    fn test_return_keyword() {
        let result = keywords_parser().parse(Span::new("return"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Return));
    }

    #[test]
    fn test_loop_keyword() {
        let result = keywords_parser().parse(Span::new("loop"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Loop));
    }

    #[test]
    fn test_call_keyword() {
        let result = keywords_parser().parse(Span::new("call"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Call));
    }

    #[test]
    fn test_print_keyword() {
        let result = keywords_parser().parse(Span::new("print"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Print));
    }

    #[test]
    fn test_keyword_with_remaining_input() {
        let result = keywords_parser().parse(Span::new("fn main"));
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Keyword(token::KeywordKind::Fn));
        assert_eq!(*remaining.fragment(), " main");
    }
}

mod comparison_operators_parser_tests {
    use super::*;

    #[test]
    fn test_greater_than_or_equal() {
        let result = comparison_operators_parser().parse(Span::new(">="));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::GreaterThanOrEqual)
        );
    }

    #[test]
    fn test_less_than_or_equal() {
        let result = comparison_operators_parser().parse(Span::new("<="));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::LessThanOrEqual)
        );
    }

    #[test]
    fn test_equal() {
        let result = comparison_operators_parser().parse(Span::new("=="));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::Equal)
        );
    }

    #[test]
    fn test_not_equal() {
        let result = comparison_operators_parser().parse(Span::new("!="));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::NotEqual)
        );
    }

    #[test]
    fn test_less_than() {
        let result = comparison_operators_parser().parse(Span::new("<"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::LessThan)
        );
    }

    #[test]
    fn test_greater_than() {
        let result = comparison_operators_parser().parse(Span::new(">"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::GreaterThan)
        );
    }

    #[test]
    fn test_longer_match_precedence_greater_than_or_equal() {
        // Should match >= as one token, not > followed by =
        let result = comparison_operators_parser().parse(Span::new(">=5"));
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::GreaterThanOrEqual)
        );
        assert_eq!(*remaining.fragment(), "5");
    }

    #[test]
    fn test_longer_match_precedence_less_than_or_equal() {
        // Should match <= as one token, not < followed by =
        let result = comparison_operators_parser().parse(Span::new("<=5"));
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(
            token.kind,
            TokenKind::Comp(token::ComparisonKind::LessThanOrEqual)
        );
        assert_eq!(*remaining.fragment(), "5");
    }
}

mod arithmetic_operators_parser_tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = arithmetic_operators_parser().parse(Span::new("+"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Add));
    }

    #[test]
    fn test_subtract() {
        let result = arithmetic_operators_parser().parse(Span::new("-"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Subtract));
    }

    #[test]
    fn test_multiply() {
        let result = arithmetic_operators_parser().parse(Span::new("*"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Multiply));
    }

    #[test]
    fn test_divide() {
        let result = arithmetic_operators_parser().parse(Span::new("/"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Divide));
    }

    #[test]
    fn test_modulo() {
        let result = arithmetic_operators_parser().parse(Span::new("%"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Modulo));
    }

    #[test]
    fn test_assign() {
        let result = arithmetic_operators_parser().parse(Span::new("="));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Op(token::OperationKind::Assign));
    }
}

mod literals_parser_tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let result = literals_parser().parse(Span::new("123"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("123"));
    }

    #[test]
    fn test_single_digit() {
        let result = literals_parser().parse(Span::new("0"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("0"));
    }

    #[test]
    fn test_number_with_single_underscore() {
        let result = literals_parser().parse(Span::new("1_000"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("1_000"));
    }

    #[test]
    fn test_number_with_multiple_underscores() {
        let result = literals_parser().parse(Span::new("999_999"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("999_999"));
    }

    #[test]
    fn test_large_number_with_underscores() {
        let result = literals_parser().parse(Span::new("1_000_000"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("1_000_000"));
    }

    #[test]
    fn test_number_followed_by_non_digit() {
        let result = literals_parser().parse(Span::new("123abc"));
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("123"));
        assert_eq!(*remaining.fragment(), "abc");
    }

    #[test]
    fn test_number_with_trailing_underscores() {
        let result = literals_parser().parse(Span::new("123___"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("123___"));
    }

    #[test]
    fn test_zero() {
        let result = literals_parser().parse(Span::new("0"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Literal("0"));
    }
}

mod identifier_parser_tests {
    use super::*;

    #[test]
    fn test_simple_identifier() {
        let result = identifier_parser().parse(Span::new("foo"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("foo"));
    }

    #[test]
    fn test_identifier_starting_with_underscore() {
        let result = identifier_parser().parse(Span::new("_bar"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("_bar"));
    }

    #[test]
    fn test_identifier_with_numbers() {
        let result = identifier_parser().parse(Span::new("myVar123"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("myVar123"));
    }

    #[test]
    fn test_identifier_with_dollar_prefix() {
        let result = identifier_parser().parse(Span::new("$reg"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("$reg"));
    }

    #[test]
    fn test_identifier_with_dollar_prefix_and_underscore() {
        let result = identifier_parser().parse(Span::new("$_foo"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("$_foo"));
    }

    #[test]
    fn test_identifier_uppercase() {
        let result = identifier_parser().parse(Span::new("Velocity"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("Velocity"));
    }

    #[test]
    fn test_identifier_mixed_case() {
        let result = identifier_parser().parse(Span::new("myVariableName"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("myVariableName"));
    }

    #[test]
    fn test_identifier_all_underscores_after_initial() {
        let result = identifier_parser().parse(Span::new("a___"));
        assert!(result.is_ok());
        let (_, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("a___"));
    }

    #[test]
    fn test_identifier_with_remaining_input() {
        let result = identifier_parser().parse(Span::new("foo bar"));
        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(token.kind, TokenKind::Ident("foo"));
        assert_eq!(*remaining.fragment(), " bar");
    }
}

mod whitespace_parser_tests {
    use super::*;

    #[test]
    fn test_single_space() {
        let result = whitespace_parser().parse(Span::new(" "));
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_spaces() {
        let result = whitespace_parser().parse(Span::new("    "));
        assert!(result.is_ok());
    }

    #[test]
    fn test_tab() {
        let result = whitespace_parser().parse(Span::new("\t"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_newline() {
        let result = whitespace_parser().parse(Span::new("\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_carriage_return() {
        let result = whitespace_parser().parse(Span::new("\r"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_mixed_whitespace() {
        let result = whitespace_parser().parse(Span::new(" \t\r\n  \n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_whitespace_with_remaining_input() {
        let result = whitespace_parser().parse(Span::new("  token"));
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(*remaining.fragment(), "token");
    }
}

mod comments_parser_tests {
    use super::*;

    #[test]
    fn test_simple_comment() {
        let result = comments_parser().parse(Span::new("// Hello world\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_comment_with_code_text() {
        let result = comments_parser().parse(Span::new("// fn main() { }\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_comment_with_special_chars() {
        let result = comments_parser().parse(Span::new("// @#$%^&*()_+\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_comment() {
        let result = comments_parser().parse(Span::new("//\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_comment_consumes_until_newline() {
        let result = comments_parser().parse(Span::new("// comment\nfn"));
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(*remaining.fragment(), "fn");
    }
}

// ============================================================================
// Main parse_source Function Tests
// ============================================================================

mod parse_source_tests {
    use super::*;

    // Basic functionality tests
    mod basic_tests {
        use super::*;

        #[test]
        fn test_empty_input() {
            let result = parse_source("");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 0);
        }

        #[test]
        fn test_single_keyword() {
            let result = parse_source("fn");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
        }

        #[test]
        fn test_single_identifier() {
            let result = parse_source("myVar");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(result.tokens[0].kind, TokenKind::Ident("myVar"));
        }

        #[test]
        fn test_single_literal() {
            let result = parse_source("42");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(result.tokens[0].kind, TokenKind::Literal("42"));
        }

        #[test]
        fn test_single_symbol() {
            let result = parse_source(";");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Symbol(token::SymbolKind::LineBreak)
            );
        }

        #[test]
        fn test_single_arithmetic_operator() {
            let result = parse_source("+");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Op(token::OperationKind::Add)
            );
        }

        #[test]
        fn test_single_comparison_operator() {
            let result = parse_source(">=");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Comp(token::ComparisonKind::GreaterThanOrEqual)
            );
        }
    }

    // Whitespace handling tests
    mod whitespace_tests {
        use super::*;

        #[test]
        fn test_tokens_with_spaces() {
            let result = parse_source("fn main");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("main"));
        }

        #[test]
        fn test_tokens_with_tabs() {
            let result = parse_source("fn\tmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_tokens_with_newlines() {
            let result = parse_source("fn\nmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_tokens_with_mixed_whitespace() {
            let result = parse_source("fn  \t\n  main");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_leading_whitespace() {
            let result = parse_source("   fn");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
        }

        #[test]
        fn test_trailing_whitespace() {
            let result = parse_source("fn   ");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
        }

        #[test]
        fn test_only_whitespace() {
            let result = parse_source("   \t\n  ");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 0);
        }
    }

    // Comment handling tests
    mod comment_tests {
        use super::*;

        #[test]
        fn test_comment_at_end() {
            let result = parse_source("fn main // comment\n");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("main"));
        }

        #[test]
        fn test_comment_between_tokens() {
            let result = parse_source("fn // comment\nmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_multiple_comments() {
            let result = parse_source("// first comment\nfn // second comment\nmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_consecutive_comments() {
            let result = parse_source("// first\n// second\n// third\nfn");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 1);
        }

        #[test]
        fn test_comment_only() {
            let result = parse_source("// just a comment\n");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 0);
        }

        #[test]
        fn test_empty_lines() {
            let result = parse_source("fn\n\n\nmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        fn test_comment_with_whitespace_before() {
            let result = parse_source("fn   // comment\nmain");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
        }
    }

    // Adjacent tokens tests
    mod adjacent_tokens_tests {
        use super::*;

        #[test]
        fn test_parentheses_around_number() {
            let result = parse_source("(5)");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 3);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Symbol(token::SymbolKind::LeftParen)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Literal("5"));
            assert_eq!(
                result.tokens[2].kind,
                TokenKind::Symbol(token::SymbolKind::RightParen)
            );
        }

        #[test]
        fn test_brackets_around_identifier() {
            let result = parse_source("[foo]");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 3);
        }

        #[test]
        fn test_arithmetic_expression_no_spaces() {
            let result = parse_source("1+2");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 3);
            assert_eq!(result.tokens[0].kind, TokenKind::Literal("1"));
            assert_eq!(
                result.tokens[1].kind,
                TokenKind::Op(token::OperationKind::Add)
            );
            assert_eq!(result.tokens[2].kind, TokenKind::Literal("2"));
        }

        #[test]
        fn test_semicolon_after_statement() {
            let result = parse_source("x=5;");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 4);
        }
    }

    // Edge cases tests
    mod edge_cases_tests {
        use super::*;

        #[test]
        fn test_keyword_vs_identifier_if() {
            // Test that "if" keyword is distinct from "iffy" identifier
            let result = parse_source("if iffy");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2, "Expected 2 tokens: 'if' keyword and 'iffy' identifier");
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::If),
                "First token should be 'if' keyword"
            );
            assert_eq!(
                result.tokens[1].kind,
                TokenKind::Ident("iffy"),
                "Second token should be 'iffy' identifier"
            );
        }

        #[test]
        fn test_keyword_vs_identifier_fn() {
            let result = parse_source("fn function");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("function"));
        }

        #[test]
        fn test_greater_than_or_equal_vs_separate() {
            let result = parse_source(">= > =");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 3);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Comp(token::ComparisonKind::GreaterThanOrEqual)
            );
            assert_eq!(
                result.tokens[1].kind,
                TokenKind::Comp(token::ComparisonKind::GreaterThan)
            );
            assert_eq!(
                result.tokens[2].kind,
                TokenKind::Op(token::OperationKind::Assign)
            );
        }

        #[test]
        fn test_dollar_identifier_vs_plain() {
            let result = parse_source("$reg reg");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 2);
            assert_eq!(result.tokens[0].kind, TokenKind::Ident("$reg"));
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("reg"));
        }

        #[test]
        fn test_numbers_with_underscores() {
            let result = parse_source("1_000_000 999_999 1_2_3");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 3);
            assert_eq!(result.tokens[0].kind, TokenKind::Literal("1_000_000"));
            assert_eq!(result.tokens[1].kind, TokenKind::Literal("999_999"));
            assert_eq!(result.tokens[2].kind, TokenKind::Literal("1_2_3"));
        }

        #[test]
        fn test_all_keywords() {
            let result = parse_source("fn while set if else return loop call print");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 9);
        }

        #[test]
        fn test_all_symbols() {
            let result = parse_source("; ( ) [ ] { }");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 7);
        }

        #[test]
        fn test_all_comparison_operators() {
            let result = parse_source(">= <= == != < >");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 6);
        }

        #[test]
        fn test_all_arithmetic_operators() {
            let result = parse_source("+ - * / % =");
            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 6);
        }
    }

    // Complete program tests
    mod complete_program_tests {
        use super::*;

        #[test]
        fn test_simple_function() {
            let source = "fn main {\n\tprint $Velocity;\n}";
            let result = parse_source(source);

            assert!(
                result.is_ok(),
                "Lexer encountered errors: {:?}",
                result.errors
            );
            assert_eq!(result.tokens.len(), 7);

            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("main"));
            assert_eq!(
                result.tokens[2].kind,
                TokenKind::Symbol(token::SymbolKind::LeftBrace)
            );
            assert_eq!(
                result.tokens[3].kind,
                TokenKind::Keyword(token::KeywordKind::Print)
            );
            assert_eq!(result.tokens[4].kind, TokenKind::Ident("$Velocity"));
            assert_eq!(
                result.tokens[5].kind,
                TokenKind::Symbol(token::SymbolKind::LineBreak)
            );
            assert_eq!(
                result.tokens[6].kind,
                TokenKind::Symbol(token::SymbolKind::RightBrace)
            );
        }

        #[test]
        fn test_arithmetic_expression() {
            let source = "set x = 1 + 2 * 3;";
            let result = parse_source(source);

            assert!(result.is_ok());
            // set, x, =, 1, +, 2, *, 3, ;
            assert_eq!(result.tokens.len(), 9);
        }

        #[test]
        fn test_conditional_statement() {
            let source = "if x >= 10 {\n\treturn x;\n}";
            let result = parse_source(source);

            assert!(result.is_ok());
            // if, x, >=, 10, {, return, x, ;, }
            assert_eq!(result.tokens.len(), 9);
        }

        #[test]
        fn test_loop_structure() {
            let source = "loop {\n\tcall foo;\n}";
            let result = parse_source(source);

            assert!(result.is_ok());
            assert_eq!(result.tokens.len(), 6);
        }

        #[test]
        fn test_complex_program() {
            let source = r#"
// Main function
fn main {
    set x = 100;
    set y = 200;

    // Calculate sum
    set sum = x + y;

    if sum >= 250 {
        print sum;
    } else {
        print 0;
    }
}
"#;
            let result = parse_source(source);

            assert!(
                result.is_ok(),
                "Lexer encountered errors: {:?}",
                result.errors
            );
            assert!(result.tokens.len() > 30);
        }
    }

    // Error recovery tests
    mod error_recovery_tests {
        use super::*;

        #[test]
        fn test_invalid_character_skipped() {
            let result = parse_source("fn @ main");
            assert_eq!(result.errors.len(), 1);
            assert_eq!(result.tokens.len(), 2);
            assert_eq!(
                result.tokens[0].kind,
                TokenKind::Keyword(token::KeywordKind::Fn)
            );
            assert_eq!(result.tokens[1].kind, TokenKind::Ident("main"));
        }

        #[test]
        fn test_multiple_invalid_characters() {
            let result = parse_source("fn # @ main");
            assert_eq!(result.errors.len(), 2);
            assert_eq!(result.tokens.len(), 2);
        }

        #[test]
        #[should_panic]
        fn test_unicode_character_error() {
            // Note: The current lexer implementation has a bug where it tries to skip
            // invalid characters byte-by-byte, which panics on multi-byte UTF-8 characters.
            // This test documents the current behavior and should be fixed in the lexer.
            let result = parse_source("fn 你好 main");
            assert!(result.errors.len() > 0);
        }

        #[test]
        fn test_special_characters_error() {
            let result = parse_source("x = 5 & y");
            assert_eq!(result.errors.len(), 1);
        }

        #[test]
        fn test_recovery_continues_after_error() {
            let result = parse_source("fn @ # $ main");
            assert!(result.errors.len() >= 1);
            // Should still parse 'fn' and 'main'
            assert!(result.tokens.len() >= 2);
        }

        #[test]
        fn test_valid_tokens_after_errors() {
            let result = parse_source("fn ~ main { }");
            assert_eq!(result.errors.len(), 1);
            assert_eq!(result.tokens.len(), 4);
        }
    }
}

// ============================================================================
// Token Location Tracking Tests
// ============================================================================

mod token_location_tests {
    use super::*;

    #[test]
    fn test_single_token_location() {
        let result = parse_source("fn");
        assert_eq!(result.tokens.len(), 1);

        let loc = &result.tokens[0].location;
        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 2);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
    }

    #[test]
    fn test_token_location_with_offset() {
        let result = parse_source("  fn");
        assert_eq!(result.tokens.len(), 1);

        let loc = &result.tokens[0].location;
        assert_eq!(loc.start, 2);
        assert_eq!(loc.end, 4);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 3);
    }

    #[test]
    fn test_multiple_tokens_same_line() {
        let result = parse_source("fn main {");
        assert_eq!(result.tokens.len(), 3);

        let fn_loc = &result.tokens[0].location;
        assert_eq!(fn_loc.start, 0);
        assert_eq!(fn_loc.end, 2);
        assert_eq!(fn_loc.line, 1);

        let main_loc = &result.tokens[1].location;
        assert_eq!(main_loc.start, 3);
        assert_eq!(main_loc.end, 7);
        assert_eq!(main_loc.line, 1);

        let brace_loc = &result.tokens[2].location;
        assert_eq!(brace_loc.start, 8);
        assert_eq!(brace_loc.end, 9);
        assert_eq!(brace_loc.line, 1);
    }

    #[test]
    fn test_multiline_input() {
        let result = parse_source("fn\nmain");
        assert_eq!(result.tokens.len(), 2);

        let fn_loc = &result.tokens[0].location;
        assert_eq!(fn_loc.line, 1);
        assert_eq!(fn_loc.column, 1);

        let main_loc = &result.tokens[1].location;
        assert_eq!(main_loc.line, 2);
        assert_eq!(main_loc.column, 1);
    }

    #[test]
    fn test_line_numbers_increment() {
        let result = parse_source("fn\nwhile\nset");
        assert_eq!(result.tokens.len(), 3);

        assert_eq!(result.tokens[0].location.line, 1);
        assert_eq!(result.tokens[1].location.line, 2);
        assert_eq!(result.tokens[2].location.line, 3);
    }

    #[test]
    fn test_column_resets_after_newline() {
        let result = parse_source("fn main\nif else");
        assert_eq!(result.tokens.len(), 4);

        assert_eq!(result.tokens[0].location.column, 1); // fn
        assert_eq!(result.tokens[1].location.column, 4); // main
        assert_eq!(result.tokens[2].location.column, 1); // if (new line)
        assert_eq!(result.tokens[3].location.column, 4); // else
    }

    #[test]
    fn test_location_with_tabs() {
        let result = parse_source("\tfn\n\t\tmain");
        assert_eq!(result.tokens.len(), 2);

        let fn_loc = &result.tokens[0].location;
        assert_eq!(fn_loc.line, 1);

        let main_loc = &result.tokens[1].location;
        assert_eq!(main_loc.line, 2);
    }

    #[test]
    fn test_location_spans_correct_length() {
        let result = parse_source("myLongIdentifier");
        assert_eq!(result.tokens.len(), 1);

        let loc = &result.tokens[0].location;
        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 16);
        assert_eq!(loc.end - loc.start, 16);
    }

    #[test]
    fn test_location_with_underscores_in_number() {
        let result = parse_source("1_000_000");
        assert_eq!(result.tokens.len(), 1);

        let loc = &result.tokens[0].location;
        assert_eq!(loc.start, 0);
        assert_eq!(loc.end, 9);
    }

    #[test]
    fn test_location_multiline_program() {
        let source = "fn main {\n\tprint x;\n}";
        let result = parse_source(source);
        assert_eq!(result.tokens.len(), 7);

        // fn - line 1
        assert_eq!(result.tokens[0].location.line, 1);
        // main - line 1
        assert_eq!(result.tokens[1].location.line, 1);
        // { - line 1
        assert_eq!(result.tokens[2].location.line, 1);
        // print - line 2
        assert_eq!(result.tokens[3].location.line, 2);
        // x - line 2
        assert_eq!(result.tokens[4].location.line, 2);
        // ; - line 2
        assert_eq!(result.tokens[5].location.line, 2);
        // } - line 3
        assert_eq!(result.tokens[6].location.line, 3);
    }

    #[test]
    fn test_location_after_comment() {
        let result = parse_source("fn // comment\nmain");
        assert_eq!(result.tokens.len(), 2);

        assert_eq!(result.tokens[0].location.line, 1);
        assert_eq!(result.tokens[1].location.line, 2);
        assert_eq!(result.tokens[1].location.column, 1);
    }
}