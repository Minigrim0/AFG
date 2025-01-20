use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    KEYWORD,
    OP,
    COMMENT,
    ENDL,
    ID,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value:  Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>) -> Self {
        Self {
            token_type,
            value
        }
    }

    pub fn is_literal(&self) -> bool {
        if self.token_type != TokenType::ID {
            return false;
        }
        self.value.is_some() && self.value.as_ref().and_then(|v| Some(v.parse::<i32>().is_ok())) == Some(true)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_type = match self.token_type {
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::LBRACE => "LBRACE",
            TokenType::RBRACE => "RBRACE",
            TokenType::KEYWORD => "KEYWORD",
            TokenType::OP => "OP",
            TokenType::COMMENT => "COMMENT",
            TokenType::ENDL => "ENDL",
            TokenType::ID => "ID",
        };

        write!(f, "{}", token_type)?;
        if let Some(value) = self.value.clone() {
            write!(f, " = {}", value)?
        }
        Ok(())
    }
}

pub struct TokenStream {
    tokens: VecDeque<Token>
}

impl TokenStream {
    pub fn from_vec(token_vec: Vec<Token>) -> Self {
        TokenStream {
            tokens: VecDeque::from(token_vec)
        }
    }

    pub fn lex(text: String) -> Self {
        let mut result: Vec<&str> = Vec::new();
        let mut last = 0;
        for (index, matched) in
            text.match_indices(|c| c == ' ' || c == '(' || c == ')' || c == '{' || c == '}' || c == '\n' || c == ';')
        {
            if last != index {
                result.push(&text[last..index]);
            }
            result.push(matched);
            last = index + matched.len();
        }
        if last < text.len() {
            result.push(&text[last..]);
        }

        let tokens = result.into_iter().filter(|t| *t != " ").filter_map(
            |t| match t {
                "fn" | "while" | "set" | "if" | "else" | "return" | "loop" | "call" => Some(Token::new(TokenType::KEYWORD, Some(t.to_string()))),
                "+" | "-" | "*" | "/" | "%" | "<" | "<=" | "==" | "!=" | "=" | ">=" | ">" => Some(Token::new(TokenType::OP, Some(t.to_string()))),
                "(" => Some(Token::new(TokenType::LPAREN, None)),
                ")" => Some(Token::new(TokenType::RPAREN, None)),
                "{" => Some(Token::new(TokenType::LBRACE, None)),
                "}" => Some(Token::new(TokenType::RBRACE, None)),
                "//" => Some(Token::new(TokenType::COMMENT, None)),
                "\n" | ";" => Some(Token::new(TokenType::ENDL, None)),
                " " => None,  // Skip whitespaces
                t => Some(Token::new(TokenType::ID, Some(t.to_string())))
            }
        ).collect::<Vec<Token>>();

        TokenStream {
            tokens: VecDeque::from(tokens)
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn get_until(&mut self, token_type: TokenType) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.tokens.pop_front() {
            let is_end_token = token.token_type == token_type;
            tokens.push(token);
            if is_end_token {
                break;
            }
        }
        tokens
    }
}

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.tokens {
            writeln!(f, "{}", token)?;
        }
        Ok(())
    }
}
