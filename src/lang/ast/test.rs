use super::token::{Token, TokenType, TokenStream};

#[test]
pub fn test_is_litteral() {
    assert!(Token::new(TokenType::ID, Some("32".to_string()))
        .is_litteral());

    assert!(!Token::new(TokenType::COMMENT, Some("32".to_string()))
        .is_litteral());

    assert!(!Token::new(TokenType::ID, Some("$VelocityY".to_string()))
        .is_litteral());

    assert!(!Token::new(TokenType::ID, Some("michel".to_string()))
        .is_litteral());
}
