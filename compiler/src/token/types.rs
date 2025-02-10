use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    RBRACKET, // Indicates a memory access (with offset)
    LBRACKET,
    KEYWORD,
    OP,
    COMMENT,
    ENDL,
    ID,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::LBRACE => "LBRACE",
            TokenType::RBRACE => "RBRACE",
            TokenType::RBRACKET => "RBRACKET",
            TokenType::LBRACKET => "LBRACKET",
            TokenType::KEYWORD => "KEYWORD",
            TokenType::OP => "OP",
            TokenType::COMMENT => "COMMENT",
            TokenType::ENDL => "ENDL",
            TokenType::ID => "ID",
        };
        write!(f, "{}", repr)
    }
}
