use crate::token::{Token, TokenType};

use super::node::parse_block;

#[test]
pub fn test_new_id_or_literal() {
    let token = Token::new(TokenType::ID, Some("$VelocityY".to_string()), 0, 0);

    let val = parse_block(&mut vec![token].into_iter().peekable());
    assert!(val.is_ok());
}
