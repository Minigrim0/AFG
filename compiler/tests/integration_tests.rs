// Integration tests for the AFG compiler
// These tests verify end-to-end functionality from source code to AST

use afgcompiler::token::lex;
use afgcompiler::ast::AST;

// ========================================
// Complete Program Integration Tests
// ========================================

#[test]
fn test_fibonacci_program() {
    let code = r#"
        fn fibonacci(n) {
            if n <= 1 {
                return n;
            }
            set a = n - 1;
            set b = n - 2;
            set fib_a = call fibonacci(a);
            set fib_b = call fibonacci(b);
            return fib_a + fib_b;
        }

        fn main() {
            set result = call fibonacci(10);
            print result;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.functions.len(), 2);
    assert!(ast.functions.contains_key("fibonacci"));
    assert!(ast.functions.contains_key("main"));
}

#[test]
fn test_factorial_iterative() {
    let code = r#"
        fn factorial(n) {
            set result = 1;
            set counter = n;

            while counter > 1 {
                set result = result * counter;
                set counter = counter - 1;
            }

            return result;
        }

        fn main() {
            set fact = call factorial(5);
            print fact;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();

    // Verify the factorial function has while loop
    let factorial_fn = &ast.functions["factorial"];
    assert!(factorial_fn.content.len() > 0);
}

#[test]
fn test_array_manipulation() {
    let code = r#"
        fn sum_array(arr, size) {
            set total = 0;
            set i = 0;

            while i < size {
                set val = arr[i];
                set total = total + val;
                set i = i + 1;
            }

            return total;
        }

        fn main() {
            set data = $ArrayData;
            set result = call sum_array(data, 10);
            print result;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert!(ast.functions.contains_key("sum_array"));
}

#[test]
fn test_nested_control_flow() {
    let code = r#"
        fn complex_logic(x, y) {
            if x > 0 {
                if y > 0 {
                    while x > y {
                        set x = x - 1;
                    }
                    return x + y;
                }
                return x;
            }
            return 0;
        }

        fn main() {
            set result = call complex_logic(10, 5);
            print result;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_multiple_arithmetic_operations() {
    let code = r#"
        fn calculator(a, b, c) {
            set sum = a + b + c;
            set product = a * b * c;
            set diff = a - b - c;
            set result = sum + product - diff;
            return result;
        }

        fn main() {
            set ans = call calculator(5, 3, 2);
            print ans;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_all_comparison_operators() {
    let code = r#"
        fn comparisons(x, y) {
            if x > y {
                return 1;
            }
            if x >= y {
                return 2;
            }
            if x < y {
                return 3;
            }
            if x <= y {
                return 4;
            }
            if x == y {
                return 5;
            }
            if x != y {
                return 6;
            }
            return 0;
        }

        fn main() {
            set result = call comparisons(5, 10);
            print result;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_infinite_loop_with_conditional_break() {
    let code = r#"
        fn find_value(target) {
            set current = 0;

            loop {
                if current == target {
                    return current;
                }
                set current = current + 1;
            }
        }

        fn main() {
            set found = call find_value(42);
            print found;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

// ========================================
// Edge Case Integration Tests
// ========================================

#[test]
fn test_empty_function_bodies() {
    let code = r#"
        fn empty1() {}
        fn empty2() {}
        fn main() {}
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.functions.len(), 3);
}

#[test]
fn test_single_statement_functions() {
    let code = r#"
        fn return_five() { return 5; }
        fn return_param(x) { return x; }
        fn print_hello() { print 42; }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_deeply_nested_blocks() {
    let code = r#"
        fn nested() {
            if 1 > 0 {
                if 2 > 0 {
                    if 3 > 0 {
                        if 4 > 0 {
                            if 5 > 0 {
                                return 42;
                            }
                        }
                    }
                }
            }
            return 0;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_many_sequential_statements() {
    let code = r#"
        fn many_statements() {
            set a = 1;
            set b = 2;
            set c = 3;
            set d = 4;
            set e = 5;
            set f = 6;
            set g = 7;
            set h = 8;
            set i = 9;
            set j = 10;
            return j;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.functions["many_statements"].content.len(), 11);
}

#[test]
fn test_comments_everywhere() {
    let code = r#"
        // Function comment
        fn test() { // inline comment
            // Before statement
            set x = 5; // After statement
            // Between statements
            set y = 10;
            // Before return
            return x + y; // End comment
        } // After function
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    // Comments should be ignored, function should parse correctly
    assert_eq!(ast.functions["test"].content.len(), 3);
}

#[test]
fn test_negative_numbers_in_various_contexts() {
    let code = r#"
        fn negatives() {
            set a = -1;
            set b = -42;
            set c = a + -5;
            set d = -10 * -2;
            if x < -5 {
                return -1;
            }
            return 0;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_memory_values_and_array_access() {
    let code = r#"
        fn memory_ops() {
            set x = $GlobalVar;
            set y = arr[0];
            set z = arr[index];
            set $Output = result;
            return 0;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_complex_expressions_in_assignments() {
    let code = r#"
        fn complex() {
            set a = 1 + 2;
            set b = 3 * 4;
            set c = 5 - 6;
            set d = 7 / 8;
            set e = 9 % 2;
            return e;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_function_calls_with_various_arguments() {
    let code = r#"
        fn caller() {
            call no_args();
            call one_arg(5);
            call two_args(x, y);
            call three_args(1, 2, 3);
            call mixed_args(literal, 42, $Mem);
            set result = call with_return(a, b);
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

// ========================================
// Error Recovery Tests
// ========================================

#[test]
fn test_invalid_syntax_produces_error() {
    let code = "fn main() { syntax error here }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

#[test]
fn test_unclosed_brace_produces_error() {
    let code = "fn main() { set x = 5;";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

#[test]
fn test_invalid_token_at_toplevel() {
    let code = "random_token";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

#[test]
fn test_malformed_function_definition() {
    let code = "fn (x) { return x; }";
    let tokens = lex(code);
    let result = AST::parse(&mut tokens.into_iter().peekable());

    assert!(result.is_err());
}

// ========================================
// Lexer + Parser Integration Tests
// ========================================

#[test]
fn test_whitespace_variations() {
    let code1 = "fn main(){set x=5;return x;}";
    let code2 = "fn main() { set x = 5; return x; }";
    let code3 = "fn main()    {    set    x    =    5    ;    return    x    ;    }";

    let tokens1 = lex(code1);
    let tokens2 = lex(code2);
    let tokens3 = lex(code3);

    let ast1 = AST::parse(&mut tokens1.into_iter().peekable());
    let ast2 = AST::parse(&mut tokens2.into_iter().peekable());
    let ast3 = AST::parse(&mut tokens3.into_iter().peekable());

    assert!(ast1.is_ok());
    assert!(ast2.is_ok());
    assert!(ast3.is_ok());

    // All should produce equivalent ASTs
    assert_eq!(ast1.unwrap().functions["main"].content.len(), 2);
    assert_eq!(ast2.unwrap().functions["main"].content.len(), 2);
    assert_eq!(ast3.unwrap().functions["main"].content.len(), 2);
}

#[test]
fn test_newline_handling() {
    let code_oneline = "fn main() { set x = 5; set y = 10; return x + y; }";
    let code_multiline = r#"
        fn main() {
            set x = 5;
            set y = 10;
            return x + y;
        }
    "#;

    let tokens1 = lex(code_oneline);
    let tokens2 = lex(code_multiline);

    let ast1 = AST::parse(&mut tokens1.into_iter().peekable());
    let ast2 = AST::parse(&mut tokens2.into_iter().peekable());

    assert!(ast1.is_ok());
    assert!(ast2.is_ok());

    // Should produce same structure
    assert_eq!(
        ast1.unwrap().functions["main"].content.len(),
        ast2.unwrap().functions["main"].content.len()
    );
}

#[test]
fn test_semicolon_placement() {
    let code = r#"
        fn main() {
            set x = 5;
            set y = 10;
            ;
            ;;
            set z = 15;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    // Extra semicolons should be ignored
    assert_eq!(ast.functions["main"].content.len(), 3);
}

// ========================================
// Real-World Use Case Tests
// ========================================

#[test]
fn test_simple_game_logic() {
    let code = r#"
        fn update_position(x, y, dx, dy) {
            set new_x = x + dx;
            set new_y = y + dy;

            if new_x < 0 {
                set new_x = 0;
            }
            if new_y < 0 {
                set new_y = 0;
            }

            set $PositionX = new_x;
            set $PositionY = new_y;
            return 0;
        }

        fn main() {
            set x = $PositionX;
            set y = $PositionY;
            set dx = $VelocityX;
            set dy = $VelocityY;

            call update_position(x, y, dx, dy);
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_state_machine_logic() {
    let code = r#"
        fn process_state(current_state) {
            if current_state == 0 {
                return 1;
            }
            if current_state == 1 {
                return 2;
            }
            if current_state == 2 {
                return 0;
            }
            return 0;
        }

        fn main() {
            set state = $CurrentState;
            set next_state = call process_state(state);
            set $CurrentState = next_state;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

#[test]
fn test_counter_with_bounds() {
    let code = r#"
        fn increment_counter(current, max) {
            set next = current + 1;
            if next >= max {
                return 0;
            }
            return next;
        }

        fn main() {
            set counter = $Counter;
            set max_value = 100;
            set new_counter = call increment_counter(counter, max_value);
            set $Counter = new_counter;
            print new_counter;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}

// ========================================
// Stress Tests
// ========================================

#[test]
fn test_many_functions() {
    let mut code = String::new();
    for i in 0..50 {
        code.push_str(&format!("fn func{}() {{ return {}; }}\n", i, i));
    }

    let tokens = lex(&code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.functions.len(), 50);
}

#[test]
fn test_long_parameter_lists() {
    let code = r#"
        fn many_params(a, b, c, d, e, f, g, h, i, j) {
            set sum = a + b + c + d + e + f + g + h + i + j;
            return sum;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
    let ast = ast.unwrap();
    assert_eq!(ast.functions["many_params"].parameters.len(), 10);
}

#[test]
fn test_deeply_nested_loops() {
    let code = r#"
        fn nested_loops() {
            while a > 0 {
                while b > 0 {
                    while c > 0 {
                        set c = c - 1;
                    }
                    set b = b - 1;
                }
                set a = a - 1;
            }
            return 0;
        }
    "#;

    let tokens = lex(code);
    let ast = AST::parse(&mut tokens.into_iter().peekable());

    assert!(ast.is_ok());
}
