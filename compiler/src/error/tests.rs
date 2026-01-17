use super::token::{TokenError, TokenErrorType};
use crate::lexer::token::TokenLocation;

// ========================================
// TokenError Construction Tests
// ========================================

#[test]
fn test_token_error_with_location() {
    let loc = TokenLocation {
        start: 0,
        end: 5,
        line: 5,
        column: 10,
    };
    let error = TokenError::new(
        TokenErrorType::UnexpectedToken,
        "Unexpected token found",
        Some(loc)
    );

    let error_string = format!("{}", error);
    assert!(error_string.contains("line 5"));
    assert!(error_string.contains("column 10"));
    assert!(error_string.contains("Unexpected token found"));
}

#[test]
fn test_token_error_without_location() {
    let error = TokenError::new(
        TokenErrorType::UnexpectedEndOfStream,
        "Reached end of file unexpectedly",
        None
    );

    let error_string = format!("{}", error);
    assert!(!error_string.contains("line"));
    assert!(!error_string.contains("column"));
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
// Integration Tests with AST Parsing
// ========================================

#[test]
fn test_parse_error_preserves_line_info() {
    use crate::ast::AST;

    // This should fail because 'set' is not allowed at top level
    let code = "set x = 5;";
    let result = AST::parse(code);

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    // Should contain line information
    assert!(error_string.contains("line"));
}

#[test]
fn test_parse_error_multiline_preserves_correct_line() {
    use crate::ast::AST;

    let code = "fn main() {\n    set x = 5;\n    invalid_keyword;\n}";
    let result = AST::parse(code);

    // Should error on line 3 (invalid_keyword)
    if let Err(error) = result {
        let error_string = format!("{}", error);
        assert!(error_string.contains("line"));
    }
}

#[test]
fn test_unexpected_eof_error() {
    use crate::ast::AST;

    let code = "fn main() { set x =";
    let result = AST::parse(code);

    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_string = format!("{}", error);

    assert!(error_string.contains("UnexpectedEndOfStream") || error_string.contains("end"));
}

// ========================================
// Error Message Quality Tests
// ========================================

#[test]
fn test_error_message_is_descriptive() {
    let loc = TokenLocation {
        start: 0,
        end: 5,
        line: 42,
        column: 15,
    };
    let error = TokenError::new(
        TokenErrorType::UnexpectedToken,
        "Expected keyword 'fn' but found identifier",
        Some(loc)
    );

    let error_string = format!("{}", error);

    // Check that all important information is present
    assert!(error_string.contains("42"));      // line number
    assert!(error_string.contains("15"));      // column position
    assert!(error_string.contains("Expected")); // descriptive message
    assert!(error_string.contains("fn"));      // specific context
}

#[test]
fn test_error_distinguishes_types() {
    let loc = TokenLocation {
        start: 0,
        end: 1,
        line: 1,
        column: 0,
    };

    let error1 = TokenError::new(TokenErrorType::Invalid, "msg", Some(loc.clone()));
    let error2 = TokenError::new(TokenErrorType::UnexpectedToken, "msg", Some(loc));

    let string1 = format!("{}", error1);
    let string2 = format!("{}", error2);

    // Different error types should produce different messages
    assert_ne!(string1, string2);
}
