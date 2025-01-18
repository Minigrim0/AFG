use super::token::{Token, TokenType, TokenStream};
use super::node::ASTBlockNode;

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

#[test]
pub fn test_parse_while() {
    let tokens = vec![
        Token::new(TokenType::ID, Some("$VelocityY".to_string())),
        Token::new(TokenType::OP, Some("<".to_string())),
        Token::new(TokenType::ID, Some("100".to_string())),
        Token::new(TokenType::LBRACE, None),
        Token::new(TokenType::RBRACE, None),
    ];

    let while_expr = ASTBlockNode::parse_while(&mut TokenStream::from_vec(tokens));
    assert!(while_expr.is_ok(), "While expression parsing returned an error: {}", while_expr.err().unwrap());
}
