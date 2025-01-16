use std::collections::HashMap;
use std::iter::Iterator;
use std::collections::VecDeque;

pub mod block;
pub mod function;
pub mod node;

#[derive(Debug)]
enum TokenType {
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
struct Token {
    ttype: TokenType,
    value:  Option<String>,
}

impl Token {
    pub fn new(ttype: TokenType, value: Option<String>)  -> Self {
        Self {
            ttype,
            value
        }
    }
}

struct TokenStream {
    tokens: VecDeque<Token>
}

impl TokenStream {
    pub fn from_text(text: String) -> Self {
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
        // println!("Pre-tokenization: {:?}", result);

        let tokens = result.into_iter().filter(|t| *t != " ").filter_map(
            |t| match t {
                "fn" | "while" | "let" | "if" | "else" | "return" => Some(Token::new(TokenType::KEYWORD, Some(t.to_string()))),
                "+" | "-" | "*" | "/" | "%" | "<" | "<=" | "==" | "=" | ">=" | ">" => Some(Token::new(TokenType::OP, Some(t.to_string()))),
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
}

fn display_tokenized(tokens: &Vec<Token>) {
    let mut level = 0;
    for token in tokens.iter() {
        match token.ttype {
            TokenType::RPAREN | TokenType::RBRACE => level -= 1,
            _ => {}
        }
        let mut prefix = String::new();
        for _ in 0..(level * 4) {
            prefix.push(' ')
        }
        match &token.value {
            Some(v) => println!("{}{:8?}: {}", prefix, token.ttype, v),
            None => println!("{}{:?}", prefix, token.ttype)
        }
        match token.ttype {
            TokenType::LPAREN | TokenType::LBRACE => level += 1,
            _ => {}
        }
    }
}

fn parse_function(tokens: &mut TokenStream) -> Result<(), String>
{
    if let Some(function_name) = tokens.next() {
        println!("Function name: {}", function_name.value.as_ref().unwrap());
        Ok(())
    } else {
        Err("Missing function name".to_string())
    }
}

pub fn parse_program<S: AsRef<str>>(text: S) -> Result<HashMap<String, node::ASTNode>, String> {
    let mut tokens = TokenStream::from_text(text.as_ref().to_string());

    while let Some(token) = tokens.next() {
        match &token.ttype {
            TokenType::KEYWORD if token.value == Some("fn".to_string()) => {
                parse_function(&mut tokens);
            },
            ttype => return Err(format!("Unexpected token {:?} {:?}", ttype, token.value))
        }
    }

    Ok(HashMap::new())
}
