use super::lex;
use super::{kind::TokenType, Token};

// ========================================
// Token Behavior Tests
// ========================================

#[test]
fn test_is_literal() {
    // Test positive integer literal
    assert!(Token::new(TokenType::ID, Some("32".to_string()), 0, 0).is_literal());

    // Test negative integer literal
    assert!(Token::new(TokenType::ID, Some("-42".to_string()), 0, 0).is_literal());

    // Test zero
    assert!(Token::new(TokenType::ID, Some("0".to_string()), 0, 0).is_literal());

    // Test non-ID token types should not be literals
    assert!(!Token::new(TokenType::COMMENT, Some("32".to_string()), 0, 0).is_literal());
    assert!(!Token::new(TokenType::KEYWORD, Some("32".to_string()), 0, 0).is_literal());

    // Test memory value identifiers (starting with $)
    assert!(!Token::new(TokenType::ID, Some("$VelocityY".to_string()), 0, 0).is_literal());

    // Test regular identifiers
    assert!(!Token::new(TokenType::ID, Some("michel".to_string()), 0, 0).is_literal());
    assert!(!Token::new(TokenType::ID, Some("x".to_string()), 0, 0).is_literal());
}

#[test]
fn test_get_literal_value() {
    // Test parsing positive integer
    let token = Token::new(TokenType::ID, Some("42".to_string()), 1, 5);
    assert_eq!(token.get_literal_value().unwrap(), 42);

    // Test parsing negative integer
    let token = Token::new(TokenType::ID, Some("-15".to_string()), 1, 5);
    assert_eq!(token.get_literal_value().unwrap(), -15);

    // Test error on non-literal identifier
    let token = Token::new(TokenType::ID, Some("test".to_string()), 2, 10);
    assert!(token.get_literal_value().is_err());

    // Test error on empty value
    let token = Token::new(TokenType::ID, None, 3, 0);
    assert!(token.get_literal_value().is_err());
}

#[test]
fn test_get_value() {
    // Test getting value from token
    let token = Token::new(TokenType::ID, Some("test".to_string()), 1, 0);
    assert_eq!(token.get_value().unwrap(), "test");

    // Test error on empty value
    let token = Token::new(TokenType::LPAREN, None, 1, 0);
    assert!(token.get_value().is_err());
}

#[test]
fn test_is_token_type() {
    let token = Token::new(TokenType::KEYWORD, Some("if".to_string()), 1, 0);
    assert!(token.is(TokenType::KEYWORD));
    assert!(!token.is(TokenType::ID));
}

// ========================================
// Basic Lexer Tests
// ========================================

#[test]
fn test_lexer_simple_assignment() {
    let text = "set test = -1;";
    let lexed = lex(text);
    assert_eq!(lexed.len(), 5);
    assert_eq!(lexed[0].token_type, TokenType::KEYWORD);
    assert_eq!(lexed[0].value, Some("set".to_string()));
    assert_eq!(lexed[1].token_type, TokenType::ID);
    assert_eq!(lexed[1].value, Some("test".to_string()));
    assert_eq!(lexed[2].token_type, TokenType::OP);
    assert_eq!(lexed[2].value, Some("=".to_string()));
    assert_eq!(lexed[3].token_type, TokenType::ID);
    assert!(lexed[3].is_literal());
    assert_eq!(lexed[3].value, Some("-1".to_string()));
    assert_eq!(lexed[4].token_type, TokenType::ENDL);
}

#[test]
fn test_lexer_arithmetic_expression() {
    let text = "set test = 1 + 2;";
    let lexed = lex(text);
    assert_eq!(lexed.len(), 7);
    assert_eq!(lexed[0].token_type, TokenType::KEYWORD);
    assert_eq!(lexed[1].token_type, TokenType::ID);
    assert_eq!(lexed[2].token_type, TokenType::OP);
    assert_eq!(lexed[3].token_type, TokenType::ID);
    assert!(lexed[3].is_literal());
    assert_eq!(lexed[4].token_type, TokenType::OP);
    assert_eq!(lexed[4].value, Some("+".to_string()));
    assert_eq!(lexed[5].token_type, TokenType::ID);
    assert!(lexed[5].is_literal());
    assert_eq!(lexed[6].token_type, TokenType::ENDL);
}

// ========================================
// Operator Lexing Tests
// ========================================

#[test]
fn test_lexer_all_arithmetic_operators() {
    // Addition
    let lexed = lex("a + b;");
    assert_eq!(lexed[1].value, Some("+".to_string()));

    // Subtraction
    let lexed = lex("a - b;");
    assert_eq!(lexed[1].value, Some("-".to_string()));

    // Multiplication
    let lexed = lex("a * b;");
    assert_eq!(lexed[1].value, Some("*".to_string()));

    // Division
    let lexed = lex("a / b;");
    assert_eq!(lexed[1].value, Some("/".to_string()));

    // Modulo
    let lexed = lex("a % b;");
    assert_eq!(lexed[1].value, Some("%".to_string()));
}

#[test]
fn test_lexer_comparison_operators() {
    // Greater than
    let lexed = lex("a > b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some(">".to_string()));

    // Greater than or equal
    let lexed = lex("a >= b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some(">=".to_string()));

    // Less than
    let lexed = lex("a < b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some("<".to_string()));

    // Less than or equal
    let lexed = lex("a <= b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some("<=".to_string()));

    // Equality
    let lexed = lex("a == b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some("==".to_string()));

    // Inequality
    let lexed = lex("a != b;");
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some("!=".to_string()));
}

#[test]
fn test_lexer_double_operator_merging() {
    // Test that >= is treated as a single operator
    let lexed = lex("x >= 5;");
    assert_eq!(lexed.len(), 4);
    assert_eq!(lexed[1].value, Some(">=".to_string()));

    // Test that <= is treated as a single operator
    let lexed = lex("x <= 10;");
    assert_eq!(lexed.len(), 4);
    assert_eq!(lexed[1].value, Some("<=".to_string()));

    // Test that == is treated as a single operator
    let lexed = lex("x == 3;");
    assert_eq!(lexed.len(), 4);
    assert_eq!(lexed[1].value, Some("==".to_string()));

    // Test that != is treated as a single operator
    let lexed = lex("x != 0;");
    assert_eq!(lexed.len(), 4);
    assert_eq!(lexed[1].value, Some("!=".to_string()));
}

// ========================================
// Keyword Lexing Tests
// ========================================

#[test]
fn test_lexer_keywords() {
    let keywords = vec![
        "fn", "while", "set", "if", "else", "return", "loop", "call", "print",
    ];

    for keyword in keywords {
        let lexed = lex(keyword);
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed[0].token_type, TokenType::KEYWORD);
        assert_eq!(lexed[0].value, Some(keyword.to_string()));
    }
}

// ========================================
// Parentheses and Braces Tests
// ========================================

#[test]
fn test_lexer_parentheses() {
    let lexed = lex("fn test() {}");
    assert!(lexed.iter().any(|t| t.token_type == TokenType::LPAREN));
    assert!(lexed.iter().any(|t| t.token_type == TokenType::RPAREN));
}

#[test]
fn test_lexer_braces() {
    let lexed = lex("{ }");
    assert_eq!(lexed.len(), 2);
    assert_eq!(lexed[0].token_type, TokenType::LBRACE);
    assert_eq!(lexed[1].token_type, TokenType::RBRACE);
}

#[test]
fn test_lexer_brackets() {
    let lexed = lex("arr[0];");
    assert_eq!(lexed[1].token_type, TokenType::LBRACKET);
    assert_eq!(lexed[3].token_type, TokenType::RBRACKET);
}

// ========================================
// Negative Number Handling Tests
// ========================================

#[test]
fn test_lexer_negative_immediate() {
    // Negative number after assignment should be merged
    let lexed = lex("set x = -5;");
    assert_eq!(lexed.len(), 5);
    assert_eq!(lexed[3].value, Some("-5".to_string()));
    assert!(lexed[3].is_literal());

    // Negative number after operator should be merged
    let lexed = lex("set x = y + -3;");
    assert_eq!(lexed[5].value, Some("-3".to_string()));
    assert!(lexed[5].is_literal());
}

#[test]
fn test_lexer_subtraction_vs_negative() {
    // Subtraction between identifiers should remain separate
    let lexed = lex("a - b;");
    assert_eq!(lexed.len(), 4);
    assert_eq!(lexed[1].token_type, TokenType::OP);
    assert_eq!(lexed[1].value, Some("-".to_string()));

    // But negative after non-ID should merge
    let lexed = lex("set x = -10;");
    assert_eq!(lexed[3].value, Some("-10".to_string()));
}

// ========================================
// Comment Handling Tests
// ========================================

#[test]
fn test_lexer_single_line_comment() {
    let lexed = lex("set x = 5; // this is a comment");
    // Comments should be filtered out entirely
    assert!(lexed.iter().all(|t| t.token_type != TokenType::COMMENT));
    assert_eq!(
        lexed.len(),
        5,
        "There should be 5 tokens remaining in the lexed source (tokens: {} - {:?})",
        lexed.len(),
        lexed
    ); // set, x, =, 5, ;
}

#[test]
fn test_lexer_comment_entire_line() {
    let lexed = lex("// full line comment\nset x = 1;");
    // Should only have the assignment tokens
    assert_eq!(lexed.len(), 5);
    assert_eq!(lexed[0].value, Some("set".to_string()));
}

#[test]
fn test_lexer_multiple_line_comments() {
    let code = "set x = 1; // comment 1\nset y = 2; // comment 2";
    let lexed = lex(code);
    // Should have 10 tokens: 5 for each assignment
    assert_eq!(lexed.len(), 10);
}

// ========================================
// Complex Expression Tests
// ========================================

#[test]
fn test_lexer_complex_arithmetic() {
    let lexed = lex("set result = a + b * c - d / e % f;");
    assert_eq!(lexed[0].value, Some("set".to_string()));
    assert_eq!(lexed[1].value, Some("result".to_string()));
    // Verify all operators are present
    let operators: Vec<_> = lexed
        .iter()
        .filter(|t| t.token_type == TokenType::OP && t.value != Some("=".to_string()))
        .collect();
    assert_eq!(operators.len(), 5); // +, *, -, /, %
}

#[test]
fn test_lexer_nested_function_call() {
    let lexed = lex("call outer(call inner(x, y));");
    assert_eq!(lexed[0].value, Some("call".to_string()));
    assert_eq!(lexed[1].value, Some("outer".to_string()));
    // Verify parentheses and inner call structure
    assert!(
        lexed
            .iter()
            .filter(|t| t.token_type == TokenType::LPAREN)
            .count()
            == 2
    );
    assert!(
        lexed
            .iter()
            .filter(|t| t.token_type == TokenType::RPAREN)
            .count()
            == 2
    );
}

// ========================================
// Memory Access Tests
// ========================================

#[test]
fn test_lexer_memory_value() {
    let lexed = lex("set x = $Velocity;");
    assert_eq!(lexed[3].value, Some("$Velocity".to_string()));
    assert_eq!(lexed[3].token_type, TokenType::ID);
}

#[test]
fn test_lexer_array_access() {
    let lexed = lex("arr[5];");
    assert_eq!(lexed[0].value, Some("arr".to_string()));
    assert_eq!(lexed[1].token_type, TokenType::LBRACKET);
    assert_eq!(lexed[2].value, Some("5".to_string()));
    assert_eq!(lexed[3].token_type, TokenType::RBRACKET);
}

// ========================================
// Whitespace and Formatting Tests
// ========================================

#[test]
fn test_lexer_multiple_spaces() {
    let lexed = lex("set    x    =    5;");
    assert_eq!(lexed.len(), 5);
    assert_eq!(lexed[0].value, Some("set".to_string()));
}

#[test]
fn test_lexer_no_spaces() {
    let lexed = lex("set x=5;");
    assert_eq!(lexed.len(), 5);
    assert_eq!(lexed[2].value, Some("=".to_string()));
}

#[test]
fn test_lexer_newlines() {
    let code = "set x = 1;\nset y = 2;";
    let lexed = lex(code);
    // Line returns should be filtered out
    assert!(lexed.iter().all(|t| t.token_type != TokenType::LRTRN));
    assert_eq!(lexed.len(), 10); // Two complete assignments
}

// ========================================
// Metadata Tests
// ========================================

#[test]
fn test_lexer_line_numbers() {
    let code = "set x = 1;\nset y = 2;";
    let lexed = lex(code);

    // First line tokens should have line number 1
    assert_eq!(lexed[0].meta.line, 1);
    assert_eq!(lexed[4].meta.line, 1);

    // Second line tokens should have line number 2
    assert_eq!(lexed[5].meta.line, 2);
    assert_eq!(lexed[9].meta.line, 2);
}

#[test]
fn test_lexer_char_positions() {
    let code = "set x = 5;";
    let lexed = lex(code);

    // Verify char positions are tracked
    assert_eq!(lexed[0].meta.char, 0); // "set" starts at 0
    assert!(lexed[1].meta.char > 0); // "x" is after "set "
}

// ========================================
// Edge Case Tests
// ========================================

#[test]
fn test_lexer_empty_string() {
    let lexed = lex("");
    assert_eq!(lexed.len(), 0);
}

#[test]
fn test_lexer_only_whitespace() {
    let lexed = lex("   \n  \n   ");
    assert_eq!(lexed.len(), 0);
}

#[test]
fn test_lexer_only_semicolons() {
    let lexed = lex(";;;");
    assert_eq!(lexed.len(), 3);
    assert!(lexed.iter().all(|t| t.token_type == TokenType::ENDL));
}

#[test]
fn test_lexer_comma_handling() {
    // Commas should be removed from identifiers
    let lexed = lex("fn test(a, b, c) {}");
    let params: Vec<_> = lexed
        .iter()
        .filter(|t| t.token_type == TokenType::ID && t.value.is_some())
        .collect();
    // Verify no commas in parameter names
    assert!(params
        .iter()
        .all(|t| !t.value.as_ref().unwrap().contains(",")));
}
