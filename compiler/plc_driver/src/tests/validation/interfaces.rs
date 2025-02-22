use source_code::SourceCode;

use crate::tests::parse_and_validate_buffered;

#[test]
fn pou_implementing_non_existing_interfaces() {
    let source = SourceCode::from(
        r"
        FUNCTION_BLOCK foo IMPLEMENTS delulu /* ... */ END_FUNCTION_BLOCK
        FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E048]: Interface `delulu` does not exist
      ┌─ <internal>:2:39
      │
    2 │         FUNCTION_BLOCK foo IMPLEMENTS delulu /* ... */ END_FUNCTION_BLOCK
      │                                       ^^^^^^ Interface `delulu` does not exist

    error[E048]: Interface `delulu` does not exist
      ┌─ <internal>:3:39
      │
    3 │         FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
      │                                       ^^^^^^ Interface `delulu` does not exist

    error[E048]: Interface `delululu` does not exist
      ┌─ <internal>:3:47
      │
    3 │         FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
      │                                               ^^^^^^^^ Interface `delululu` does not exist
    "###);
}

#[test]
fn pou_implementing_same_interface_multiple_times() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA /* ... */ END_INTERFACE
        FUNCTION_BLOCK foo IMPLEMENTS interfaceA, interfaceA /* ... */ END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn not_supported_pou_type_implements_interface() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA /* ... */ END_INTERFACE
        INTERFACE interfaceB /* ... */ END_INTERFACE

        // Valid
        CLASS           foo IMPLEMENTS interfaceA             /* ... */ END_CLASS
        FUNCTION_BLOCK  bar IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION_BLOCK
        
        // Invalid
        PROGRAM     baz IMPLEMENTS interfaceA            /* ... */ END_PROGRAM
        FUNCTION    qux IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E110]: Interfaces can only be implemented by `CLASS` or `FUNCTION_BLOCK`
       ┌─ <internal>:10:36
       │
    10 │         PROGRAM     baz IMPLEMENTS interfaceA            /* ... */ END_PROGRAM
       │                                    ^^^^^^^^^^ Interfaces can only be implemented by `CLASS` or `FUNCTION_BLOCK`

    error[E110]: Interfaces can only be implemented by `CLASS` or `FUNCTION_BLOCK`
       ┌─ <internal>:11:36
       │
    11 │         FUNCTION    qux IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION
       │                                    ^^^^^^^^^^^^^^^^^^^^^^ Interfaces can only be implemented by `CLASS` or `FUNCTION_BLOCK`
    "###);
}

#[test]
fn pou_implements_method_with_wrong_return_type() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA
            METHOD methodA : DINT /* ... */ END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS interfaceA
            METHOD methodA : BOOL /* ... */ END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` implemented type `BOOL`
      ┌─ <internal>:7:20
      │
    3 │             METHOD methodA : DINT /* ... */ END_METHOD
      │                    ------- see also
      ·
    7 │             METHOD methodA : BOOL /* ... */ END_METHOD
      │                    ^^^^^^^ Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` implemented type `BOOL`
    ");
}

#[test]
fn pou_does_not_implement_interface_methods() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA
            METHOD methodA /* ... */ END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        // Missing `methodA` implementation
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Method `methodA` defined in interface `interfaceA` is missing in POU `fb`
      ┌─ <internal>:6:24
      │
    3 │             METHOD methodA /* ... */ END_METHOD
      │                    ------- see also
      ·
    6 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA
      │                        ^^ Method `methodA` defined in interface `interfaceA` is missing in POU `fb`
    "###);
}

#[test]
fn pou_with_missing_parameter_in_interface_implementation() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
        METHOD methodA
        VAR_INPUT
            a : DINT;
            b : DINT;
            c : DINT;
        END_VAR
        END_METHOD
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        METHOD methodA
        VAR_INPUT
            a : DINT;
            b : DINT;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Parameter `c : DINT` missing in method `methodA`
       ┌─ <internal>:13:16
       │
     7 │             c : DINT;
       │             - see also
       ·
    13 │         METHOD methodA
       │                ^^^^^^^ Parameter `c : DINT` missing in method `methodA`
    "###);
}

#[test]
fn pou_with_unordered_parameters_in_interface_implementation() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
        METHOD methodA
        VAR_INPUT
            b : DINT;
            a : DINT;
            c : DINT;
        END_VAR
        END_METHOD
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        METHOD methodA
        VAR_INPUT
            a : DINT;
            b : DINT;
            c : DINT;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Interface implementation mismatch: expected parameter `b` but got `a`
       ┌─ <internal>:5:13
       │
     5 │             b : DINT;
       │             ^ Interface implementation mismatch: expected parameter `b` but got `a`
       ·
    15 │             a : DINT;
       │             - see also

    error[E112]: Interface implementation mismatch: expected parameter `a` but got `b`
       ┌─ <internal>:6:13
       │
     6 │             a : DINT;
       │             ^ Interface implementation mismatch: expected parameter `a` but got `b`
       ·
    16 │             b : DINT;
       │             - see also
    "###);
}

#[test]
fn pou_with_incorrect_parameter_type_in_interface_implementation() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
        METHOD methodA
        VAR_INPUT
            a : DINT;
        END_VAR
        END_METHOD
    END_INTERFACE
        
    FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        METHOD methodA
        VAR_INPUT
            a : BOOL;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: Parameter `a` has different types in declaration and implemenation:
       ┌─ <internal>:11:16
       │
     5 │             a : DINT;
       │             - see also
       ·
    11 │         METHOD methodA
       │                ^^^^^^^ Interface implementation mismatch: Parameter `a` has different types in declaration and implemenation:

    error[E112]: Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` implemented type `BOOL`
       ┌─ <internal>:11:16
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    11 │         METHOD methodA
       │                ^^^^^^^ Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` implemented type `BOOL`
    ");
}

#[test]
fn pou_with_incorrect_parameter_declaration_type_in_interface_implementation() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
        METHOD methodA
        VAR_INPUT {ref}
            a : DINT;
        END_VAR
        END_METHOD
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        METHOD methodA
        VAR_IN_OUT
            a : DINT;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Interface implementation mismatch: Expected parameter `a` to have `Input` as its declaration type but got `InOut`
       ┌─ <internal>:11:16
       │
     5 │             a : DINT;
       │             - see also
       ·
    11 │         METHOD methodA
       │                ^^^^^^^ Interface implementation mismatch: Expected parameter `a` to have `Input` as its declaration type but got `InOut`
    "###);
}

#[test]
fn pou_with_more_parameters_than_defined_in_interface() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
        METHOD methodA
        VAR_INPUT
            a : DINT;
            b : DINT;
            c : DINT;
        END_VAR
        END_METHOD
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        METHOD methodA
        VAR_INPUT
            // Invalid parameters
            d : DINT;
            e : DINT;
            f : DINT;

            a : DINT;
            b : DINT;
            c : DINT;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Interface implementation mismatch: expected parameter `a` but got `d`
       ┌─ <internal>:5:13
       │
     5 │             a : DINT;
       │             ^ Interface implementation mismatch: expected parameter `a` but got `d`
       ·
    16 │             d : DINT;
       │             - see also

    error[E112]: Interface implementation mismatch: expected parameter `b` but got `e`
       ┌─ <internal>:6:13
       │
     6 │             b : DINT;
       │             ^ Interface implementation mismatch: expected parameter `b` but got `e`
       ·
    17 │             e : DINT;
       │             - see also

    error[E112]: Interface implementation mismatch: expected parameter `c` but got `f`
       ┌─ <internal>:7:13
       │
     7 │             c : DINT;
       │             ^ Interface implementation mismatch: expected parameter `c` but got `f`
       ·
    18 │             f : DINT;
       │             - see also

    error[E112]: Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:20:13
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    20 │             a : DINT;
       │             ^ Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`

    error[E112]: Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:21:13
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    21 │             b : DINT;
       │             ^ Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`

    error[E112]: Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:22:13
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    22 │             c : DINT;
       │             ^ Parameter count mismatch: `methodA` has more parameters than the method defined in `interfaceA`
    "###);
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_return_type() {
    let source = SourceCode::from(
        r"
    INTERFACE interfaceA
    METHOD foo : INT
    VAR_INPUT
        a : INT;
        b : INT;
    END_VAR
    END_METHOD
    END_INTERFACE

    INTERFACE interfaceB
        METHOD foo : DINT
        VAR_INPUT
            a : INT;
            b : INT;
        END_VAR
        END_METHOD
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
        METHOD foo : INT
        VAR_INPUT
            a : INT;
            b : INT;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:20:20
       │
     3 │     METHOD foo : INT
       │            --- see also
       ·
    12 │         METHOD foo : DINT
       │                --- see also
       ·
    20 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                    ^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`

    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` implemented type `DINT`
       ┌─ <internal>:12:16
       │
     3 │     METHOD foo : INT
       │            --- see also
       ·
    12 │         METHOD foo : DINT
       │                ^^^ Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` implemented type `DINT`
    ");
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_parameter_list_type() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA
            METHOD foo : INT
            VAR_INPUT
                a : INT;
                b : INT;
            END_VAR
        END_METHOD
        END_INTERFACE
        
        INTERFACE interfaceB
            METHOD foo : INT
            VAR_INPUT
                a : INT;
                b : DINT;
            END_VAR
            END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
            METHOD foo : INT
            VAR_INPUT
                a : DINT;
                b : DINT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:20:24
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
       ·
    20 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`

    error[E112]: Interface implementation mismatch: Parameter `b` has different types in declaration and implemenation:
       ┌─ <internal>:12:20
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    ^^^ Interface implementation mismatch: Parameter `b` has different types in declaration and implemenation:

    error[E112]: Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` implemented type `DINT`
       ┌─ <internal>:12:20
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    ^^^ Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` implemented type `DINT`
    ");
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_parameter_list_declaration_type() {
    let source = SourceCode::from(
        r"
        INTERFACE interfaceA
            METHOD foo : INT
            VAR_INPUT
                a : INT;
                b : INT;
            END_VAR
            END_METHOD
        END_INTERFACE

        INTERFACE interfaceB
            METHOD foo : INT
            VAR_OUTPUT
                a : INT;
            END_VAR
            END_METHOD
        END_INTERFACE
        
        FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
            METHOD foo : INT
            VAR_INPUT
                a : INT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:19:24
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
       ·
    19 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`

    error[E112]: Interface implementation mismatch: Expected parameter `a` to have `Input` as its declaration type but got `Output`
       ┌─ <internal>:12:20
       │
     5 │                 a : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    ^^^ Interface implementation mismatch: Expected parameter `a` to have `Input` as its declaration type but got `Output`

    error[E112]: Parameter `b : INT` missing in method `foo`
       ┌─ <internal>:12:20
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    ^^^ Parameter `b : INT` missing in method `foo`
    "###);
}

#[test]
fn interface_with_aggregate_return_type_string() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn interface_with_aggregate_return_type_string_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : WSTRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `WSTRING`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : STRING
      │                    --- see also
      ·
    8 │             METHOD bar : WSTRING
      │                    ^^^ Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `WSTRING`
    ");
}

#[test]
fn interface_with_aliased_aggregate_return_type_string() {
    let source = SourceCode::from(
        r"
        TYPE myString : STRING[10]; END_TYPE
        INTERFACE foo
            METHOD bar : myString
            END_METHOD
        END_INTERFACE
        
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Expected string of length `11` but got string of length `81`
      ┌─ <internal>:9:20
      │
    4 │             METHOD bar : myString
      │                    --- see also
      ·
    9 │             METHOD bar : STRING
      │                    ^^^ Expected string of length `11` but got string of length `81`
    ");
}

#[test]
fn interface_with_aggregate_return_type_array() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn interface_with_aggregate_return_type_array_length_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : ARRAY[1..6] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Expected array size `6` but got `5`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : ARRAY[1..6] OF STRING
      │                    --- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF STRING
      │                    ^^^ Expected array size `6` but got `5`
    ");
}

#[test]
fn interface_with_aggregate_return_type_array_inner_type_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF WSTRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Expected array of type `STRING` but got `WSTRING`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : ARRAY[1..5] OF STRING
      │                    --- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF WSTRING
      │                    ^^^ Expected array of type `STRING` but got `WSTRING`
    ");
}

#[test]
fn interface_with_aggregate_return_type_nested_arrays() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
            METHOD baz : ARRAY[1..5] OF ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
            METHOD baz : ARRAY[1..5] OF ARRAY[1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK        
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aggregate_return_type_nested_arrays_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK        
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Expected array of type `foo.bar_` but got `STRING`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : ARRAY[1..5] OF ARRAY[1..5] OF STRING
      │                    --- see also
      ·
    8 │             METHOD bar : ARRAY[1..5, 1..5] OF STRING
      │                    ^^^ Expected array of type `foo.bar_` but got `STRING`
    ");
}

#[test]
fn interface_with_aggregate_return_type_struct() {
    let source = SourceCode::from(
        r"
            TYPE txn : STRUCT
                id      : DINT;
                block   : DINT;
                values  : STRING;
            END_STRUCT END_TYPE
            
            INTERFACE foo
                METHOD bar : txn
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK fb IMPLEMENTS foo
                METHOD bar : txn
                END_METHOD
            END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn interface_with_aliased_aggregate_return_type_struct() {
    let source = SourceCode::from(
        r"
            TYPE txn : STRUCT
                id      : DINT;
                block   : DINT;
                values  : STRING;
            END_STRUCT END_TYPE

            TYPE myTxn : txn; END_TYPE
            
            INTERFACE foo
                METHOD bar : txn
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK fb IMPLEMENTS foo
                METHOD bar : myTxn
                END_METHOD
            END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn interface_with_aggregate_return_type_non_aggregate_impl() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `DINT`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : STRING
      │                    --- see also
      ·
    8 │             METHOD bar : DINT
      │                    ^^^ Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `DINT`
    ");
}

#[test]
fn interface_with_non_aggregate_return_type_aggregate_impl() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `DINT` declared in `foo.bar` but `fb.bar` implemented type `STRING`
      ┌─ <internal>:8:20
      │
    3 │             METHOD bar : DINT
      │                    --- see also
      ·
    8 │             METHOD bar : STRING
      │                    ^^^ Type `DINT` declared in `foo.bar` but `fb.bar` implemented type `STRING`
    ");
}

#[test]
fn interface_with_aggregate_return_type_non_aggregate_impl_parameter_count_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : STRING
            VAR_INPUT
                a : DINT;
                b : DINT;
            END_VAR
            END_METHOD

            METHOD baz : STRING 
            VAR_INPUT
                a : DINT;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : DINT
            VAR_INPUT
                a : DINT;
            END_VAR
            END_METHOD
            
            METHOD baz : DINT 
            VAR_INPUT
                a : DINT;
                b : DINT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `DINT`
       ┌─ <internal>:18:20
       │
     3 │             METHOD bar : STRING
       │                    --- see also
       ·
    18 │             METHOD bar : DINT
       │                    ^^^ Type `STRING` declared in `foo.bar` but `fb.bar` implemented type `DINT`

    error[E112]: Parameter `b : DINT` missing in method `bar`
       ┌─ <internal>:18:20
       │
     6 │                 b : DINT;
       │                 - see also
       ·
    18 │             METHOD bar : DINT
       │                    ^^^ Parameter `b : DINT` missing in method `bar`

    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `STRING` declared in `foo.baz` but `fb.baz` implemented type `DINT`
       ┌─ <internal>:24:20
       │
    10 │             METHOD baz : STRING 
       │                    --- see also
       ·
    24 │             METHOD baz : DINT 
       │                    ^^^ Type `STRING` declared in `foo.baz` but `fb.baz` implemented type `DINT`

    error[E112]: Parameter count mismatch: `baz` has more parameters than the method defined in `foo`
       ┌─ <internal>:27:17
       │
    10 │             METHOD baz : STRING 
       │                    --- see also
       ·
    27 │                 b : DINT;
       │                 ^ Parameter count mismatch: `baz` has more parameters than the method defined in `foo`
    ");
}

#[test]
fn interface_with_non_aggregate_return_type_aggregate_impl_parameter_count_mismatch() {
    let source = SourceCode::from(
        r"
        INTERFACE foo
            METHOD bar : DINT
            VAR_INPUT
                a : DINT;
                b : DINT;
            END_VAR
            END_METHOD

            METHOD baz : DINT 
            VAR_INPUT
                a : DINT;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            VAR_INPUT
                a : DINT;
            END_VAR
            END_METHOD
            
            METHOD baz : STRING 
            VAR_INPUT
                a : DINT;
                b : DINT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `DINT` declared in `foo.bar` but `fb.bar` implemented type `STRING`
       ┌─ <internal>:18:20
       │
     3 │             METHOD bar : DINT
       │                    --- see also
       ·
    18 │             METHOD bar : STRING
       │                    ^^^ Type `DINT` declared in `foo.bar` but `fb.bar` implemented type `STRING`

    error[E112]: Parameter `b : DINT` missing in method `bar`
       ┌─ <internal>:18:20
       │
     6 │                 b : DINT;
       │                 - see also
       ·
    18 │             METHOD bar : STRING
       │                    ^^^ Parameter `b : DINT` missing in method `bar`

    error[E112]: Interface implementation mismatch: return types do not match:

    error[E112]: Type `DINT` declared in `foo.baz` but `fb.baz` implemented type `STRING`
       ┌─ <internal>:24:20
       │
    10 │             METHOD baz : DINT 
       │                    --- see also
       ·
    24 │             METHOD baz : STRING 
       │                    ^^^ Type `DINT` declared in `foo.baz` but `fb.baz` implemented type `STRING`

    error[E112]: Parameter count mismatch: `baz` has more parameters than the method defined in `foo`
       ┌─ <internal>:27:17
       │
    10 │             METHOD baz : DINT 
       │                    --- see also
       ·
    27 │                 b : DINT;
       │                 ^ Parameter count mismatch: `baz` has more parameters than the method defined in `foo`
    ");
}
