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
/// use csai::lang::TokenStream;
/// let input = String::from("set x = 12;");
/// let lexed = TokenStream::lex(input);
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
/// ├── ID "x"
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
/// Example algorithm:
/// ```rust
/// use crate::lang::{AST, SemanticError};
///
/// fn analyze(ast: &AST) -> Result<(), SemanticError> {
///     // Check for semantic issues, e.g., variable declarations and type correctness.
/// }
/// ```
///
/// ### 4. Code Generation
/// In this stage, the validated AST is transformed into AsmFG code. The AsmFG output is a lower-level, assembly-style language
/// that closely matches the target machine's architecture or a virtual machine.
///
/// For example, the AFG code:
/// ```AFG
/// let x = 42;
/// ```
///
/// Might be translated to:
/// ```AsmFG
/// LOAD_CONST 42
/// STORE "x"
/// ```
///
/// **Future Work:**
/// - Add optimizations during code generation to produce more efficient AsmFG.
/// - Support for more advanced AFG constructs such as loops, functions, and classes.
///
/// ### 5. Optimization (Upcoming Feature)
/// An optimization phase will be added to improve the efficiency of the generated AsmFG. This will include:
/// - Dead code elimination.
/// - Constant folding (e.g., replacing `1 + 1` with `2` at compile time).
/// - Inline expansion and other common compiler optimizations.
///
/// Example Algorithm:
/// ```ignore
/// fn optimize(asm_fg: &mut AsmFG) {
///     // Analyze and remove redundant instructions.
/// }
/// ```
///
/// ## Roadmap
/// - [x] Lexical analysis
/// - [x] AST generation
/// - [ ] Semantic analysis
/// - [ ] Code generation
/// - [ ] Optimization
///
/// By defining these stages, this module ensures a structured approach to compiling AFG into AsmFG, making the process
/// extensible and maintainable.

mod token;
mod ast;
mod semantic;

pub use token::TokenStream;
pub use ast::AST;
pub use semantic::{analyze, SemanticError};