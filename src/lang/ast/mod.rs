use std::collections::HashMap;
use std::iter::Iterator;
use std::collections::VecDeque;

use node::ASTBlockNode;

pub mod block;
pub mod function;
pub mod node;

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

pub struct TokenStream {
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
                "fn" | "while" | "set" | "if" | "else" | "return" => Some(Token::new(TokenType::KEYWORD, Some(t.to_string()))),
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

    pub fn get_until(&mut self, token_type: TokenType) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.tokens.pop_front() {
            let is_end_token = token.ttype == token_type;
            tokens.push(token);
            if is_end_token {
                break;
            }
        }
        tokens
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

fn parse_block(stream: &mut TokenStream) -> Result<Vec<ASTBlockNode>, String> {
    let mut block_tree = vec![];
    while let Some(token) = stream.next() {
        match token.ttype {
            TokenType::KEYWORD if token.value == Some("set".to_string()) => {
                let assignment = ASTBlockNode::parse_assignment(stream)?;
                block_tree.push(assignment);
            },
            TokenType::KEYWORD if token.value == Some("while".to_string()) => {
                let while_block = ASTBlockNode::parse_while(stream)?;
                // Parse while block content here
                block_tree.push(while_block);
            }
            TokenType::ENDL => {
                continue
            }
            TokenType::RBRACE => {
                break;
            },
            t => println!("Unexpected or unhandled token: {:?}", t)
        }
    }

    Ok(block_tree)
}

fn parse_function(stream: &mut TokenStream) -> Result<(), String> {
    if let Some(function_name) = stream.next() {
        if stream.next().and_then(|t| Some(t.ttype)) != Some(TokenType::LPAREN) {
            return Err("Unexpected token after function name, expected (".to_string());
        }

        let mut args = stream.get_until(TokenType::RPAREN);
        args.pop();  // Pop the RPAREN

        if stream.next().and_then(|t| Some(t.ttype)) != Some(TokenType::LBRACE) {
            return Err("Unexpected token after function name, expected {".to_string());
        }

        let funtion_block = parse_block(stream)?;

        Ok(())
    } else {
        Err("Missing function name".to_string())
    }
}

pub fn parse_program<S: AsRef<str>>(text: S) -> Result<HashMap<String, node::ASTBlockNode>, String> {
    let mut tokens = TokenStream::from_text(text.as_ref().to_string());

    while let Some(token) = tokens.next() {
        match &token.ttype {
            TokenType::KEYWORD if token.value == Some("fn".to_string()) => {
                parse_function(&mut tokens)?;
            },
            TokenType::ENDL => continue,
            ttype => return Err(format!("Unexpected token {:?} {:?}", ttype, token.value))
        }
    }

    Ok(HashMap::new())
}
