use super::ASTNodeInfo;
use std::fmt;

use regex::Regex;

// Statement (e.g. assignment, function call. if, while)
#[derive(Debug)]
pub struct Assignment {
    variable: String,
}

impl Assignment {
    pub fn new(variable_name: String) -> Self {
        Self {
            variable: variable_name
        }
    }

    // Parses an expression from an assignment
    pub fn parse_expression<T>(&mut self, tokens: &mut T) -> Result<(), String>
    where
        T: Iterator,
        T::Item: AsRef<str>
    {
        let mut expression = vec![];
        while let Some(token) = tokens.next() {
            match token.as_ref().trim() {
                t if t.ends_with(";") => {
                    expression.push(t[..t.len()-1].to_string());
                    break
                },
                t => expression.push(t.to_string())
            }
        }

        if expression.len() > 3 {
            return Err("Too much tokens for assignment expression, please split operations into multiple assignments".to_string());
        }

        for (id, token) in expression.iter().enumerate() {
            let token: &str = token.as_ref();
            if (id + 1) % 2 == 0 {
                match token {
                    "+" => println!("add"),
                    "-" => println!("sub"),
                    "*" => println!("mult"),
                    "/" => println!("div"),
                    "%" => println!("mod"),
                    t => return Err(format!("Invalid token '{}' expected one of +, -, *, /, %", t))
                }
            } else {
                let var_name = Regex::new(r"[a-zA-Z_]+").unwrap();

                if token.to_string().starts_with("$") {
                    println!("SpecialVar => Literal");
                } else if let Ok(literal) = token.parse::<i32>() {
                    println!("Literal: {}", literal);
                } else if var_name.is_match(token) {
                    println!("identifier")
                } else {
                    return Err(format!("Invalid token '{}', expected identifier, special variable or litteral", token))
                }
            }
        }

        Ok(())
    }
}

impl ASTNodeInfo for Assignment {}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assignment")
    }
}
