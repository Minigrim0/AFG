use super::{types::TokenType, Token};
use super::lex;

#[test]
pub fn test_is_litteral() {
    assert!(Token::new(TokenType::ID, Some("32".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::COMMENT, Some("32".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::ID, Some("$VelocityY".to_string()), 0, 0).is_literal());

    assert!(!Token::new(TokenType::ID, Some("michel".to_string()), 0, 0).is_literal());
}

#[test]
pub fn test_lexer() {
    let text  = "set test = -1;";
    let lexed = lex(text);
    assert_eq!(lexed.len(), 5, "Amount of tokens lexed is not correct");
    assert!(lexed[0].token_type == TokenType::KEYWORD, "First token is not a keyword");
    assert!(lexed[1].token_type == TokenType::ID, "Second token is not an identifier");
    assert!(lexed[2].token_type == TokenType::OP, "Third token is not an operator");
    assert!(lexed[3].token_type == TokenType::ID && lexed[3].is_literal(), "Fourth token is not a literal");
    assert!(lexed[4].token_type == TokenType::ENDL, "Fifth token is not an endline");
}
