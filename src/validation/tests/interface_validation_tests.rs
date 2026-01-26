use test_utils::parse_and_validate_buffered;

#[test]
fn pou_implementing_non_existing_interfaces() {
    let source = r"
        FUNCTION_BLOCK foo IMPLEMENTS delulu /* ... */ END_FUNCTION_BLOCK
        FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
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
    ");
}

#[test]
fn pou_implementing_same_interface_multiple_times() {
    let source = r"
        INTERFACE interfaceA /* ... */ END_INTERFACE
        FUNCTION_BLOCK foo IMPLEMENTS interfaceA, interfaceA /* ... */ END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn not_supported_pou_type_implements_interface() {
    let source = r"
        INTERFACE interfaceA /* ... */ END_INTERFACE
        INTERFACE interfaceB /* ... */ END_INTERFACE

        // Valid
        CLASS           foo IMPLEMENTS interfaceA             /* ... */ END_CLASS
        FUNCTION_BLOCK  bar IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION_BLOCK
        
        // Invalid
        PROGRAM     baz IMPLEMENTS interfaceA            /* ... */ END_PROGRAM
        FUNCTION    qux IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
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
    ");
}

#[test]
fn pou_implements_method_with_wrong_return_type() {
    let source = r"
        INTERFACE interfaceA
            METHOD methodA : DINT /* ... */ END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS interfaceA
            METHOD methodA : BOOL /* ... */ END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:6:24
      │
    6 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` declared type `BOOL`
      ┌─ <internal>:3:20
      │
    3 │             METHOD methodA : DINT /* ... */ END_METHOD
      │                    ------- see also
      ·
    7 │             METHOD methodA : BOOL /* ... */ END_METHOD
      │                    ------- see also
    ");
}

#[test]
fn pou_does_not_implement_interface_methods() {
    let source = r"
        INTERFACE interfaceA
            METHOD methodA /* ... */ END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK fb IMPLEMENTS interfaceA
        // Missing `methodA` implementation
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Method `methodA` defined in interface `interfaceA` is missing in POU `fb`
      ┌─ <internal>:6:24
      │
    3 │             METHOD methodA /* ... */ END_METHOD
      │                    ------- see also
      ·
    6 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA
      │                        ^^ Method `methodA` defined in interface `interfaceA` is missing in POU `fb`
    ");
}

#[test]
fn pou_with_missing_parameter_in_interface_implementation() {
    let source = r"
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
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:12:20
       │
    12 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
       │                    ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `c : DINT` missing in method `methodA`
       ┌─ <internal>:7:13
       │
     7 │             c : DINT;
       │             - see also
       ·
    13 │         METHOD methodA
       │                ------- see also
    ");
}

#[test]
fn pou_with_unordered_parameters_in_interface_implementation() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:12:20
       │
    12 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
       │                    ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Expected parameter `b` but got `a`
       ┌─ <internal>:5:13
       │
     5 │             b : DINT;
       │             - see also
       ·
    15 │             a : DINT;
       │             - see also

    note[E118]: Expected parameter `a` but got `b`
       ┌─ <internal>:6:13
       │
     6 │             a : DINT;
       │             - see also
       ·
    16 │             b : DINT;
       │             - see also
    ");
}

#[test]
fn pou_with_incorrect_parameter_type_in_interface_implementation() {
    let source = r"
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
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:10:20
       │
    10 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
       │                    ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `a` has conflicting type declarations:
       ┌─ <internal>:5:13
       │
     5 │             a : DINT;
       │             - see also
       ·
    11 │         METHOD methodA
       │                ------- see also

    note[E118]: Type `DINT` declared in `interfaceA.methodA` but `fb.methodA` declared type `BOOL`
       ┌─ <internal>:3:16
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    11 │         METHOD methodA
       │                ------- see also
    ");
}

#[test]
fn pou_with_incorrect_parameter_declaration_type_in_interface_implementation() {
    let source = r"
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
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:10:20
       │
    10 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
       │                    ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Expected parameter `a` to have `Input` as its declaration type but got `InOut`
       ┌─ <internal>:5:13
       │
     5 │             a : DINT;
       │             - see also
       ·
    11 │         METHOD methodA
       │                ------- see also
    ");
}

#[test]
fn pou_with_more_parameters_than_defined_in_interface() {
    let source = r"
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
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:12:20
       │
    12 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
       │                    ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Expected parameter `a` but got `d`
       ┌─ <internal>:5:13
       │
     5 │             a : DINT;
       │             - see also
       ·
    16 │             d : DINT;
       │             - see also

    note[E118]: Expected parameter `b` but got `e`
       ┌─ <internal>:6:13
       │
     6 │             b : DINT;
       │             - see also
       ·
    17 │             e : DINT;
       │             - see also

    note[E118]: Expected parameter `c` but got `f`
       ┌─ <internal>:7:13
       │
     7 │             c : DINT;
       │             - see also
       ·
    18 │             f : DINT;
       │             - see also

    note[E118]: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:3:16
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    20 │             a : DINT;
       │             - see also

    note[E118]: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:3:16
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    21 │             b : DINT;
       │             - see also

    note[E118]: `methodA` has more parameters than the method defined in `interfaceA`
       ┌─ <internal>:3:16
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    22 │             c : DINT;
       │             - see also
    ");
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_return_type() {
    let source = r"
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
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`
       ┌─ <internal>:20:20
       │
     3 │     METHOD foo : INT
       │            --- see also
       ·
    12 │         METHOD foo : DINT
       │                --- see also
       ·
    20 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                    ^^ Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:20:20
       │
    20 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                    ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` declared type `DINT`
       ┌─ <internal>:3:12
       │
     3 │     METHOD foo : INT
       │            --- see also
       ·
    12 │         METHOD foo : DINT
       │                --- see also
    ");
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_parameter_list_type() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`
       ┌─ <internal>:20:24
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
       ·
    20 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:20:24
       │
    20 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `b` has conflicting type declarations:
       ┌─ <internal>:6:17
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also

    note[E118]: Type `INT` declared in `interfaceA.foo` but `interfaceB.foo` declared type `DINT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
    ");
}

#[test]
fn interfaces_with_same_method_name_but_different_signatures_parameter_list_declaration_type() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`
       ┌─ <internal>:19:24
       │
     3 │             METHOD foo : INT
       │                    --- see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
       ·
    19 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Method `foo` in `fb` is declared with conflicting signatures in `interfaceA` and `interfaceB`

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:19:24
       │
    19 │         FUNCTION_BLOCK fb IMPLEMENTS interfaceA, interfaceB
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Expected parameter `a` to have `Input` as its declaration type but got `Output`
       ┌─ <internal>:5:17
       │
     5 │                 a : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also

    note[E118]: Parameter `b : INT` missing in method `foo`
       ┌─ <internal>:6:17
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │             METHOD foo : INT
       │                    --- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_string() {
    let source = r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aggregate_return_type_string_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : WSTRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `STRING` declared in `foo.bar` but `fb.bar` declared type `WSTRING`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : STRING
      │                    --- see also
      ·
    8 │             METHOD bar : WSTRING
      │                    --- see also
    ");
}

#[test]
fn interface_with_aliased_aggregate_return_type_string() {
    let source = r"
        TYPE myString : STRING[10]; END_TYPE
        INTERFACE foo
            METHOD bar : myString
            END_METHOD
        END_INTERFACE
        
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:8:24
      │
    8 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Expected string of length `11` but got string of length `81`
      ┌─ <internal>:4:20
      │
    4 │             METHOD bar : myString
      │                    --- see also
      ·
    9 │             METHOD bar : STRING
      │                    --- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_array() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aggregate_return_type_array_length_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..6] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Array range declared as `[1..6]` but implemented as `[1..5]`
      ┌─ <internal>:3:26
      │
    3 │             METHOD bar : ARRAY[1..6] OF STRING
      │                          --------------------- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF STRING
      │                          --------------------- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_array_dimension_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
            METHOD baz : ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
            METHOD baz : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:9:24
      │
    9 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Array declared with `1` dimension but implemented with `2`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar : ARRAY[1..5, 1..5] OF STRING
       │                    ---   --------------------------- see also
       │                    │      
       │                    see also
       ·
    10 │             METHOD bar : ARRAY[1..5] OF STRING
       │                    ---   --------------------- see also
       │                    │      
       │                    see also

    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:9:24
      │
    9 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Array declared with `2` dimensions but implemented with `1`
       ┌─ <internal>:5:20
       │
     5 │             METHOD baz : ARRAY[1..5] OF STRING
       │                    ---   --------------------- see also
       │                    │      
       │                    see also
       ·
    12 │             METHOD baz : ARRAY[1..5, 1..5] OF STRING
       │                    ---   --------------------------- see also
       │                    │      
       │                    see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_array_inner_type_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF WSTRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Expected array of type `STRING` but got `WSTRING`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : ARRAY[1..5] OF STRING
      │                    --- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF WSTRING
      │                    --- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_nested_arrays() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aggregate_return_type_nested_arrays_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5, 1..5] OF STRING
            END_METHOD
        END_FUNCTION_BLOCK        
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Array declared with `2` dimensions but implemented with `1`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : ARRAY[1..5] OF ARRAY[1..5] OF STRING
      │                    ---   ------------------------------------ see also
      │                    │      
      │                    see also
      ·
    8 │             METHOD bar : ARRAY[1..5, 1..5] OF STRING
      │                    ---   --------------------------- see also
      │                    │      
      │                    see also

    note[E118]: Expected array of type `foo.bar_` but got `STRING`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : ARRAY[1..5] OF ARRAY[1..5] OF STRING
      │                    --- see also
      ·
    8 │             METHOD bar : ARRAY[1..5, 1..5] OF STRING
      │                    --- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_nested_arrays_dimension_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : ARRAY[1..5] OF ARRAY[2..20] OF ARRAY[3..10] OF ARRAY[1..5] OF STRING
            END_METHOD
        END_INTERFACE
            
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : ARRAY[1..5] OF ARRAY[1..100] OF ARRAY[-2..5] OF ARRAY[1..5] OF STRING
            //                                ^^^^^^           ^^^^^ <--- Mismatch
            END_METHOD
        END_FUNCTION_BLOCK        
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Array range declared as `[1..5]` but implemented as `[1..100]`
      ┌─ <internal>:3:26
      │
    3 │             METHOD bar : ARRAY[1..5] OF ARRAY[2..20] OF ARRAY[3..10] OF ARRAY[1..5] OF STRING
      │                          -------------------------------------------------------------------- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF ARRAY[1..100] OF ARRAY[-2..5] OF ARRAY[1..5] OF STRING
      │                          --------------------------------------------------------------------- see also

    note[E118]: Array range declared as `[1..5]` but implemented as `[-2..5]`
      ┌─ <internal>:3:26
      │
    3 │             METHOD bar : ARRAY[1..5] OF ARRAY[2..20] OF ARRAY[3..10] OF ARRAY[1..5] OF STRING
      │                          -------------------------------------------------------------------- see also
      ·
    8 │             METHOD bar : ARRAY[1..5] OF ARRAY[1..100] OF ARRAY[-2..5] OF ARRAY[1..5] OF STRING
      │                                         ------------------------------------------------------ see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_struct() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aliased_aggregate_return_type_struct() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn interface_with_aggregate_return_type_non_aggregate_impl() {
    let source = r"
        INTERFACE foo
            METHOD bar : STRING
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `STRING` declared in `foo.bar` but `fb.bar` declared type `DINT`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : STRING
      │                    --- see also
      ·
    8 │             METHOD bar : DINT
      │                    --- see also
    ");
}

#[test]
fn interface_with_non_aggregate_return_type_aggregate_impl() {
    let source = r"
        INTERFACE foo
            METHOD bar : DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : STRING
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `STRING`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : DINT
      │                    --- see also
      ·
    8 │             METHOD bar : STRING
      │                    --- see also
    ");
}

#[test]
fn interface_with_aggregate_return_type_non_aggregate_impl_parameter_count_mismatch() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `STRING` declared in `foo.bar` but `fb.bar` declared type `DINT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar : STRING
       │                    --- see also
       ·
    18 │             METHOD bar : DINT
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `b : DINT` missing in method `bar`
       ┌─ <internal>:6:17
       │
     6 │                 b : DINT;
       │                 - see also
       ·
    18 │             METHOD bar : DINT
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `STRING` declared in `foo.baz` but `fb.baz` declared type `DINT`
       ┌─ <internal>:10:20
       │
    10 │             METHOD baz : STRING 
       │                    --- see also
       ·
    24 │             METHOD baz : DINT 
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: `baz` has more parameters than the method defined in `foo`
       ┌─ <internal>:10:20
       │
    10 │             METHOD baz : STRING 
       │                    --- see also
       ·
    27 │                 b : DINT;
       │                 - see also
    ");
}

#[test]
fn interface_with_non_aggregate_return_type_aggregate_impl_parameter_count_mismatch() {
    let source = r"
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
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `STRING`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar : DINT
       │                    --- see also
       ·
    18 │             METHOD bar : STRING
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `b : DINT` missing in method `bar`
       ┌─ <internal>:6:17
       │
     6 │                 b : DINT;
       │                 - see also
       ·
    18 │             METHOD bar : STRING
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.baz` but `fb.baz` declared type `STRING`
       ┌─ <internal>:10:20
       │
    10 │             METHOD baz : DINT 
       │                    --- see also
       ·
    24 │             METHOD baz : STRING 
       │                    --- see also

    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:17:24
       │
    17 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: `baz` has more parameters than the method defined in `foo`
       ┌─ <internal>:10:20
       │
    10 │             METHOD baz : DINT 
       │                    --- see also
       ·
    27 │                 b : DINT;
       │                 - see also
    ");
}

#[test]
fn pointer_return() {
    let source = r"
        INTERFACE foo
            METHOD bar : REF_TO DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : REF_TO DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn pointer_return_type_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : REF_TO INT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : REF_TO DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `INT` declared in `foo.bar` but `fb.bar` declared type `DINT`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : REF_TO INT
      │                    --- see also
      ·
    8 │             METHOD bar : REF_TO DINT
      │                    --- see also
    ");
}

#[test]
fn pointer_to_pointer_return() {
    let source = r"
        INTERFACE foo
            METHOD bar : REF_TO REF_TO DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : REF_TO REF_TO DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn pointer_to_pointer_return_inner_type_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : REF_TO REF_TO DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : REF_TO REF_TO INT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `INT`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : REF_TO REF_TO DINT
      │                    --- see also
      ·
    8 │             METHOD bar : REF_TO REF_TO INT
      │                    --- see also
    ");
}

#[test]
fn pointer_to_pointer_return_indirection_level_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar : REF_TO DINT
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar : REF_TO REF_TO DINT
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:24
      │
    7 │         FUNCTION_BLOCK fb IMPLEMENTS foo
      │                        ^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `fb.bar_`
      ┌─ <internal>:3:20
      │
    3 │             METHOD bar : REF_TO DINT
      │                    --- see also
      ·
    8 │             METHOD bar : REF_TO REF_TO DINT
      │                    --- see also
    ");
}

#[test]
fn pointer_fields() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : REF_TO DINT;
                b : REF_TO REF_TO DINT;
                c : REFERENCE TO DINT;
                d AT a : DINT;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : REF_TO DINT;
                b : REF_TO REF_TO DINT;
                c : REFERENCE TO DINT;
                d AT a : DINT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn pointer_fields_type_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : REF_TO DINT;
                b : REF_TO REF_TO DINT;
                c : REFERENCE TO DINT;
                d AT a : DINT;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : REF_TO INT;
                b : REF_TO REF_TO INT;
                c : REFERENCE TO INT;
                d AT a : INT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:13:24
       │
    13 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `a` has conflicting type declarations:
       ┌─ <internal>:5:17
       │
     5 │                 a : REF_TO DINT;
       │                 - see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `INT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Parameter `b` has conflicting type declarations:
       ┌─ <internal>:6:17
       │
     6 │                 b : REF_TO REF_TO DINT;
       │                 - see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `INT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Parameter `c` has conflicting type declarations:
       ┌─ <internal>:7:17
       │
     7 │                 c : REFERENCE TO DINT;
       │                 - see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `INT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Parameter `d` has conflicting type declarations:
       ┌─ <internal>:8:17
       │
     8 │                 d AT a : DINT;
       │                 - see also
       ·
    14 │             METHOD bar
       │                    --- see also

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `INT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    14 │             METHOD bar
       │                    --- see also
    ");
}

#[test]
fn pointer_fields_indirection_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : REF_TO DINT;
                b : REF_TO REF_TO DINT;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : REF_TO REF_TO DINT;
                b : REF_TO DINT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:11:24
       │
    11 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `a` has conflicting type declarations:
       ┌─ <internal>:5:17
       │
     5 │                 a : REF_TO DINT;
       │                 - see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Type `DINT` declared in `foo.bar` but `fb.bar` declared type `__fb.bar_a_`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Parameter `b` has conflicting type declarations:
       ┌─ <internal>:6:17
       │
     6 │                 b : REF_TO REF_TO DINT;
       │                 - see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Type `__foo.bar_b_` declared in `foo.bar` but `fb.bar` declared type `DINT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    12 │             METHOD bar
       │                    --- see also
    ");
}

#[test]
fn subranges() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : UINT(1..10);
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : UINT(1..10);
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn subranges_type_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : UINT(1..10);
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : INT(1..10);
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:10:24
       │
    10 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `a` has conflicting type declarations:
       ┌─ <internal>:5:17
       │
     5 │                 a : UINT(1..10);
       │                 - see also
       ·
    11 │             METHOD bar
       │                    --- see also

    note[E118]: Type `UINT` declared in `foo.bar` but `fb.bar` declared type `INT`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    11 │             METHOD bar
       │                    --- see also
    ");
}

#[test]
fn pointer_to_array_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD bar
            VAR_INPUT
                a : REF_TO ARRAY[1..5] OF STRING;
                b : REF_TO ARRAY[1..5] OF ARRAY[1..5] OF STRING;
            END_VAR
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS foo
            METHOD bar
            VAR_INPUT
                a : REF_TO REF_TO ARRAY[1..6] OF WSTRING;
                b : REF_TO REF_TO REF_TO ARRAY[-3..2] OF ARRAY[1..5] OF ARRAY[1..5] OF WSTRING;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Derived methods with conflicting signatures, parameters do not match:
       ┌─ <internal>:11:24
       │
    11 │         FUNCTION_BLOCK fb IMPLEMENTS foo
       │                        ^^ Derived methods with conflicting signatures, parameters do not match:

    note[E118]: Parameter `a` has conflicting type declarations:
       ┌─ <internal>:5:17
       │
     5 │                 a : REF_TO ARRAY[1..5] OF STRING;
       │                 - see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Type `__foo.bar_a_` declared in `foo.bar` but `fb.bar` declared type `__fb.bar_a_`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Parameter `b` has conflicting type declarations:
       ┌─ <internal>:6:17
       │
     6 │                 b : REF_TO ARRAY[1..5] OF ARRAY[1..5] OF STRING;
       │                 - see also
       ·
    12 │             METHOD bar
       │                    --- see also

    note[E118]: Type `__foo.bar_b_` declared in `foo.bar` but `fb.bar` declared type `__fb.bar_b_`
       ┌─ <internal>:3:20
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    12 │             METHOD bar
       │                    --- see also
    ");
}

#[test]
fn pou_implementing_all_methods_of_extended_interface_does_not_err() {
    let source = r"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS baz
            METHOD bar
            END_METHOD

            METHOD qux
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    assert!(diagnostics.is_empty(), "{:?}", diagnostics);
}

#[test]
fn pou_missing_methods_of_extended_interface() {
    let source = r"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS baz
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Method `bar` defined in interface `foo` is missing in POU `fb`
       ┌─ <internal>:12:24
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    12 │         FUNCTION_BLOCK fb IMPLEMENTS baz
       │                        ^^ Method `bar` defined in interface `foo` is missing in POU `fb`

    error[E112]: Method `qux` defined in interface `baz` is missing in POU `fb`
       ┌─ <internal>:12:24
       │
     8 │             METHOD qux
       │                    --- see also
       ·
    12 │         FUNCTION_BLOCK fb IMPLEMENTS baz
       │                        ^^ Method `qux` defined in interface `baz` is missing in POU `fb`
    ");
}

#[test]
fn pou_missing_methods_of_nested_extended_interface() {
    let source = r"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux EXTENDS baz
            METHOD corge
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS quux
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Method `corge` defined in interface `quux` is missing in POU `fb`
       ┌─ <internal>:17:24
       │
    13 │             METHOD corge
       │                    ----- see also
       ·
    17 │         FUNCTION_BLOCK fb IMPLEMENTS quux
       │                        ^^ Method `corge` defined in interface `quux` is missing in POU `fb`

    error[E112]: Method `bar` defined in interface `foo` is missing in POU `fb`
       ┌─ <internal>:17:24
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    17 │         FUNCTION_BLOCK fb IMPLEMENTS quux
       │                        ^^ Method `bar` defined in interface `foo` is missing in POU `fb`

    error[E112]: Method `qux` defined in interface `baz` is missing in POU `fb`
       ┌─ <internal>:17:24
       │
     8 │             METHOD qux
       │                    --- see also
       ·
    17 │         FUNCTION_BLOCK fb IMPLEMENTS quux
       │                        ^^ Method `qux` defined in interface `baz` is missing in POU `fb`
    ");
}

#[test]
fn pou_missing_methods_of_multiple_nested_interfaces() {
    let source = r"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD

            METHOD garply
            END_METHOD
        END_INTERFACE

        INTERFACE quxat
            METHOD waldo
            END_METHOD
        END_INTERFACE

        INTERFACE quxar EXTENDS quuz, baz, quxat
            METHOD fred
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS quxar
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Method `fred` defined in interface `quxar` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
    31 │             METHOD fred
       │                    ---- see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `fred` defined in interface `quxar` is missing in POU `fb`

    error[E112]: Method `garply` defined in interface `quuz` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
    21 │             METHOD garply
       │                    ------ see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `garply` defined in interface `quuz` is missing in POU `fb`

    error[E112]: Method `corge` defined in interface `quux` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
    13 │             METHOD corge
       │                    ----- see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `corge` defined in interface `quux` is missing in POU `fb`

    error[E112]: Method `waldo` defined in interface `quxat` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
    26 │             METHOD waldo
       │                    ----- see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `waldo` defined in interface `quxat` is missing in POU `fb`

    error[E112]: Method `grault` defined in interface `quuz` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
    18 │             METHOD grault
       │                    ------ see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `grault` defined in interface `quuz` is missing in POU `fb`

    error[E112]: Method `bar` defined in interface `foo` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
     3 │             METHOD bar
       │                    --- see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `bar` defined in interface `foo` is missing in POU `fb`

    error[E112]: Method `qux` defined in interface `baz` is missing in POU `fb`
       ┌─ <internal>:35:24
       │
     8 │             METHOD qux
       │                    --- see also
       ·
    35 │         FUNCTION_BLOCK fb IMPLEMENTS quxar
       │                        ^^ Method `qux` defined in interface `baz` is missing in POU `fb`
    ");
}
#[test]
fn pou_implementing_methods_of_multiple_nested_interfaces_does_not_err() {
    let source = r"
        INTERFACE foo
            METHOD bar
            END_METHOD
        END_INTERFACE

        INTERFACE baz EXTENDS foo
            METHOD qux
            END_METHOD
        END_INTERFACE

        INTERFACE quux
            METHOD corge
            END_METHOD
        END_INTERFACE

        INTERFACE quuz EXTENDS quux
            METHOD grault
            END_METHOD

            METHOD garply
            END_METHOD
        END_INTERFACE

        INTERFACE quxat
            METHOD waldo
            END_METHOD
        END_INTERFACE

        INTERFACE quxar EXTENDS quuz, baz, quxat
            METHOD fred
            END_METHOD
        END_INTERFACE
                
        FUNCTION_BLOCK fb IMPLEMENTS quxar
            METHOD bar
            END_METHOD
            METHOD qux
            END_METHOD
            METHOD corge
            END_METHOD
            METHOD grault
            END_METHOD
            METHOD garply
            END_METHOD
            METHOD waldo
            END_METHOD
            METHOD fred
            END_METHOD
        END_FUNCTION_BLOCK
        ";

    let diagnostics = parse_and_validate_buffered(source);
    assert!(diagnostics.is_empty(), "{:?}", diagnostics);
}

#[test]
fn interface_inheriting_undefined_interface() {
    let source = r"
        INTERFACE foo EXTENDS bar
            METHOD baz
            END_METHOD
        END_INTERFACE
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E048]: Interface `bar` does not exist
      ┌─ <internal>:2:31
      │
    2 │         INTERFACE foo EXTENDS bar
      │                               ^^^ Interface `bar` does not exist
    ");
}

#[test]
fn extended_interface_method_signature_mismatch() {
    let source = r"
        INTERFACE foo
            METHOD baz : DINT
            END_METHOD
        END_INTERFACE
        INTERFACE bar EXTENDS foo
            METHOD baz : STRING
            END_METHOD
        END_INTERFACE
        ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `baz` in `bar` is declared with conflicting signatures in `bar` and `foo`
      ┌─ <internal>:6:19
      │
    3 │             METHOD baz : DINT
      │                    --- see also
      ·
    6 │         INTERFACE bar EXTENDS foo
      │                   ^^^ Method `baz` in `bar` is declared with conflicting signatures in `bar` and `foo`
    7 │             METHOD baz : STRING
      │                    --- see also

    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:6:19
      │
    6 │         INTERFACE bar EXTENDS foo
      │                   ^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `STRING` declared in `bar.baz` but `foo.baz` declared type `DINT`
      ┌─ <internal>:3:20
      │
    3 │             METHOD baz : DINT
      │                    --- see also
      ·
    7 │             METHOD baz : STRING
      │                    --- see also
    ");
}

#[test]
fn interface_extending_multiple_interfaces_with_incompatible_method_signatures() {
    let source = r"
    INTERFACE foo
        METHOD baz : DINT
        END_METHOD
    END_INTERFACE
    INTERFACE bar
        METHOD baz : STRING
        END_METHOD
    END_INTERFACE
    INTERFACE qux EXTENDS foo, bar
    END_INTERFACE
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `baz` in `qux` is declared with conflicting signatures in `foo` and `bar`
       ┌─ <internal>:10:15
       │
     3 │         METHOD baz : DINT
       │                --- see also
       ·
     7 │         METHOD baz : STRING
       │                --- see also
       ·
    10 │     INTERFACE qux EXTENDS foo, bar
       │               ^^^ Method `baz` in `qux` is declared with conflicting signatures in `foo` and `bar`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:10:15
       │
    10 │     INTERFACE qux EXTENDS foo, bar
       │               ^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.baz` but `bar.baz` declared type `STRING`
      ┌─ <internal>:3:16
      │
    3 │         METHOD baz : DINT
      │                --- see also
      ·
    7 │         METHOD baz : STRING
      │                --- see also
    ");
}

#[test]
fn function_block_implementing_erroneous_interface() {
    let source = r"
    INTERFACE foo
        METHOD baz : DINT
        END_METHOD
    END_INTERFACE
    INTERFACE bar
        METHOD baz : STRING
        END_METHOD
    END_INTERFACE
    INTERFACE qux EXTENDS foo, bar
    END_INTERFACE
    FUNCTION_BLOCK quux IMPLEMENTS qux
    END_FUNCTION_BLOCK
    ";
    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"
    error[E111]: Method `baz` in `quux` is declared with conflicting signatures in `foo` and `bar`
       ┌─ <internal>:12:20
       │
     3 │         METHOD baz : DINT
       │                --- see also
       ·
     7 │         METHOD baz : STRING
       │                --- see also
       ·
    12 │     FUNCTION_BLOCK quux IMPLEMENTS qux
       │                    ^^^^ Method `baz` in `quux` is declared with conflicting signatures in `foo` and `bar`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:12:20
       │
    12 │     FUNCTION_BLOCK quux IMPLEMENTS qux
       │                    ^^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.baz` but `bar.baz` declared type `STRING`
      ┌─ <internal>:3:16
      │
    3 │         METHOD baz : DINT
      │                --- see also
      ·
    7 │         METHOD baz : STRING
      │                --- see also

    error[E111]: Method `baz` in `qux` is declared with conflicting signatures in `foo` and `bar`
       ┌─ <internal>:10:15
       │
     3 │         METHOD baz : DINT
       │                --- see also
       ·
     7 │         METHOD baz : STRING
       │                --- see also
       ·
    10 │     INTERFACE qux EXTENDS foo, bar
       │               ^^^ Method `baz` in `qux` is declared with conflicting signatures in `foo` and `bar`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
       ┌─ <internal>:10:15
       │
    10 │     INTERFACE qux EXTENDS foo, bar
       │               ^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `foo.baz` but `bar.baz` declared type `STRING`
      ┌─ <internal>:3:16
      │
    3 │         METHOD baz : DINT
      │                --- see also
      ·
    7 │         METHOD baz : STRING
      │                --- see also
    ");
}

#[test]
fn property_not_implemented() {
    let source = r"
    INTERFACE intf
        PROPERTY prop : DINT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS intf
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` (GET) defined in interface `intf` is missing in POU `fb`
      ┌─ <internal>:9:20
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │     FUNCTION_BLOCK fb IMPLEMENTS intf
      │                    ^^ Property `prop` (GET) defined in interface `intf` is missing in POU `fb`

    error[E112]: Property `prop` (SET) defined in interface `intf` is missing in POU `fb`
      ┌─ <internal>:9:20
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │     FUNCTION_BLOCK fb IMPLEMENTS intf
      │                    ^^ Property `prop` (SET) defined in interface `intf` is missing in POU `fb`
    ");
}

#[test]
fn property_partially_implemented() {
    let source = r"
    INTERFACE intf
        PROPERTY prop : DINT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_INTERFACE

    FUNCTION_BLOCK fb IMPLEMENTS intf
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` (SET) defined in interface `intf` is missing in POU `fb`
      ┌─ <internal>:9:20
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │     FUNCTION_BLOCK fb IMPLEMENTS intf
      │                    ^^ Property `prop` (SET) defined in interface `intf` is missing in POU `fb`
    ");
}

#[test]
fn property_with_conflicting_signatures() {
    let source = r"
    INTERFACE intf1
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intf2
        PROPERTY prop : STRING
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intf3 EXTENDS intf1, intf2
    END_INTERFACE
    ";

    insta::assert_snapshot!(parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `intf1` and `intf2` have different datatypes
       ┌─ <internal>:14:15
       │
     3 │         PROPERTY prop : DINT
       │                         ---- see also
       ·
     9 │         PROPERTY prop : STRING
       │                         ------ see also
       ·
    14 │     INTERFACE intf3 EXTENDS intf1, intf2
       │               ^^^^^ Property `prop` defined in interface `intf1` and `intf2` have different datatypes
    ");
}

#[test]
fn interface_with_property_set_extending_other_interface_with_property_get() {
    let source = r"
    INTERFACE intfA
        PROPERTY prop: DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intfB
        PROPERTY prop: DINT
            SET END_SET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intfC EXTENDS intfA, intfB
    END_INTERFACE

    FUNCTION_BLOCK fb1 IMPLEMENTS intfC
        PROPERTY prop: DINT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 IMPLEMENTS intfA, intfB
        PROPERTY prop: DINT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(parse_and_validate_buffered(source), @"");
}

#[test]
fn missing_property_accessor_implementation() {
    let source = r"
    INTERFACE intfA
        PROPERTY prop: DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intfB
        PROPERTY prop: DINT
            SET END_SET
        END_PROPERTY
    END_INTERFACE

    INTERFACE intfC EXTENDS intfA, intfB
    END_INTERFACE

    FUNCTION_BLOCK fb1 IMPLEMENTS intfC
        PROPERTY prop: DINT
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 IMPLEMENTS intfA, intfB
        PROPERTY prop: DINT
            SET END_SET
        END_PROPERTY
    END_FUNCTION_BLOCK
  ";

    insta::assert_snapshot!(parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` (SET) defined in interface `intfB` is missing in POU `fb1`
       ┌─ <internal>:17:20
       │
     9 │         PROPERTY prop: DINT
       │                  ---- see also
       ·
    17 │     FUNCTION_BLOCK fb1 IMPLEMENTS intfC
       │                    ^^^ Property `prop` (SET) defined in interface `intfB` is missing in POU `fb1`

    error[E112]: Property `prop` (GET) defined in interface `intfA` is missing in POU `fb2`
       ┌─ <internal>:23:20
       │
     3 │         PROPERTY prop: DINT
       │                  ---- see also
       ·
    23 │     FUNCTION_BLOCK fb2 IMPLEMENTS intfA, intfB
       │                    ^^^ Property `prop` (GET) defined in interface `intfA` is missing in POU `fb2`
    ");
}
