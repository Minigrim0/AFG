use super::token::{TokenError, TokenErrorType};
use crate::token::{TokenMetaData, Token, TokenType};

// ========================================
// TokenError Construction Tests
// ========================================

#[test]
fn test_token_error_with_metadata() {
    let meta = TokenMetaData { line: 5, char: 10 };
    let error = TokenError::new(
        TokenErrorType::UnexpectedToken,
        "Unexpected token found",
        Some(meta)
    );

    let error_string = format!("{}", error);
    assert!(error_string.contains("line 5"));
    assert!(error_string.contains("char 10"));
    assert!(error_string.contains("Unexpected token found"));
}

#[test]
fn test_token_error_without_metadata() {
    let error = TokenError::new(
        TokenErrorType::UnexpectedEndOfStream,
        "Reached end of file unexpectedly",
        None
    );

    let error_string = format!("{}", error);
    assert!(!error_string.contains("line"));
    assert!(!error_string.contains("char"));
    assert!(error_string.contains("Reached end of file unexpectedly"));
}

// ========================================
// TokenErrorType Display Tests
// ========================================

#[test]
fn test_error_type_invalid() {
    let error = TokenError::new(TokenErrorType::Invalid, "Invalid token", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("Invalid"));
}

#[test]
fn test_error_type_not_a_literal() {
    let error = TokenError::new(TokenErrorType::NotALiteral, "Expected literal", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("NotALiteral"));
}

#[test]
fn test_error_type_unexpected_token() {
    let error = TokenError::new(TokenErrorType::UnexpectedToken, "Wrong token", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("UnexpectedToken"));
}

#[test]
fn test_error_type_unexpected_end_of_stream() {
    let error = TokenError::new(TokenErrorType::UnexpectedEndOfStream, "EOF", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("UnexpectedEndOfStream"));
}

#[test]
fn test_error_type_parse_error() {
    let error = TokenError::new(TokenErrorType::ParseError, "Parse failed", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("ParseError"));
}

#[test]
fn test_error_type_empty_token() {
    let error = TokenError::new(TokenErrorType::EmptyToken, "Token empty", None);
    let error_string = format!("{}", error);
    assert!(error_string.contains("EmptyToken"));
}

#[test]
fn test_error_type_invalid_arithmetic_operator() {
    let error = TokenError::new(
        TokenErrorType::InvalidArithmeticOperator,
        "Bad operator",
        None
    );
    let error_string = format!("{}", error);
    assert!(error_string.contains("InvalidArithmeticOperator"));
}

#[test]
fn test_error_type_invalid_comparison_operator() {
    let error = TokenError::new(
        TokenErrorType::InvalidComparisonOperator,
        "Bad comparison",
        None
    );
    let error_string = format!("{}", error);
    assert!(error_string.contains("InvalidComparisonOperator"));
}

// ========================================
// Token Method Error Tests
// ========================================

#[test]
fn test_get_literal_value_error_on_non_literal() {
    let token = Token::new(TokenType::ID, Some("identifier".to_string()), 3, 7);
    let result = token.get_literal_value();

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);
    assert!(error_string.contains("line 3"));
    assert!(error_string.contains("char 7"));
}

#[test]
fn test_get_literal_value_error_on_empty_token() {
    let token = Token::new(TokenType::ID, None, 1, 0);
    let result = token.get_literal_value();

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);
    assert!(error_string.contains("NotALiteral"));
}

#[test]
fn test_get_value_error_on_empty_token() {
    let token = Token::new(TokenType::LPAREN, None, 2, 5);
    let result = token.get_value();

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);
    assert!(error_string.contains("EmptyToken"));
    assert!(error_string.contains("line 2"));
    assert!(error_string.contains("char 5"));
}

// ========================================
// Integration Tests with AST Parsing
// ========================================

#[test]
fn test_parse_error_preserves_line_info() {
    use crate::token::lex;
    use crate::ast::AST;

    // This should fail because 'set' is not allowed at top level
    let code = "set x = 5;";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Should contain line information
    assert!(error_string.contains("line"));
    assert!(error_string.contains("char"));
}

#[test]
fn test_parse_error_multiline_preserves_correct_line() {
    use crate::token::lex;
    use crate::ast::AST;

    let code = "fn main() {\n    set x = 5;\n    invalid_keyword;\n}";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    // Should error on line 3 (invalid_keyword)
    if let Err(error) = result {
        let error_string = format!("{}", error);
        assert!(error_string.contains("line 3"));
    }
}

#[test]
fn test_missing_semicolon_error_location() {
    use crate::token::lex;
    use crate::ast::AST;

    let code = "fn main() { set x = 5 }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Error should contain location information
    assert!(error_string.contains("line"));
}

#[test]
fn test_unexpected_eof_error() {
    use crate::token::lex;
    use crate::ast::AST;

    let code = "fn main() { set x =";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    assert!(error_string.contains("UnexpectedEndOfStream") || error_string.contains("end"));
}

#[test]
fn test_invalid_operator_error_with_location() {
    use crate::token::lex;
    use crate::ast::AST;

    // Using '&' which is not a valid operator
    let code = "fn main() { set x = a & b; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Should indicate invalid arithmetic operator
    assert!(error_string.contains("InvalidArithmeticOperator") || error_string.contains("Unknown operator"));
}

#[test]
fn test_invalid_comparison_operator_error() {
    use crate::token::lex;
    use crate::ast::AST;

    // Using '<>' which is not valid (should be !=)
    let code = "fn main() { if x <> y {} }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    // This will fail during parsing as <> becomes < followed by >
    assert!(result.is_err());
}

// ========================================
// Ensure Next Token Error Tests
// ========================================

#[test]
fn test_ensure_next_token_missing_expected() {
    use crate::token::{ensure_next_token, lex};

    let tokens = lex("fn main ( ) {}");
    let mut token_iter = tokens.into_iter().peekable();

    // Skip 'fn' and 'main'
    token_iter.next();
    token_iter.next();

    // Next is '(' but we expect '{'
    let result = ensure_next_token(&mut token_iter, TokenType::LBRACE, None);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_string = format!("{}", error);
    assert!(error_string.contains("UnexpectedToken"));
}

#[test]
fn test_ensure_next_token_unexpected_eof() {
    use crate::token::{ensure_next_token, lex};

    let tokens = lex("fn main");
    let mut token_iter = tokens.into_iter().peekable();

    // Skip tokens
    token_iter.next();
    token_iter.next();

    // Try to get next token when none exists
    let result = ensure_next_token(&mut token_iter, TokenType::LPAREN, None);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_string = format!("{}", error);
    assert!(error_string.contains("UnexpectedEndOfStream"));
}

// ========================================
// Complex Error Scenario Tests
// ========================================

#[test]
fn test_nested_error_reporting() {
    use crate::token::lex;
    use crate::ast::AST;

    let code = r#"
        fn main() {
            if x > 0 {
                set y = 5;
                if z < 10 {
                    set w = invalid_op 3;
                }
            }
        }
    "#;

    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Should report error in nested context
    assert!(error_string.contains("line"));
}

#[test]
fn test_function_definition_error() {
    use crate::token::lex;
    use crate::ast::AST;

    // Missing left brace
    let code = "fn test() set x = 5; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    assert!(error_string.contains("UnexpectedToken") && error_string.contains("line"));
}

#[test]
fn test_multiple_errors_reports_first() {
    use crate::token::lex;
    use crate::ast::AST;

    // Multiple errors: missing semicolon and invalid syntax
    let code = "fn main() { set x = 5 set y = 10; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    // Should fail on first error encountered
}

#[test]
fn test_array_access_error() {
    use crate::token::lex;
    use crate::ast::AST;

    // Missing closing bracket
    let code = "fn main() { set x = arr[5; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    assert!(error_string.contains("line"));
}

#[test]
fn test_function_call_error() {
    use crate::token::lex;
    use crate::ast::AST;

    // Missing closing parenthesis
    let code = "fn main() { call test(x, y; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

#[test]
fn test_while_loop_error() {
    use crate::token::lex;
    use crate::ast::AST;

    // Missing opening brace
    let code = "fn main() { while x < 10 set x = x + 1; } }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

// ========================================
// Error Message Quality Tests
// ========================================

#[test]
fn test_error_message_is_descriptive() {
    let meta = TokenMetaData { line: 42, char: 15 };
    let error = TokenError::new(
        TokenErrorType::UnexpectedToken,
        "Expected keyword 'fn' but found identifier",
        Some(meta)
    );

    let error_string = format!("{}", error);

    // Check that all important information is present
    assert!(error_string.contains("42"));      // line number
    assert!(error_string.contains("15"));      // char position
    assert!(error_string.contains("Expected")); // descriptive message
    assert!(error_string.contains("fn"));      // specific context
}

#[test]
fn test_error_distinguishes_types() {
    let meta = TokenMetaData { line: 1, char: 0 };

    let error1 = TokenError::new(TokenErrorType::Invalid, "msg", Some(meta));
    let error2 = TokenError::new(TokenErrorType::UnexpectedToken, "msg", Some(meta));

    let string1 = format!("{}", error1);
    let string2 = format!("{}", error2);

    // Different error types should produce different messages
    assert_ne!(string1, string2);
}
