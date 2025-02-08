use super::{types::TokenType, Token};

#[test]
pub fn test_is_litteral() {
    assert!(Token::new(TokenType::ID, Some("32".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::COMMENT, Some("32".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::ID, Some("$VelocityY".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::ID, Some("michel".to_string()), 0, 0).is_literal());
}
