use crate::lang::token::{Token, TokenType, TokenStream};

use super::node::parse_block;

#[test]
pub fn test_new_id_or_literal() {
    let token = Token::new(TokenType::ID, Some("$VelocityY".to_string()));

    let val = parse_block(&mut TokenStream::from_vec(vec![token]));
    assert!(val.is_ok());
}
