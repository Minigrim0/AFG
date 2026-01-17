/// This module is a compiler from the AFG lang to the AsmFG assembly-like language.
///
/// ## Overview
/// This compiler module is designed to take programs written in the AFG (Automated Fighting Game)
/// language and translate them into AsmFG (Assembly for Fighting Game), an intermediate assembly-like language.
///
/// The goal of this system is to enable the analysis, transformation, and execution of AFG programs through multiple
/// well-defined stages, each responsible for a critical part of the compilation process.
///
/// ## Compilation Stages
///
/// ### 1. Lexical Analysis (Tokenizing)
/// In this stage, the source code is broken up into tokens, which are the smallest meaningful units of the code.
/// For example, in the following AFG code:
///
/// ```AFG
/// let x = 42;
/// ```
///
/// The lexer would produce tokens such as:
/// - `set` (keyword)
/// - `x` (identifier)
/// - `=` (operator)
/// - `42` (literal)
/// - `;` (end line)
///
/// **Module:** [`token`](./token.rs)
/// - Contains functions and data structures for breaking input text into tokens.
/// - Implements pattern matching using rust's builtin pattern matching.
///
/// Example:
/// ```rust
/// use afgcompiler::lexer::parse_source;
///
/// let tokens = parse_source("fn main() { set x = 0; print x; }");
/// ```
///
/// ### 2. Parsing (Abstract Syntax Tree [AST] Generation)
/// The parser takes the list of tokens from the lexer and organizes them into a tree-like structure called the Abstract
/// Syntax Tree (AST). This tree represents the grammatical structure of the AFG code.
///
/// For example, in the same AFG code:
/// ```AFG
/// let x = 42;
/// ```
///
/// The resulting AST would look like:
/// ```text
/// Assignment
/// ├─ ID "x"
/// └─ LIT 42
/// ```
///
/// **Module:** [`ast`](./ast/)
/// - Defines the data structures for representing the AST.
/// - Implements recursive descent parsing to build the AST from tokens.
///
/// ### 3. Semantic Analysis
/// At this stage, the AST is checked for correctness according to the rules of the AFG language.
/// This includes variable scope validation, and ensuring that operations are valid.
///
/// Example issues caught in this phase:
/// - Using an undeclared variable.
/// - Invalid Operation (e.g. trying to assign a value to a literal)
///
/// ### 4. Code Generation
/// In this stage, the validated AST is transformed into Pseudo-AsmFG code. The Pseudo-AsmFG output is a lower-level, assembly-style language
/// that closely matches the target virtual machine's architecture. This language supersets AsmFG and includes additional information such as
/// variable names and labels.
///
/// For example, the AFG code:
/// ```AFG
/// let x = 42;
/// ```
///
/// Might be translated to:
/// ```AsmFG
/// mov @x #42
/// ```
///
/// **Module:** [`asm`](./asm/)
/// - Contains data structures and functions for generating Pseudo-AsmFG code from the AST.
/// - Implements a simple code generation algorithm that maps AST nodes to Pseudo-AsmFG instructions.
///
/// **Future Work:**
/// - Add optimizations during code generation to produce more efficient Pseudo-AsmFG.
///
/// ### 5. Optimization (Upcoming Feature)
/// An optimization phase will be added to improve the efficiency of the generated AsmFG. This will include:
/// - Dead code elimination.
/// - Constant folding (e.g., replacing `1 + 1` with `2` at compile time).
/// - Inline expansion and other common compiler optimizations.
///
/// **Module:** [`optimization`](./optimization/)
///
/// ## Roadmap
/// - [x] Lexical analysis
/// - [x] AST generation
/// - [x] Semantic analysis
/// - [x] Code generation
/// - [ ] Optimization
/// - [X] Stack Allocation
/// By defining these stages, this module ensures a structured approach to compiling AFG into AsmFG, making the process
/// extensible and maintainable.
pub mod allocation;
pub mod ast;
pub mod error;
pub mod labels;
pub mod lexer;
pub mod liveness;
pub mod pasm;
pub mod semantic;

pub mod prelude {
    pub use super::allocation::allocate;
    pub use super::ast::{node::NodeKind, AST};
    pub use super::labels::resolve_labels;
    pub use super::lexer::parse_source;
    pub use super::liveness::PASMProgramWithInterferenceGraph;
    pub use super::pasm::{PASMAllocatedProgram, PASMInstruction, PASMProgram};
    pub use super::semantic::{analyze, SemanticError};
}
