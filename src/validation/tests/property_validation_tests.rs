#[test]
fn invalid_pou_type() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION foo : DINT
            PROPERTY prop : DINT
                GET
                    prop := 5;
                END_GET
            END_PROPERTY
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E001]: Properties cannot be declared in a Function
      ┌─ <internal>:2:24
      │
    2 │         FUNCTION foo : DINT
      │                        ^^^^ Properties cannot be declared in a Function
    ");
}

#[test]
fn more_than_one_get_or_set_block() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK foo
            PROPERTY foo_prop : DINT
                GET END_GET
                GET END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar
            PROPERTY bar_prop : DINT
                SET END_SET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK baz
            PROPERTY baz_prop : DINT
                GET END_GET
                GET END_GET

                SET END_SET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );
    insta::assert_snapshot!(diagnostics, @r"
    error[E117]: Property has more than one GET block
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY foo_prop : DINT
      │                      ^^^^^^^^ Property has more than one GET block

    error[E117]: Property has more than one SET block
       ┌─ <internal>:10:22
       │
    10 │             PROPERTY bar_prop : DINT
       │                      ^^^^^^^^ Property has more than one SET block

    error[E117]: Property has more than one GET block
       ┌─ <internal>:17:22
       │
    17 │             PROPERTY baz_prop : DINT
       │                      ^^^^^^^^ Property has more than one GET block

    error[E117]: Property has more than one SET block
       ┌─ <internal>:17:22
       │
    17 │             PROPERTY baz_prop : DINT
       │                      ^^^^^^^^ Property has more than one SET block
    ");
}

#[test]
fn invalid_variable_block_type() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK foo
            PROPERTY prop : DINT
                GET
                    VAR_INPUT
                        var_get_in : DINT;
                    END_VAR

                    VAR_OUTPUT
                        var_get_out : DINT;
                    END_VAR

                    VAR_IN_OUT
                        var_get_inout : DINT;
                    END_VAR

                    VAR_TEMP
                        var_get_temp : DINT;
                    END_VAR

                    VAR
                        var_get_local : DINT;
                    END_VAR
                END_GET

                SET
                    VAR_INPUT
                        var_set_in : DINT;
                    END_VAR

                    VAR_OUTPUT
                        var_set_out : DINT;
                    END_VAR

                    VAR_IN_OUT
                        var_set_inout : DINT;
                    END_VAR

                    VAR_TEMP
                        var_set_temp : DINT;
                    END_VAR

                    VAR
                        var_set_local : DINT;
                    END_VAR
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E116]: Properties only allow variable blocks of type VAR
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Properties only allow variable blocks of type VAR
    4 │                 GET
    5 │                     VAR_INPUT
      │                     --------- see also

    error[E116]: Properties only allow variable blocks of type VAR
      ┌─ <internal>:3:22
      │
    3 │             PROPERTY prop : DINT
      │                      ^^^^ Properties only allow variable blocks of type VAR
      ·
    9 │                     VAR_OUTPUT
      │                     ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY prop : DINT
       │                      ^^^^ Properties only allow variable blocks of type VAR
       ·
    13 │                     VAR_IN_OUT
       │                     ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY prop : DINT
       │                      ^^^^ Properties only allow variable blocks of type VAR
       ·
    27 │                     VAR_INPUT
       │                     --------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY prop : DINT
       │                      ^^^^ Properties only allow variable blocks of type VAR
       ·
    31 │                     VAR_OUTPUT
       │                     ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY prop : DINT
       │                      ^^^^ Properties only allow variable blocks of type VAR
       ·
    35 │                     VAR_IN_OUT
       │                     ---------- see also
    ");
}

#[test]
fn name_clash_with_member_variable() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
            VAR
                foo: DINT;
            END_VAR

            PROPERTY foo : DINT
                GET
                    foo := 42;
                END_GET

                SET
                    foo := 3;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E004]: foo: Duplicate symbol.
      ┌─ <internal>:4:17
      │
    4 │                 foo: DINT;
      │                 ^^^ foo: Duplicate symbol.
      ·
    7 │             PROPERTY foo : DINT
      │                      --- see also

    error[E004]: foo: Duplicate symbol.
      ┌─ <internal>:7:22
      │
    4 │                 foo: DINT;
      │                 --- see also
      ·
    7 │             PROPERTY foo : DINT
      │                      ^^^ foo: Duplicate symbol.
    ");
}

#[test]
fn name_clash_with_parent_variable() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
      ┌─ <internal>:9:22
      │
    4 │                 foo: DINT;
      │                 --- see also
      ·
    9 │             PROPERTY foo : DINT
      │                      ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
    ");
}

#[test]
fn name_clash_with_child_variable() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY foo : DINT
       │                      ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
       ·
    11 │                 foo: DINT;
       │                 --- see also
    ");
}

#[test]
fn name_clash_with_property_in_parent_chained() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            VAR
                bar : REAL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb2
            VAR
                baz : STRING;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb4 EXTENDS fb3
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
       ┌─ <internal>:3:22
       │
     3 │             PROPERTY foo : DINT
       │                      ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
       ·
    23 │                 foo: DINT;
       │                 --- see also
    ");
}

#[test]
fn name_clash_with_variable_in_parent_chained() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            VAR
                foo: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            VAR
                bar : REAL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb3 EXTENDS fb2
            VAR
                baz : STRING;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb4 EXTENDS fb3
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
       ┌─ <internal>:21:22
       │
     4 │                 foo: DINT;
       │                 --- see also
       ·
    21 │             PROPERTY foo : DINT
       │                      ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
    ");
}

#[test]
fn overriding_property_in_function_block_with_same_datatype_is_ok() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY foo : DINT
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            PROPERTY foo : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    // Essentially we're overriding the property in the child, which is OK because properties are methods
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn overriding_property_in_function_block_with_different_datatype_is_not_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY prop : STRING
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Overridden property `prop` has different signatures in POU `fb2` and `fb1`
      ┌─ <internal>:9:18
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │         PROPERTY prop : STRING
      │                  ^^^^ Overridden property `prop` has different signatures in POU `fb2` and `fb1`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:9:18
      │
    9 │         PROPERTY prop : STRING
      │                  ^^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `fb1.__get_prop` but `fb2.__get_prop` declared type `STRING`
      ┌─ <internal>:3:18
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │         PROPERTY prop : STRING
      │                  ---- see also
    ");
}

#[test]
fn extending_property_in_function_block_by_accessor_with_same_datatype_is_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY prop : DINT
            SET END_SET
        END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    // Essentially we're extending fb2 by a `__set_prop` method which isn't defined in the parent
    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @"");
}

#[test]
fn extending_property_in_function_block_by_accessor_with_different_datatype_is_not_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY prop : INT
            SET END_SET
        END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Overridden property `prop` has different signatures in POU `fb2` and `fb1`
      ┌─ <internal>:9:18
      │
    3 │         PROPERTY prop : DINT
      │                  ---- see also
      ·
    9 │         PROPERTY prop : INT
      │                  ^^^^ Overridden property `prop` has different signatures in POU `fb2` and `fb1`
    ");
}

#[test]
fn overriding_property_in_interface_with_same_datatype_is_ok() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        INTERFACE intf1
            PROPERTY prop : DINT
                GET END_GET
            END_PROPERTY
        END_INTERFACE

        INTERFACE intf2 EXTENDS intf1
            PROPERTY prop : DINT
                GET END_GET
                SET END_SET
            END_PROPERTY
        END_INTERFACE
        ",
    );

    // Essentially we're overriding the property in the child, which is OK because properties are methods
    insta::assert_snapshot!(diagnostics, @"");
}

#[test]
fn overriding_property_in_interface_with_different_datatype_is_not_ok() {
    let source = r"
    INTERFACE intf1
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    // We extend the property by a SET accessor in this interface, but with the wrong datatype
    INTERFACE intf2 EXTENDS intf1
        PROPERTY prop : STRING 
            SET END_SET
        END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `intf1` and `intf2` have different datatypes
       ┌─ <internal>:9:15
       │
     3 │         PROPERTY prop : DINT
       │                         ---- see also
       ·
     9 │     INTERFACE intf2 EXTENDS intf1
       │               ^^^^^ Property `prop` defined in interface `intf1` and `intf2` have different datatypes
    10 │         PROPERTY prop : STRING 
       │                         ------ see also
    ");
}

#[test]
fn extending_interface_with_interfaces_with_conflicting_signatures_is_not_ok() {
    let source = r"
    INTERFACE A
        PROPERTY prop : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE B
        PROPERTY prop : STRING
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE C EXTENDS A, B
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `B` and `A` have different datatypes
       ┌─ <internal>:14:15
       │
     3 │         PROPERTY prop : DINT
       │                         ---- see also
       ·
     9 │         PROPERTY prop : STRING
       │                         ------ see also
       ·
    14 │     INTERFACE C EXTENDS A, B
       │               ^ Property `prop` defined in interface `B` and `A` have different datatypes
    ");
}

#[test]
fn multiple_levels() {
    let source = r"
    INTERFACE A
        PROPERTY propA : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE B
        PROPERTY propB : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE C EXTENDS A
        PROPERTY propC : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE

    INTERFACE D EXTENDS C
        PROPERTY propD : DINT
            GET END_GET
        END_PROPERTY
    END_INTERFACE
    
    // All of these are overrides with different signatures
    INTERFACE E EXTENDS B, C, A
        PROPERTY propA : REAL
            GET END_GET
            SET END_SET
        END_PROPERTY

        PROPERTY propB : STRING
            GET END_GET 
            SET END_SET
        END_PROPERTY

        PROPERTY propC : INT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_INTERFACE

    // These on the other hand are overrides, but with the same signature and hence OK
    INTERFACE F EXTENDS B, C, A
        PROPERTY propA : DINT
            GET END_GET
            SET END_SET
        END_PROPERTY

        PROPERTY propB : DINT
            GET END_GET
            SET END_SET
        END_PROPERTY

        PROPERTY propC : DINT
            GET END_GET
            SET END_SET
        END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `propA` defined in interface `E` and `A` have different datatypes
       ┌─ <internal>:27:15
       │
     3 │         PROPERTY propA : DINT
       │                          ---- see also
       ·
    27 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propA` defined in interface `E` and `A` have different datatypes
    28 │         PROPERTY propA : REAL
       │                          ---- see also

    error[E112]: Property `propA` defined in interface `A` and `E` have different datatypes
       ┌─ <internal>:27:15
       │
     3 │         PROPERTY propA : DINT
       │                          ---- see also
       ·
    27 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propA` defined in interface `A` and `E` have different datatypes
       ·
    38 │         PROPERTY propC : INT
       │                          --- see also

    error[E112]: Property `propC` defined in interface `E` and `C` have different datatypes
       ┌─ <internal>:27:15
       │
    15 │         PROPERTY propC : DINT
       │                          ---- see also
       ·
    27 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propC` defined in interface `E` and `C` have different datatypes
       ·
    38 │         PROPERTY propC : INT
       │                          --- see also

    error[E112]: Property `propC` defined in interface `C` and `E` have different datatypes
       ┌─ <internal>:27:15
       │
    15 │         PROPERTY propC : DINT
       │                          ---- see also
       ·
    27 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propC` defined in interface `C` and `E` have different datatypes
       ·
    33 │         PROPERTY propB : STRING
       │                          ------ see also

    error[E112]: Property `propB` defined in interface `E` and `B` have different datatypes
       ┌─ <internal>:27:15
       │
     9 │         PROPERTY propB : DINT
       │                          ---- see also
       ·
    27 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propB` defined in interface `E` and `B` have different datatypes
       ·
    33 │         PROPERTY propB : STRING
       │                          ------ see also
    ");
}

#[test]
fn undefined_references_inheritance() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            PROPERTY myProp : DINT
                GET END_GET
            END_PROPERTY

            myProp;         // Ok, this represents GET
            myProp := 5;    // Error, this represents a SET which is not defined in here
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            PROPERTY myProp : DINT
                SET END_SET
            END_PROPERTY

            myProp := 5;            // Ok, this represents a GET which is inherited from the parent
            myProp := myProp  + 1;  // Ok, this represents a SET that is overriden here
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                parent_fb: parent;
                child_fb: child;
            END_VAR

            parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a SET
            child_fb.myProp := 5;                   // Ok, the `child` FB does define a SET
            child_fb.myProp := parent_fb.myProp;    // Ok, the `child` FB does define a SET, the `parent` a GET
            child_fb.myProp := child_fb.myProp + 1; // Ok, the `child` FB does define a SET, inherits the GET from the parent
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E048]: Could not resolve reference to myProp
      ┌─ <internal>:8:13
      │
    8 │             myProp := 5;    // Error, this represents a SET which is not defined in here
      │             ^^^^^^ Could not resolve reference to myProp

    error[E048]: Could not resolve reference to myProp
       ┌─ <internal>:26:23
       │
    26 │             parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a SET
       │                       ^^^^^^ Could not resolve reference to myProp
    ");
}

#[test]
fn conflicting_signatures_in_head_and_tail_inheritance_chain() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fbA
            PROPERTY myProp : DINT
                GET END_GET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbB EXTENDS fbA
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbC EXTENDS fbB
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbD EXTENDS fbC
            PROPERTY myProp : STRING // Conflicting signature with A, where myProp has a `DINT` datatype
                SET END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Overridden property `myProp` has different signatures in POU `fbD` and `fbA`
       ┌─ <internal>:15:22
       │
     3 │             PROPERTY myProp : DINT
       │                      ------ see also
       ·
    15 │             PROPERTY myProp : STRING // Conflicting signature with A, where myProp has a `DINT` datatype
       │                      ^^^^^^ Overridden property `myProp` has different signatures in POU `fbD` and `fbA`
    ");
}
