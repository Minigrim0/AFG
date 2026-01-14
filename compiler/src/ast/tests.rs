use super::node::{Node, ComparisonType, OperationType};
use super::AST;
use crate::token::lex;

// ========================================
// Helper Functions
// ========================================

fn parse_program(code: &str) -> Result<AST, crate::error::TokenError> {
    let tokens = lex(code);
    AST::parse(&mut tokens.into_iter().peekable())
}

// ========================================
// Function Parsing Tests
// ========================================

#[test]
fn test_parse_simple_function() {
    let code = "fn main() {}";
    let ast = parse_program(code).unwrap();
    assert!(ast.functions.contains_key("main"));
    assert_eq!(ast.functions["main"].parameters.len(), 0);
    assert_eq!(ast.functions["main"].content.len(), 0);
}

#[test]
fn test_parse_function_with_parameters() {
    let code = "fn add(a, b) {}";
    let ast = parse_program(code).unwrap();
    assert!(ast.functions.contains_key("add"));
    assert_eq!(ast.functions["add"].parameters.len(), 2);
    assert_eq!(ast.functions["add"].parameters[0], "a");
    assert_eq!(ast.functions["add"].parameters[1], "b");
}

#[test]
fn test_parse_function_with_many_parameters() {
    let code = "fn multi(a, b, c, d, e) {}";
    let ast = parse_program(code).unwrap();
    assert_eq!(ast.functions["multi"].parameters.len(), 5);
}

#[test]
fn test_parse_multiple_functions() {
    let code = "fn main() {} fn helper() {}";
    let ast = parse_program(code).unwrap();
    assert_eq!(ast.functions.len(), 2);
    assert!(ast.functions.contains_key("main"));
    assert!(ast.functions.contains_key("helper"));
}

// ========================================
// Assignment Parsing Tests
// ========================================

#[test]
fn test_parse_simple_assignment() {
    let code = "fn main() { set x = 5; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 1);

    match &*content[0] {
        Node::Assignment { lparam, rparam } => {
            match &**lparam {
                Node::Identifier { name } => assert_eq!(name, "x"),
                _ => panic!("Expected identifier on left side"),
            }
            match &**rparam {
                Node::Litteral { value } => assert_eq!(*value, 5),
                _ => panic!("Expected literal on right side"),
            }
        }
        _ => panic!("Expected assignment node"),
    }
}

#[test]
fn test_parse_assignment_with_identifier() {
    let code = "fn main() { set x = y; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { lparam, rparam } => {
            match &**lparam {
                Node::Identifier { name } => assert_eq!(name, "x"),
                _ => panic!("Expected identifier"),
            }
            match &**rparam {
                Node::Identifier { name } => assert_eq!(name, "y"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_assignment_with_negative_number() {
    let code = "fn main() { set x = -42; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Litteral { value } => assert_eq!(*value, -42),
                _ => panic!("Expected literal"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_assignment_with_memory_value() {
    let code = "fn main() { set x = $Velocity; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::MemoryValue { name } => assert_eq!(name, "Velocity"),
                _ => panic!("Expected memory value, got {:?}", rparam),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

// ========================================
// Arithmetic Operation Tests
// ========================================

#[test]
fn test_parse_addition() {
    let code = "fn main() { set x = a + b; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { lparam, rparam, operation } => {
                    assert!(matches!(operation, OperationType::Addition));
                    match &**lparam {
                        Node::Identifier { name } => assert_eq!(name, "a"),
                        _ => panic!("Expected identifier"),
                    }
                    match &**rparam {
                        Node::Identifier { name } => assert_eq!(name, "b"),
                        _ => panic!("Expected identifier"),
                    }
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_subtraction() {
    let code = "fn main() { set x = 10 - 5; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { operation, .. } => {
                    assert!(matches!(operation, OperationType::Substraction));
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_multiplication() {
    let code = "fn main() { set x = 3 * 4; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { operation, .. } => {
                    assert!(matches!(operation, OperationType::Multiplication));
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_division() {
    let code = "fn main() { set x = 20 / 4; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { operation, .. } => {
                    assert!(matches!(operation, OperationType::Division));
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_modulo() {
    let code = "fn main() { set x = 10 % 3; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { operation, .. } => {
                    assert!(matches!(operation, OperationType::Modulo));
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_operation_with_literals_and_identifiers() {
    let code = "fn main() { set x = count + 1; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::Operation { lparam, rparam, .. } => {
                    match &**lparam {
                        Node::Identifier { name } => assert_eq!(name, "count"),
                        _ => panic!("Expected identifier"),
                    }
                    match &**rparam {
                        Node::Litteral { value } => assert_eq!(*value, 1),
                        _ => panic!("Expected literal"),
                    }
                }
                _ => panic!("Expected operation"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

// ========================================
// Comparison Tests
// ========================================

#[test]
fn test_parse_greater_than() {
    let code = "fn main() { if a > b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::GT));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_greater_than_or_equal() {
    let code = "fn main() { if a >= b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::GE));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_less_than() {
    let code = "fn main() { if a < b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::LT));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_less_than_or_equal() {
    let code = "fn main() { if a <= b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::LE));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_equality() {
    let code = "fn main() { if a == b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::EQ));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_inequality() {
    let code = "fn main() { if a != b {} }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { condition, .. } => {
            match &**condition {
                Node::Comparison { comparison, .. } => {
                    assert!(matches!(comparison, ComparisonType::DIFF));
                }
                _ => panic!("Expected comparison"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

// ========================================
// Control Flow Tests
// ========================================

#[test]
fn test_parse_if_statement() {
    let code = "fn main() { if x > 0 { set y = 1; } }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 1);

    match &*content[0] {
        Node::IfCondition { condition, content } => {
            assert!(matches!(&**condition, Node::Comparison { .. }));
            assert_eq!(content.len(), 1);
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_while_loop() {
    let code = "fn main() { while x < 10 { set x = x + 1; } }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 1);

    match &*content[0] {
        Node::WhileLoop { condition, content } => {
            assert!(matches!(&**condition, Node::Comparison { .. }));
            assert_eq!(content.len(), 1);
        }
        _ => panic!("Expected while loop"),
    }
}

#[test]
fn test_parse_infinite_loop() {
    let code = "fn main() { loop { set x = 1; } }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 1);

    match &*content[0] {
        Node::Loop { content } => {
            assert_eq!(content.len(), 1);
        }
        _ => panic!("Expected loop"),
    }
}

#[test]
fn test_parse_nested_if_statements() {
    let code = "fn main() { if x > 0 { if y > 0 { set z = 1; } } }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::IfCondition { content, .. } => {
            assert_eq!(content.len(), 1);
            match &*content[0] {
                Node::IfCondition { content, .. } => {
                    assert_eq!(content.len(), 1);
                }
                _ => panic!("Expected nested if"),
            }
        }
        _ => panic!("Expected if condition"),
    }
}

#[test]
fn test_parse_nested_loops() {
    let code = "fn main() { while x < 10 { while y < 5 { set y = y + 1; } } }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::WhileLoop { content, .. } => {
            assert_eq!(content.len(), 1);
            match &*content[0] {
                Node::WhileLoop { .. } => {}, // Success
                _ => panic!("Expected nested while loop"),
            }
        }
        _ => panic!("Expected while loop"),
    }
}

// ========================================
// Function Call Tests
// ========================================

#[test]
fn test_parse_function_call_no_params() {
    let code = "fn main() { call helper(); }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::FunctionCall { function_name, parameters } => {
            assert_eq!(function_name, "helper");
            assert_eq!(parameters.len(), 0);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_function_call_with_params() {
    let code = "fn main() { call add(5, 3); }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::FunctionCall { function_name, parameters } => {
            assert_eq!(function_name, "add");
            assert_eq!(parameters.len(), 2);
            match &*parameters[0] {
                Node::Litteral { value } => assert_eq!(*value, 5),
                _ => panic!("Expected literal"),
            }
            match &*parameters[1] {
                Node::Litteral { value } => assert_eq!(*value, 3),
                _ => panic!("Expected literal"),
            }
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_function_call_in_assignment() {
    let code = "fn main() { set result = call add(x, y); }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::FunctionCall { function_name, parameters } => {
                    assert_eq!(function_name, "add");
                    assert_eq!(parameters.len(), 2);
                }
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

// ========================================
// Return Statement Tests
// ========================================

#[test]
fn test_parse_return_with_value() {
    let code = "fn main() { return 42; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Return { value } => {
            match &**value {
                Node::Litteral { value } => assert_eq!(*value, 42),
                _ => panic!("Expected literal"),
            }
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_parse_return_with_identifier() {
    let code = "fn main() { return x; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Return { value } => {
            match &**value {
                Node::Identifier { name } => assert_eq!(name, "x"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected return statement"),
    }
}

#[test]
fn test_parse_return_without_value() {
    let code = "fn main() { return; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Return { value } => {
            match &**value {
                Node::Litteral { value } => assert_eq!(*value, 0),
                _ => panic!("Expected default literal 0"),
            }
        }
        _ => panic!("Expected return statement"),
    }
}

// ========================================
// Print Statement Tests
// ========================================

#[test]
fn test_parse_print_literal() {
    let code = "fn main() { print 42; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Print { value } => {
            match &**value {
                Node::Litteral { value } => assert_eq!(*value, 42),
                _ => panic!("Expected literal"),
            }
        }
        _ => panic!("Expected print statement"),
    }
}

#[test]
fn test_parse_print_identifier() {
    let code = "fn main() { print x; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Print { value } => {
            match &**value {
                Node::Identifier { name } => assert_eq!(name, "x"),
                _ => panic!("Expected identifier"),
            }
        }
        _ => panic!("Expected print statement"),
    }
}

// ========================================
// Memory Access Tests
// ========================================

#[test]
fn test_parse_array_access_with_literal() {
    let code = "fn main() { set x = arr[5]; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::MemoryOffset { base, offset } => {
                    match &**base {
                        Node::Identifier { name } => assert_eq!(name, "arr"),
                        _ => panic!("Expected identifier for base"),
                    }
                    match &**offset {
                        Node::Litteral { value } => assert_eq!(*value, 5),
                        _ => panic!("Expected literal for offset"),
                    }
                }
                _ => panic!("Expected memory offset"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_parse_array_access_with_identifier() {
    let code = "fn main() { set x = arr[i]; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;

    match &*content[0] {
        Node::Assignment { rparam, .. } => {
            match &**rparam {
                Node::MemoryOffset { base, offset } => {
                    match &**base {
                        Node::Identifier { name } => assert_eq!(name, "arr"),
                        _ => panic!("Expected identifier for base"),
                    }
                    match &**offset {
                        Node::Identifier { name } => assert_eq!(name, "i"),
                        _ => panic!("Expected identifier for offset"),
                    }
                }
                _ => panic!("Expected memory offset"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

// ========================================
// Complex Program Tests
// ========================================

#[test]
fn test_parse_complete_program() {
    let code = r#"
        fn factorial(n) {
            if n <= 1 {
                return 1;
            }
            set temp = n - 1;
            set result = call factorial(temp);
            return n * result;
        }

        fn main() {
            set x = 5;
            set fact = call factorial(x);
            print fact;
        }
    "#;

    let ast = parse_program(code).unwrap();
    assert_eq!(ast.functions.len(), 2);
    assert!(ast.functions.contains_key("factorial"));
    assert!(ast.functions.contains_key("main"));

    // Verify factorial function structure
    let factorial = &ast.functions["factorial"];
    assert_eq!(factorial.parameters.len(), 1);
    assert_eq!(factorial.parameters[0], "n");
    assert!(factorial.content.len() > 0);

    // Verify main function structure
    let main = &ast.functions["main"];
    assert_eq!(main.parameters.len(), 0);
    assert_eq!(main.content.len(), 3);
}

#[test]
fn test_parse_multiple_statements() {
    let code = "fn main() { set x = 1; set y = 2; set z = 3; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 3);
}

// ========================================
// Edge Case Tests
// ========================================

#[test]
fn test_parse_empty_program() {
    let code = "";
    let ast = parse_program(code).unwrap();
    assert_eq!(ast.functions.len(), 0);
}

#[test]
fn test_parse_function_with_only_semicolons() {
    let code = "fn main() { ;;; }";
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    assert_eq!(content.len(), 0);
}

#[test]
fn test_parse_with_comments() {
    let code = r#"
        fn main() {
            // This is a comment
            set x = 5; // Another comment
            // set y = 10;
        }
    "#;
    let ast = parse_program(code).unwrap();
    let content = &ast.functions["main"].content;
    // Should only have one assignment, comments should be ignored
    assert_eq!(content.len(), 1);
}

// ========================================
// Error Handling Tests
// ========================================

#[test]
fn test_error_missing_function_lparen() {
    let code = "fn main {}";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_function_lbrace() {
    let code = "fn main() }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_unexpected_token_at_top_level() {
    let code = "set x = 5;";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_assignment_operator() {
    let code = "fn main() { set x 5; }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_arithmetic_operator() {
    let code = "fn main() { set x = a & b; }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_invalid_comparison_operator() {
    let code = "fn main() { if a <> b {} }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_loop_lbrace() {
    let code = "fn main() { loop set x = 1; } }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_unexpected_end_of_stream_in_function() {
    let code = "fn main(";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_unexpected_end_of_stream_in_assignment() {
    let code = "fn main() { set x = }";
    let result = parse_program(code);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_array_rbracket() {
    let code = "fn main() { set x = arr[5; }";
    let result = parse_program(code);
    assert!(result.is_err());
}

// ========================================
// Error Metadata Tests
// ========================================

#[test]
fn test_error_contains_line_information() {
    let code = "fn main() {\n    set x & 5;\n}";
    let result = parse_program(code);

    match result {
        Err(e) => {
            let error_string = format!("{}", e);
            // Error should mention line 2 where the invalid operator is
            assert!(error_string.contains("line"));
        }
        Ok(_) => panic!("Expected error"),
    }
}

#[test]
fn test_error_unexpected_token_has_metadata() {
    let code = "set x = 5;";
    let result = parse_program(code);

    match result {
        Err(e) => {
            let error_string = format!("{}", e);
            // Should contain line and char information
            assert!(error_string.contains("line"));
            assert!(error_string.contains("char"));
        }
        Ok(_) => panic!("Expected error"),
    }
}
