use std::rc::Rc;
use std::cell::RefCell;

use super::block::Block;

type SafeTable = Rc<RefCell<Vec<String>>>;

struct Function {
    name: String,  // Function name
    arguments: Vec<String>, // Locally defined variables through arguments
    block: Block,
}

impl Function {
    // It is assumed this is called after consuming the 'fn' keyword
    pub fn new<T>(tokens: &mut T) -> Result<Self, String>
    where T: Iterator,
        T::Item: AsRef<str>
    {
        let function_name = tokens
            .next()
            .ok_or("No function name found")?
            .as_ref()
            .to_string();
        let args = Self::parse_args(tokens)?;

        Err("noe".to_string())
    }

    fn parse_args<T>(tokens: &mut T) -> Result<Vec<String>, String>
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
