use super::node::CodeBlock;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: CodeBlock,
}

impl Function {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: vec![],
            content: vec![],
        }
    }
}
