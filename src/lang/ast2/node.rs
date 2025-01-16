use std::{
    cell::RefCell,
    rc::{Rc, Weak}
};

type SafeTable = Rc<RefCell<Vec<String>>>;

#[derive(Debug, Default)]
pub enum NodeType {
    CONST,     // For interger and string constants. As a special case, if the constant is NULL, the type is set to void.
    ID,        // user-defined type 	For all variable literals.
    PLUS,      //
    MINUS,     //
    MUL,       //
    DIV,       // For arithmetic operators '+', '-', '*', '/', '%'. 'ptr1' and 'ptr2' must be of type int and set to the AST's of the left and the right operands respectively.
    LE,        // For relational operators '>', '<', '>=', '<='. 'ptr1' and 'ptr2' must be of type int and set to the AST's of the left and the right operands respectively.
    GT,        //
    LT,        //
    GE,        //
    EQ,        //
    NE,        // For relational operator '==' or '!='. 'ptr1' and 'ptr2' must be set to AST of the left and the right operands respectively and both must be of the same type.
    IF,        // For the conditional construct 'if'. 'ptr1' must be set to the AST of the logical expression and must be of type 'boolean', 'ptr2' must be set to the AST of list of statements corresponding to the 'then part' and 'ptr3' must be set to the AST of list of statements corresponding to the 'else part'.
    WHILE,     // For conditional construct 'while'. 'ptr1' is set to the conditional logical expression and 'ptr2' is set to AST of list of statements under the body of while construct.
    READ,      // For input statement 'read', 'ptr1' must have nodetype ID or FIELD and type of 'ptr1' must be either 'int' or 'str'.
    WRITE,     // For output statement 'write', 'ptr1' must be of type 'int' and 'str' and must be set to AST of the expression whose value is to be written to the standard output.
    ASGN,      // For assignment statement (<var> = <expr>). 'ptr1' must be set to an AST of nodetype ID or FIELD and 'ptr2' must be set to an AST of expression whose value will be assigned to lvalue given by 'ptr1'. The types of the variable and the expression must match.
    SLIST,     // To form a tree with multiple statements. The execution semantics is that the sequence of statements represented by the left subtree 'ptr1' must be evaluated before evaluating 'ptr2'.
    BODY,      // For body of a function, type indicates the return type of the function. This is created when the definition of a function is processed.
    RET,       // For return statement of a function.
    #[default]
    FUNCTION,  // For function calls. The type must be same as the return type of the function. The field 'arglist' must be set to list of arguments for the function.
}

#[derive(Debug, Default)]
pub struct ASTNode {
    node_type: NodeType,
    name: String,
    value: Option<i32>,
    arglist: Vec<String>,
    left: Weak<RefCell<ASTNode>>,
    right: Weak<RefCell<ASTNode>>,
    globals_table: Option<SafeTable>,
    local_table: Option<SafeTable>,
}

impl ASTNode {
    pub fn new() -> Self {
        Self::default()
    }
}
