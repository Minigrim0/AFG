use std::fmt;

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub char: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize, char: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            char,
        }
    }

    pub fn is_literal(&self) -> bool {
        if self.token_type != TokenType::ID {
            return false;
        }
        self.value.is_some()
            && self
                .value
                .as_ref()
                .and_then(|v| Some(v.parse::<i32>().is_ok()))
                == Some(true)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_type = match self.token_type {
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

        write!(f, "{}", token_type)?;
        if let Some(value) = self.value.clone() {
            write!(f, " = {}", value)?
        }
        Ok(())
    }
}

pub fn lex(text: String) -> impl Iterator<Item = Token> {
    let mut result: Vec<(&str, usize, usize)> = Vec::new();
    let mut last = 0;
    let mut line: usize = 1;
    let mut current_line_start: usize = 0;
    for (index, matched) in text.match_indices(|c| {
        c == ' '
            || c == '('
            || c == ')'
            || c == '{'
            || c == '}'
            || c == '\n'
            || c == ';'
            || c == '['
            || c == ']'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '%'
            || c == '<'
            || c == '='
            || c == '>'
    }) {
        if last != index {
            result.push((&text[last..index], line, last));
        }
        result.push((matched, line, index));
        last = index + matched.len();

        if matched == "\n" {
            line += 1;
            current_line_start = index + 1;
        }
    }
    if last < text.len() {
        result.push((&text[last..], line, last - current_line_start));
    }

    let tokens = result
        .into_iter()
        .filter(|t| t.0 != " ")
        .filter_map(|t| match t.0 {
            "fn" | "while" | "set" | "if" | "else" | "return" | "loop" | "call" | "print" => Some(
                Token::new(TokenType::KEYWORD, Some(t.0.to_string()), t.1, t.2),
            ),
            "+" | "-" | "*" | "/" | "%" | "<" | "=" | ">" => {
                Some(Token::new(TokenType::OP, Some(t.0.to_string()), t.1, t.2))
            }
            "(" => Some(Token::new(TokenType::LPAREN, None, t.1, t.2)),
            ")" => Some(Token::new(TokenType::RPAREN, None, t.1, t.2)),
            "{" => Some(Token::new(TokenType::LBRACE, None, t.1, t.2)),
            "}" => Some(Token::new(TokenType::RBRACE, None, t.1, t.2)),
            "[" => Some(Token::new(TokenType::LBRACKET, None, t.1, t.2)),
            "]" => Some(Token::new(TokenType::RBRACKET, None, t.1, t.2)),
            "//" => Some(Token::new(TokenType::COMMENT, None, t.1, t.2)),
            "\n" | ";" => Some(Token::new(TokenType::ENDL, None, t.1, t.2)),
            " " => None, // Skip whitespaces
            _ => Some(Token::new(
                TokenType::ID,
                Some(t.0.to_string().replace(",", "")),
                t.1,
                t.2,
            )),
        })
        .collect::<Vec<Token>>();

    // Match double character operators
    let mut token_iter = tokens.into_iter();
    let mut last_token = if let Some(token) = token_iter.next() {
        Some(token)
    } else {
        return vec![].into_iter();
    };

    let mut new_tokens = vec![];
    for current_token in token_iter {
        let last_token_value = if let Some(token) = &last_token {
            if let Some(value) = &token.value {
                value.as_str()
            } else {
                ""
            }
        } else {
            last_token = Some(current_token);
            continue;
        };

        if let Some(current_value) = &current_token.value {
            match (last_token_value, current_value.as_str()) {
                (">", "=") => {
                    new_tokens.push(Token::new(TokenType::OP, Some(">=".to_string()), 0, 0));
                    last_token = None;
                }
                ("<", "=") => {
                    new_tokens.push(Token::new(TokenType::OP, Some("<=".to_string()), 0, 0));
                    last_token = None;
                }
                ("=", "=") => {
                    new_tokens.push(Token::new(TokenType::OP, Some("==".to_string()), 0, 0));
                    last_token = None;
                }
                ("!", "=") => {
                    new_tokens.push(Token::new(TokenType::OP, Some("!=".to_string()), 0, 0));
                    last_token = None;
                }
                _ => {
                    if let Some(token) = last_token {
                        new_tokens.push(token);
                    }
                    last_token = Some(current_token);
                }
            }
        } else {
            if let Some(token) = last_token {
                new_tokens.push(token);
            }
            last_token = Some(current_token);
        }
    }

    new_tokens.into_iter()
}
