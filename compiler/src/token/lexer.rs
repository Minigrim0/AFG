use super::{types::TokenType, Token};

/// Transforms a sets of two operators into a single operator
/// This transforms the token list from:
/// `OP(>), OP(=)`
/// into:
/// `OP(>=)`
fn match_double_operators(tokens: Vec<Token>) -> Vec<Token> {
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
            if current_token.token_type == TokenType::LRTRN {
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
    if let Some(token) = last_token {
        new_tokens.push(token);
    }

    new_tokens
}

/// Transforms a set (NON-ID, OP('-'), ID[is_literal]) into a set (NON-ID, ID[is_literal,negative])
/// This transforms the token list from:
/// `KEYWORD(set), ID(test), OP(=), OP(-), ID(1), ENDL(;)`
/// into:
/// `KEYWORD(set), ID(test), OP(=), ID(-1), ENDL(;)`
fn lex_negative_immediates(tokens: Vec<Token>) -> Vec<Token> {
    let mut last_token: &Token = if let Some(token) = tokens.first() {
        token
    } else {
        return tokens;
    };

    let mut negate_next_token = false;
    let mut new_tokens: Vec<Token> = vec![last_token.clone()];

    for token in tokens.iter().skip(1) {
        if token.token_type == TokenType::OP && token.value == Some("-".to_string()) && &last_token.token_type != &TokenType::ID {
            negate_next_token = true;
        } else {
            if negate_next_token {
                if let Some(value) = &token.value {
                    if let Ok(value) = value.parse::<i32>() {
                        let new_token = Token::new(TokenType::ID, Some((-value).to_string()), 0, 0);
                        new_tokens.push(new_token);
                    }
                }
                negate_next_token = false;
            } else {
                new_tokens.push(token.clone());
            }
        }
        last_token = token;
    }

    new_tokens
}

/// Remove line return tokens from the list of tokens
/// Line return tokens are only useful to remove comments from the code
fn remove_line_returns(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().filter(|t| t.token_type != TokenType::LRTRN).cloned().collect()
}

pub fn lex<S: AsRef<str>>(text: S) -> Vec<Token> {
    let text = text.as_ref().to_string();
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
            ";" => Some(Token::new(TokenType::ENDL, None, t.1, t.2)),
            "\n" => Some(Token::new(TokenType::LRTRN, None, t.1, t.2)),
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
    let tokens = match_double_operators(tokens);

    // Match negative immediates
    let tokens = lex_negative_immediates(tokens);

    remove_line_returns(tokens)
}
