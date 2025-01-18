use std::collections::HashSet;

use node::ASTBlockNode;

pub mod token;
pub mod block;
pub mod function;
pub mod node;
mod utils;
pub mod parser;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: Vec<ASTBlockNode>
}

impl Function {
    /// Variables are assigned in the memory directly to avoid register allocation
    /// Context represents up to what point the memory has been used by the caller function, letting the callee work with
    /// addresses above. Once the call
    pub fn to_asm(&self) -> Result<(), String> {
        let function_registers = ["FPA", "FPB", "FPC", "FPD"];

        println!("; Function {}", self.name);
        for (idx, _) in self.parameters.iter().enumerate() {
            if idx < function_registers.len() {
                // Pop into the registers
                println!("pop '{}", function_registers[idx]);
            } else {
                return Err(format!("Too much arguments for function {}", self.name))
            }
        }

        Ok(())
    }

    /// Returns the amount of addresses required by this function to work properly.
    /// This represents the amount of variable this function has.
    /// RECURSION WILL RESULT IN AN UNDEFINED BEHAVIOUR -> Variables are staticallly
    /// saved in memory
    pub fn get_size(&self) -> i32 {
        let mut total = HashSet::new();
        for node in self.content.iter() {
            total.extend(node.get_variables.iter());
        }
    }
}

#[cfg(test)]
pub mod test;
