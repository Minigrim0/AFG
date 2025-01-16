use super::ASTNodeInfo;
use std::fmt;

#[derive(Debug)]
// Function declaration (e.g. fn foo() {})
pub struct Function {
    identifier: String,
    parameters: Vec<String>,
}

impl Function {
    pub fn new<T>(identifier: String, tokens: &mut T) -> Result<Self, String>
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        Ok(Self {
            identifier,
            parameters: Self::parse_fn_args(tokens)?,
        })
    }

    fn parse_fn_args<T>(tokens: &mut T) -> Result<Vec<String>, String>
    where
        T: Iterator,
        T::Item: AsRef<str>,
    {
        let mut args = Vec::new();

        while let Some(arg) = tokens.next() {
            match arg.as_ref() {
                "(" => continue,
                " " => continue,
                ")" => return Ok(args),
                arg => args.push(arg.to_string().replace(',', "")),
            }
        }
        Err("Unclosed Parenthese for function arguments".to_string())
    }
}

impl ASTNodeInfo for Function {}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function '{}'", self.identifier)
    }
}
