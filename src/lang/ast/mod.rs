pub mod node;
pub mod parser;

use node::CodeBlock;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub content: CodeBlock
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
}

#[cfg(test)]
mod tests;
