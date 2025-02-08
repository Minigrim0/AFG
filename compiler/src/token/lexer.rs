use super::{types::TokenType, Token};

pub fn lex(text: String) -> Vec<Token> {
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
        return vec![];
    };

    let mut new_tokens = vec![];
    let mut skip_until_newline = false;
    for current_token in token_iter {
        if skip_until_newline {
            if current_token.token_type == TokenType::ENDL {
                last_token = None;
                skip_until_newline = false;
            }
            continue;
        }

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
                ("/", "/") => skip_until_newline = true,
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

    new_tokens
}
