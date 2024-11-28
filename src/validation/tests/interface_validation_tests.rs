use crate::test_utils::tests::parse_and_validate_buffered;

#[test]
fn pou_implementing_non_existing_interfaces() {
    let source = r"
    FUNCTION_BLOCK foo IMPLEMENTS delulu /* ... */ END_FUNCTION_BLOCK
    FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E048]: Interface `delulu` does not exist
      ┌─ <internal>:2:35
      │
    2 │     FUNCTION_BLOCK foo IMPLEMENTS delulu /* ... */ END_FUNCTION_BLOCK
      │                                   ^^^^^^ Interface `delulu` does not exist

    error[E048]: Interface `delulu` does not exist
      ┌─ <internal>:3:35
      │
    3 │     FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
      │                                   ^^^^^^ Interface `delulu` does not exist

    error[E048]: Interface `delululu` does not exist
      ┌─ <internal>:3:43
      │
    3 │     FUNCTION_BLOCK bar IMPLEMENTS delulu, delululu /* ... */ END_FUNCTION_BLOCK
      │                                           ^^^^^^^^ Interface `delululu` does not exist

    "###);
}

#[test]
fn pou_implementing_same_interface_multiple_times() {
    let source = r"
    INTERFACE interfaceA /* ... */ END_INTERFACE
    FUNCTION_BLOCK foo IMPLEMENTS interfaceA, interfaceA /* ... */ END_FUNCTION_BLOCK
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r"");
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E110]: Interfaces can only be implemented by classes or function blocks
       ┌─ <internal>:10:32
       │
    10 │     PROGRAM     baz IMPLEMENTS interfaceA            /* ... */ END_PROGRAM
       │                                ^^^^^^^^^^ Interfaces can only be implemented by classes or function blocks

    error[E110]: Interfaces can only be implemented by classes or function blocks
       ┌─ <internal>:11:32
       │
    11 │     FUNCTION    qux IMPLEMENTS interfaceA, interfaceB /* ... */ END_FUNCTION
       │                                ^^^^^^^^^^^^^^^^^^^^^^ Interfaces can only be implemented by classes or function blocks

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Return type of `fb.methodA` does not match the return type of the method defined in `interfaceA`, expected `DINT` but got `BOOL` instead
      ┌─ <internal>:7:16
      │
    3 │         METHOD methodA : DINT /* ... */ END_METHOD
      │                ------- see also
      ·
    7 │         METHOD methodA : BOOL /* ... */ END_METHOD
      │                ^^^^^^^ Return type of `fb.methodA` does not match the return type of the method defined in `interfaceA`, expected `DINT` but got `BOOL` instead

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Method implementation of `methodA` missing in POU `fb`
      ┌─ <internal>:6:20
      │
    3 │         METHOD methodA /* ... */ END_METHOD
      │                ------- see also
      ·
    6 │     FUNCTION_BLOCK fb IMPLEMENTS interfaceA
      │                    ^^ Method implementation of `methodA` missing in POU `fb`

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Parameter `c : DINT` missing in method `fb.methodA`
       ┌─ <internal>:13:16
       │
     7 │                 c : DINT;
       │                 - see also
       ·
    13 │         METHOD methodA
       │                ^^^^^^^ Parameter `c : DINT` missing in method `fb.methodA`

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Expected parameter `b` but got `a`
       ┌─ <internal>:5:17
       │
     5 │                 b : DINT;
       │                 ^ Expected parameter `b` but got `a`
       ·
    15 │                 a : DINT;
       │                 - see also

    error[E112]: Expected parameter `a` but got `b`
       ┌─ <internal>:6:17
       │
     6 │                 a : DINT;
       │                 ^ Expected parameter `a` but got `b`
       ·
    16 │                 b : DINT;
       │                 - see also

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Expected parameter `a` to have `DINT` as its type but got `BOOL`
       ┌─ <internal>:11:16
       │
     5 │                 a : DINT;
       │                 - see also
       ·
    11 │         METHOD methodA
       │                ^^^^^^^ Expected parameter `a` to have `DINT` as its type but got `BOOL`

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Expected parameter `a` to have `Input` as its declaration type but got `InOut`
       ┌─ <internal>:11:16
       │
     5 │                 a : DINT;
       │                 - see also
       ·
    11 │         METHOD methodA
       │                ^^^^^^^ Expected parameter `a` to have `Input` as its declaration type but got `InOut`

    "###);
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
                a : DINT;
                b : DINT;
                c : DINT;

                // Invalid parameters
                d : DINT;
                e : DINT;
                f : DINT;
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    let diagnostics = parse_and_validate_buffered(source);
    insta::assert_snapshot!(diagnostics, @r###"
    error[E112]: Parameter `d` defined in `fb` but not in `interfaceA`
       ┌─ <internal>:20:17
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    20 │                 d : DINT;
       │                 ^ Parameter `d` defined in `fb` but not in `interfaceA`

    error[E112]: Parameter `e` defined in `fb` but not in `interfaceA`
       ┌─ <internal>:21:17
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    21 │                 e : DINT;
       │                 ^ Parameter `e` defined in `fb` but not in `interfaceA`

    error[E112]: Parameter `f` defined in `fb` but not in `interfaceA`
       ┌─ <internal>:22:17
       │
     3 │         METHOD methodA
       │                ------- see also
       ·
    22 │                 f : DINT;
       │                 ^ Parameter `f` defined in `fb` but not in `interfaceA`

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:3:16
       │
     3 │         METHOD foo : INT
       │                ^^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ·
    12 │         METHOD foo : DINT
       │                --- see also

    error[E112]: Return type of `interfaceB.foo` does not match the return type of the method defined in `interfaceA`, expected `INT` but got `DINT` instead
       ┌─ <internal>:12:16
       │
     3 │         METHOD foo : INT
       │                --- see also
       ·
    12 │         METHOD foo : DINT
       │                ^^^ Return type of `interfaceB.foo` does not match the return type of the method defined in `interfaceA`, expected `INT` but got `DINT` instead

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:3:16
       │
     3 │         METHOD foo : INT
       │                ^^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ·
    12 │         METHOD foo : INT
       │                --- see also

    error[E112]: Expected parameter `b` to have `INT` as its type but got `DINT`
       ┌─ <internal>:12:16
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │         METHOD foo : INT
       │                ^^^ Expected parameter `b` to have `INT` as its type but got `DINT`

    "###);
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
    insta::assert_snapshot!(diagnostics, @r###"
    error[E111]: Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ┌─ <internal>:3:16
       │
     3 │         METHOD foo : INT
       │                ^^^ Method `foo` is defined with different signatures in interfaces `interfaceA` and `interfaceB`
       ·
    12 │         METHOD foo : INT
       │                --- see also

    error[E112]: Expected parameter `a` to have `Input` as its declaration type but got `Output`
       ┌─ <internal>:12:16
       │
     5 │                 a : INT;
       │                 - see also
       ·
    12 │         METHOD foo : INT
       │                ^^^ Expected parameter `a` to have `Input` as its declaration type but got `Output`

    error[E112]: Parameter `b : INT` missing in method `interfaceB.foo`
       ┌─ <internal>:12:16
       │
     6 │                 b : INT;
       │                 - see also
       ·
    12 │         METHOD foo : INT
       │                ^^^ Parameter `b : INT` missing in method `interfaceB.foo`

    "###);
}
