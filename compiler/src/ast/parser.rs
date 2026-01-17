use crate::error::{TokenError, TokenErrorType};
use crate::lexer::token::{
    ComparisonKind, KeywordKind, OperationKind, SymbolKind, Token, TokenKind, TokenLocation,
};

use super::function::Function;
use super::node::{CodeBlock, ComparisonType, Node, NodeKind, OperationType};
use super::AST;

use std::collections::HashMap;

/// A recursive descent parser using token slice with index for efficient parsing.
///
/// This parser uses a slice-based approach which provides:
/// - Multiple token lookahead
/// - Easy backtracking via save/restore
/// - Clean error messages with exact token locations
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    // ========== Core Navigation Methods ==========

    /// Look at current token without consuming
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    /// Look n tokens ahead (0 = current token)
    fn _peek_nth(&self, n: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.pos + n)
    }

    /// Get current token's location for error messages
    fn current_location(&self) -> Option<TokenLocation> {
        self.peek().map(|t| t.location.clone())
    }

    /// Consume and return current token
    fn advance(&mut self) -> Option<&Token<'a>> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    /// Check if at end of tokens
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    /// Save position for backtracking
    fn _save(&self) -> usize {
        self.pos
    }

    /// Restore position for backtracking
    fn _restore(&mut self, pos: usize) {
        self.pos = pos;
    }

    // ========== Token Matching Methods ==========

    /// Check if current token matches a specific kind
    fn _check(&self, kind: &TokenKind) -> bool {
        self.peek()
            .map(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
            .unwrap_or(false)
    }

    /// Check if current token is a specific keyword
    fn check_keyword(&self, keyword: KeywordKind) -> bool {
        matches!(self.peek(), Some(Token { kind: TokenKind::Keyword(kw), .. }) if *kw == keyword)
    }

    /// Check if current token is a specific symbol
    fn check_symbol(&self, symbol: SymbolKind) -> bool {
        matches!(self.peek(), Some(Token { kind: TokenKind::Symbol(s), .. }) if *s == symbol)
    }

    /// Consume token if it matches, return whether it matched
    fn _match_keyword(&mut self, keyword: KeywordKind) -> bool {
        if self.check_keyword(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume token if it matches symbol
    fn match_symbol(&mut self, symbol: SymbolKind) -> bool {
        if self.check_symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Expect a specific symbol, error if not found
    fn expect_symbol(&mut self, symbol: SymbolKind) -> Result<(), TokenError> {
        if self.match_symbol(symbol.clone()) {
            Ok(())
        } else {
            Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!("Expected symbol {:?}", symbol),
                self.current_location(),
            ))
        }
    }

    /// Expect a specific keyword, error if not found
    fn _expect_keyword(&mut self, keyword: KeywordKind) -> Result<(), TokenError> {
        if self._match_keyword(keyword.clone()) {
            Ok(())
        } else {
            Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!("Expected keyword {:?}", keyword),
                self.current_location(),
            ))
        }
    }

    /// Skip line breaks
    fn skip_line_breaks(&mut self) {
        while self.check_symbol(SymbolKind::LineBreak) {
            self.advance();
        }
    }

    // ========== Parsing Methods ==========

    /// Parse a complete program (entry point)
    pub fn parse_program(&mut self) -> Result<AST, TokenError> {
        let mut functions = HashMap::new();

        while !self.is_at_end() {
            self.skip_line_breaks();

            if self.is_at_end() {
                break;
            }

            if self.check_keyword(KeywordKind::Fn) {
                self.advance(); // consume 'fn'
                let function = self.parse_function()?;
                functions.insert(function.name.clone(), function);
            } else {
                return Err(TokenError::new(
                    TokenErrorType::UnexpectedToken,
                    format!(
                        "Expected 'fn' keyword, found {:?}",
                        self.peek().map(|t| &t.kind)
                    ),
                    self.current_location(),
                ));
            }
        }

        Ok(AST { functions })
    }

    /// Parse a function definition
    fn parse_function(&mut self) -> Result<Function, TokenError> {
        // Parse function name
        let name = self.parse_identifier()?;

        // Parse parameters
        self.expect_symbol(SymbolKind::LeftParen)?;
        let parameters = self.parse_parameter_list()?;
        self.expect_symbol(SymbolKind::RightParen)?;

        // Parse function body
        self.expect_symbol(SymbolKind::LeftBrace)?;
        let content = self.parse_block()?;

        Ok(Function {
            name,
            parameters,
            content,
        })
    }

    /// Parse a comma-separated parameter list
    fn parse_parameter_list(&mut self) -> Result<Vec<String>, TokenError> {
        let mut params = Vec::new();

        while !self.check_symbol(SymbolKind::RightParen) && !self.is_at_end() {
            if let Some(Token {
                kind: TokenKind::Ident(name),
                ..
            }) = self.peek()
            {
                params.push(name.to_string());
                self.advance();
            }
            // Skip commas (we're lenient here)
            if let Some(Token {
                kind: TokenKind::Symbol(SymbolKind::Separator),
                ..
            }) = self.peek()
            {
                self.advance(); // Consume comma
            }
        }

        Ok(params)
    }

    /// Parse a block of statements (inside braces)
    fn parse_block(&mut self) -> Result<CodeBlock, TokenError> {
        let mut statements = Vec::new();

        while !self.check_symbol(SymbolKind::RightBrace) && !self.is_at_end() {
            self.skip_line_breaks();

            if self.check_symbol(SymbolKind::RightBrace) {
                break;
            }

            let stmt = self.parse_statement()?;
            statements.push(Box::new(stmt));
        }

        self.expect_symbol(SymbolKind::RightBrace)?;
        Ok(statements)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> Result<Node, TokenError> {
        self.skip_line_breaks();

        let result = match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Keyword(KeywordKind::Set)) => {
                self.advance();
                self.parse_assignment()
            }
            Some(TokenKind::Keyword(KeywordKind::While)) => {
                self.advance();
                self.parse_while()
            }
            Some(TokenKind::Keyword(KeywordKind::If)) => {
                self.advance();
                self.parse_if()
            }
            Some(TokenKind::Keyword(KeywordKind::Loop)) => {
                self.advance();
                self.parse_loop()
            }
            Some(TokenKind::Keyword(KeywordKind::Return)) => {
                self.advance();
                self.parse_return()
            }
            Some(TokenKind::Keyword(KeywordKind::Call)) => {
                self.advance();
                self.parse_function_call()
            }
            Some(TokenKind::Keyword(KeywordKind::Print)) => {
                self.advance();
                self.parse_print()
            }
            Some(kind) => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!("Unexpected token in statement: {:?}", kind),
                self.current_location(),
            )),
            None => Err(TokenError::new(
                TokenErrorType::UnexpectedEndOfStream,
                "Unexpected end of input",
                None,
            )),
        };

        // Consume trailing line break if present
        self.match_symbol(SymbolKind::LineBreak);

        result
    }

    /// Parse an assignment: set <ident> = <expr>
    fn parse_assignment(&mut self) -> Result<Node, TokenError> {
        let lparam = self.parse_primary()?;

        // Expect '='
        if !matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::Op(OperationKind::Assign),
                ..
            })
        ) {
            return Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                "Expected '=' in assignment",
                self.current_location(),
            ));
        }
        self.advance();

        let rparam = self.parse_expression()?;

        Ok(Node::new(NodeKind::Assignment {
            lparam: Box::new(lparam),
            rparam: Box::new(rparam),
        }))
    }

    /// Parse a while loop: while <condition> { <block> }
    fn parse_while(&mut self) -> Result<Node, TokenError> {
        let condition = self.parse_comparison()?;

        self.expect_symbol(SymbolKind::LeftBrace)?;
        let content = self.parse_block()?;

        Ok(Node::new(NodeKind::WhileLoop {
            condition: Box::new(condition),
            content,
        }))
    }

    /// Parse an if statement: if <condition> { <block> }
    fn parse_if(&mut self) -> Result<Node, TokenError> {
        let condition = self.parse_comparison()?;

        self.expect_symbol(SymbolKind::LeftBrace)?;
        let content = self.parse_block()?;

        Ok(Node::new(NodeKind::IfCondition {
            condition: Box::new(condition),
            content,
        }))
    }

    /// Parse a loop: loop { <block> }
    fn parse_loop(&mut self) -> Result<Node, TokenError> {
        self.expect_symbol(SymbolKind::LeftBrace)?;
        let content = self.parse_block()?;

        Ok(Node::new(NodeKind::Loop { content }))
    }

    /// Parse a return statement: return [<expr>]
    fn parse_return(&mut self) -> Result<Node, TokenError> {
        if self.check_symbol(SymbolKind::LineBreak)
            || self.check_symbol(SymbolKind::RightBrace)
            || self.is_at_end()
        {
            Ok(Node::new(NodeKind::Return {
                value: Box::new(Node::new(NodeKind::Litteral { value: 0 })),
            }))
        } else {
            let value = self.parse_primary()?;
            Ok(Node::new(NodeKind::Return {
                value: Box::new(value),
            }))
        }
    }

    /// Parse a function call: <ident>(<args>)
    fn parse_function_call(&mut self) -> Result<Node, TokenError> {
        let fun_call_location = self.current_location();
        let function_name = self.parse_identifier()?;

        self.expect_symbol(SymbolKind::LeftParen)?;

        let mut parameters = Vec::new();
        while !self.check_symbol(SymbolKind::RightParen) && !self.is_at_end() {
            let param = self.parse_primary()?;
            parameters.push(Box::new(param));

            // Skip comma separator if present
            if self.check_symbol(SymbolKind::Separator) {
                self.advance();
            }
        }

        self.expect_symbol(SymbolKind::RightParen)?;

        match fun_call_location {
            Some(location) => Ok(Node::with_span(
                NodeKind::new_fun_call(function_name, parameters),
                location,
            )),
            None => Ok(Node::new(NodeKind::new_fun_call(function_name, parameters))),
        }
    }

    /// Parse a print statement: print <expr>
    fn parse_print(&mut self) -> Result<Node, TokenError> {
        let value = self.parse_primary()?;
        Ok(Node::new(NodeKind::Print {
            value: Box::new(value),
        }))
    }

    /// Parse a comparison expression: <expr> <cmp_op> <expr>
    fn parse_comparison(&mut self) -> Result<Node, TokenError> {
        let lparam = self.parse_primary()?;

        if let Some(Token {
            kind: TokenKind::Comp(cmp),
            location,
        }) = self.peek()
        {
            let comparison = match cmp {
                ComparisonKind::GreaterThan => ComparisonType::GT,
                ComparisonKind::GreaterThanOrEqual => ComparisonType::GE,
                ComparisonKind::Equal => ComparisonType::EQ,
                ComparisonKind::NotEqual => ComparisonType::DIFF,
                ComparisonKind::LessThanOrEqual => ComparisonType::LE,
                ComparisonKind::LessThan => ComparisonType::LT,
            };
            let loc = location.clone();
            self.advance();

            let rparam = self.parse_primary()?;

            Ok(Node::with_span(
                NodeKind::Comparison {
                    lparam: Box::new(lparam),
                    rparam: Box::new(rparam),
                    comparison,
                },
                loc,
            ))
        } else {
            // No comparison operator, just return the primary
            Ok(lparam)
        }
    }

    /// Parse an expression (handles operators)
    fn parse_expression(&mut self) -> Result<Node, TokenError> {
        let left = self.parse_primary()?;

        // Check for binary operator
        if let Some(Token {
            kind: TokenKind::Op(op),
            location,
        }) = self.peek()
        {
            // Clone location to free mut ref
            let location = location.clone();

            if *op != OperationKind::Assign {
                let operation = match op {
                    OperationKind::Add => OperationType::Addition,
                    OperationKind::Subtract => OperationType::Substraction,
                    OperationKind::Multiply => OperationType::Multiplication,
                    OperationKind::Divide => OperationType::Division,
                    OperationKind::Modulo => OperationType::Modulo,
                    OperationKind::Assign => unreachable!(),
                };
                self.advance();

                let right = self.parse_primary()?;

                return Ok(Node::with_span(
                    NodeKind::Operation {
                        lparam: Box::new(left),
                        rparam: Box::new(right),
                        operation,
                    },
                    location,
                ));
            }
        }

        // Check if this is a function call (identifier followed by paren)
        if let NodeKind::Identifier { name } = &left.kind {
            if self.check_symbol(SymbolKind::LeftParen) {
                self.advance(); // consume '('
                let mut parameters = Vec::new();
                while !self.check_symbol(SymbolKind::RightParen) && !self.is_at_end() {
                    let param = self.parse_primary()?;
                    parameters.push(Box::new(param));

                    // Skip comma separator if present
                    if self.check_symbol(SymbolKind::Separator) {
                        self.advance();
                    }
                }
                self.expect_symbol(SymbolKind::RightParen)?;

                return Ok(match left.span {
                    Some(span) => Node::with_span(
                        NodeKind::new_fun_call(name.clone(), parameters),
                        span.clone(),
                    ),
                    None => Node::new(NodeKind::new_fun_call(name.clone(), parameters)),
                });
            }
        }

        Ok(left)
    }

    /// Parse a primary expression (identifier, literal, or parenthesized expression)
    fn parse_primary(&mut self) -> Result<Node, TokenError> {
        match self.peek() {
            Some(Token {
                kind: TokenKind::Literal(value),
                location,
            }) => {
                let location = location.clone();
                let value: i32 = value.replace("_", "").parse().map_err(|_| {
                    TokenError::new(
                        TokenErrorType::ParseError,
                        format!("Invalid integer literal: {}", value),
                        self.current_location(),
                    )
                })?;
                self.advance();
                Ok(Node::with_span(NodeKind::Litteral { value }, location))
            }
            Some(Token {
                kind: TokenKind::Ident(name),
                location,
            }) => {
                let name = name.to_string();
                let location = location.clone();
                self.advance();

                // Create base node: MemoryValue for $identifier, Identifier otherwise
                let base_node = if name.starts_with('$') {
                    Node::with_span(
                        NodeKind::MemoryValue {
                            name: name[1..].to_string(),
                        },
                        location.clone(),
                    )
                } else {
                    Node::with_span(NodeKind::Identifier { name }, location.clone())
                };

                // Check for array access: ident[index] or $ident[index]
                if self.check_symbol(SymbolKind::LeftBracket) {
                    self.advance();
                    let offset = self.parse_primary()?;
                    self.expect_symbol(SymbolKind::RightBracket)?;

                    return Ok(Node::with_span(
                        NodeKind::new_mem_offset(base_node, offset),
                        location.clone(),
                    ));
                }

                Ok(base_node)
            }
            Some(Token {
                kind: TokenKind::Symbol(SymbolKind::LeftParen),
                ..
            }) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect_symbol(SymbolKind::RightParen)?;
                Ok(expr)
            }
            // Handle unary minus for negative numbers
            Some(Token {
                kind: TokenKind::Op(OperationKind::Subtract),
                location,
            }) => {
                let location = location.clone();
                self.advance(); // consume '-'
                let inner = self.parse_primary()?;
                match inner.kind {
                    NodeKind::Litteral { value } => Ok(Node::with_span(
                        NodeKind::Litteral { value: -value },
                        location,
                    )),
                    _ => Err(TokenError::new(
                        TokenErrorType::UnexpectedToken,
                        "Unary minus only supported for literals",
                        self.current_location(),
                    )),
                }
            }
            Some(token) => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                format!("Expected expression, found {:?}", token.kind),
                self.current_location(),
            )),
            None => Err(TokenError::new(
                TokenErrorType::UnexpectedEndOfStream,
                "Unexpected end of input while parsing expression",
                None,
            )),
        }
    }

    /// Parse an identifier and return its name
    fn parse_identifier(&mut self) -> Result<String, TokenError> {
        match self.peek() {
            Some(Token {
                kind: TokenKind::Ident(name),
                ..
            }) => {
                let name = name.to_string();
                self.advance();
                Ok(name)
            }
            Some(Token { location, .. }) => Err(TokenError::new(
                TokenErrorType::UnexpectedToken,
                "Expected identifier",
                Some(location.clone()),
            )),
            None => Err(TokenError::new(
                TokenErrorType::UnexpectedEndOfStream,
                "Unexpected end of input while parsing identifier",
                None,
            )),
        }
    }
}
