#[test]
fn invalid_pou_type() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION foo : DINT
            PROPERTY_GET prop: DINT
                prop := 5;
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
            PROPERTY_GET foo_prop: DINT END_PROPERTY
            PROPERTY_GET foo_prop: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar
            PROPERTY_SET bar_prop: DINT END_PROPERTY
            PROPERTY_SET bar_prop: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK baz
            PROPERTY_GET baz_prop: DINT END_PROPERTY
            PROPERTY_GET baz_prop: DINT END_PROPERTY

            PROPERTY_SET baz_prop: DINT END_PROPERTY
            PROPERTY_SET baz_prop: DINT END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );
    insta::assert_snapshot!(diagnostics, @r"
    error[E004]: Property has multiple PROPERTY_GET blocks
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET foo_prop: DINT END_PROPERTY
      │                          ^^^^^^^^ Property has multiple PROPERTY_GET blocks

    error[E004]: Property has multiple PROPERTY_SET blocks
      ┌─ <internal>:8:26
      │
    8 │             PROPERTY_SET bar_prop: DINT END_PROPERTY
      │                          ^^^^^^^^ Property has multiple PROPERTY_SET blocks

    error[E004]: Property has multiple PROPERTY_GET blocks
       ┌─ <internal>:13:26
       │
    13 │             PROPERTY_GET baz_prop: DINT END_PROPERTY
       │                          ^^^^^^^^ Property has multiple PROPERTY_GET blocks

    error[E004]: Property has multiple PROPERTY_SET blocks
       ┌─ <internal>:13:26
       │
    13 │             PROPERTY_GET baz_prop: DINT END_PROPERTY
       │                          ^^^^^^^^ Property has multiple PROPERTY_SET blocks
    ");
}

#[test]
fn invalid_variable_block_type() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK foo
            PROPERTY_GET prop: DINT
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
            END_PROPERTY

            PROPERTY_SET prop: DINT
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
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E116]: Properties only allow variable blocks of type VAR
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET prop: DINT
      │                          ^^^^ Properties only allow variable blocks of type VAR
    4 │                 VAR_INPUT
      │                 --------- see also

    error[E116]: Properties only allow variable blocks of type VAR
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET prop: DINT
      │                          ^^^^ Properties only allow variable blocks of type VAR
      ·
    8 │                 VAR_OUTPUT
      │                 ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:26
       │
     3 │             PROPERTY_GET prop: DINT
       │                          ^^^^ Properties only allow variable blocks of type VAR
       ·
    12 │                 VAR_IN_OUT
       │                 ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:26
       │
     3 │             PROPERTY_GET prop: DINT
       │                          ^^^^ Properties only allow variable blocks of type VAR
       ·
    26 │                 VAR_INPUT
       │                 --------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:26
       │
     3 │             PROPERTY_GET prop: DINT
       │                          ^^^^ Properties only allow variable blocks of type VAR
       ·
    30 │                 VAR_OUTPUT
       │                 ---------- see also

    error[E116]: Properties only allow variable blocks of type VAR
       ┌─ <internal>:3:26
       │
     3 │             PROPERTY_GET prop: DINT
       │                          ^^^^ Properties only allow variable blocks of type VAR
       ·
    34 │                 VAR_IN_OUT
       │                 ---------- see also
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

            PROPERTY_GET foo: DINT
                foo := 42;
            END_PROPERTY

            PROPERTY_SET foo: DINT
                foo := 3;
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
    7 │             PROPERTY_GET foo: DINT
      │                          --- see also

    error[E004]: foo: Duplicate symbol.
      ┌─ <internal>:7:26
      │
    4 │                 foo: DINT;
      │                 --- see also
      ·
    7 │             PROPERTY_GET foo: DINT
      │                          ^^^ foo: Duplicate symbol.
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
            PROPERTY_GET foo: DINT END_PROPERTY
            PROPERTY_SET foo: DINT END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
      ┌─ <internal>:9:26
      │
    4 │                 foo: DINT;
      │                 --- see also
      ·
    9 │             PROPERTY_GET foo: DINT END_PROPERTY
      │                          ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
    ");
}

#[test]
fn name_clash_with_child_variable() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY_GET foo: DINT END_PROPERTY
            PROPERTY_SET foo: DINT END_PROPERTY
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
      ┌─ <internal>:3:26
      │
    3 │             PROPERTY_GET foo: DINT END_PROPERTY
      │                          ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb2`
      ·
    9 │                 foo: DINT;
      │                 --- see also
    ");
}

#[test]
fn name_clash_with_property_in_parent_chained() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY_GET foo: DINT END_PROPERTY
            PROPERTY_SET foo: DINT END_PROPERTY
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
       ┌─ <internal>:3:26
       │
     3 │             PROPERTY_GET foo: DINT END_PROPERTY
       │                          ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
       ·
    21 │                 foo: DINT;
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
            PROPERTY_GET foo: DINT END_PROPERTY
            PROPERTY_SET foo: DINT END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E021]: Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
       ┌─ <internal>:21:26
       │
     4 │                 foo: DINT;
       │                 --- see also
       ·
    21 │             PROPERTY_GET foo: DINT END_PROPERTY
       │                          ^^^ Name conflict between property and variable `foo` defined in POU `fb1` and `fb4`
    ");
}

#[test]
fn overriding_property_in_function_block_with_same_datatype_is_ok() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb1
            PROPERTY_SET foo: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb1
            PROPERTY_GET foo: DINT END_PROPERTY
            PROPERTY_SET foo: DINT END_PROPERTY
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
        PROPERTY_GET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY_GET prop: STRING END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Overridden property `prop` has different signatures in POU `fb2` and `fb1`
      ┌─ <internal>:7:22
      │
    3 │         PROPERTY_GET prop: DINT END_PROPERTY
      │                      ---- see also
      ·
    7 │         PROPERTY_GET prop: STRING END_PROPERTY
      │                      ^^^^ Overridden property `prop` has different signatures in POU `fb2` and `fb1`

    error[E112]: Derived methods with conflicting signatures, return types do not match:
      ┌─ <internal>:7:22
      │
    7 │         PROPERTY_GET prop: STRING END_PROPERTY
      │                      ^^^^ Derived methods with conflicting signatures, return types do not match:

    note[E118]: Type `DINT` declared in `fb1.__get_prop` but `fb2.__get_prop` declared type `STRING`
      ┌─ <internal>:3:22
      │
    3 │         PROPERTY_GET prop: DINT END_PROPERTY
      │                      ---- see also
      ·
    7 │         PROPERTY_GET prop: STRING END_PROPERTY
      │                      ---- see also
    ");
}

#[test]
fn extending_property_in_function_block_by_accessor_with_same_datatype_is_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY_GET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY_SET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    // Essentially we're extending fb2 by a `__set_prop` method which isn't defined in the parent
    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @"");
}

#[test]
fn extending_property_in_function_block_by_accessor_with_different_datatype_is_not_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY_GET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY_SET prop: INT END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Overridden property `prop` has different signatures in POU `fb2` and `fb1`
      ┌─ <internal>:7:22
      │
    3 │         PROPERTY_GET prop: DINT END_PROPERTY
      │                      ---- see also
      ·
    7 │         PROPERTY_SET prop: INT END_PROPERTY
      │                      ^^^^ Overridden property `prop` has different signatures in POU `fb2` and `fb1`
    ");
}

#[test]
fn extending_property_in_function_block_by_getter_with_same_datatype_is_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY_SET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY_GET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @"");
}

#[test]
fn extending_property_in_function_block_by_getter_with_different_datatype_is_not_ok() {
    let source = r"
    FUNCTION_BLOCK fb1
        PROPERTY_SET prop: DINT END_PROPERTY
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb2 EXTENDS fb1
        PROPERTY_GET prop: INT END_PROPERTY
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Overridden property `prop` has different signatures in POU `fb2` and `fb1`
      ┌─ <internal>:7:22
      │
    3 │         PROPERTY_SET prop: DINT END_PROPERTY
      │                      ---- see also
      ·
    7 │         PROPERTY_GET prop: INT END_PROPERTY
      │                      ^^^^ Overridden property `prop` has different signatures in POU `fb2` and `fb1`
    ");
}

#[test]
fn overriding_property_in_interface_with_same_datatype_is_ok() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        INTERFACE intf1
            PROPERTY_GET prop: DINT END_PROPERTY
        END_INTERFACE

        INTERFACE intf2 EXTENDS intf1
            PROPERTY_GET prop: DINT END_PROPERTY
            PROPERTY_SET prop: DINT END_PROPERTY
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
        PROPERTY_GET prop: DINT END_PROPERTY
    END_INTERFACE

    // We extend the property by a PROPERTY_SET accessor in this interface, but with the wrong datatype
    INTERFACE intf2 EXTENDS intf1
        PROPERTY_SET prop: STRING END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `intf1` and `intf2` have different datatypes
      ┌─ <internal>:7:15
      │
    3 │         PROPERTY_GET prop: DINT END_PROPERTY
      │                            ---- see also
      ·
    7 │     INTERFACE intf2 EXTENDS intf1
      │               ^^^^^ Property `prop` defined in interface `intf1` and `intf2` have different datatypes
    8 │         PROPERTY_SET prop: STRING END_PROPERTY
      │                            ------ see also
    ");
}

#[test]
fn properties_with_same_name_but_different_datatypes_are_not_ok() {
    let source = r"
    INTERFACE A
        PROPERTY_GET propertyA: INT END_PROPERTY
        PROPERTY_SET propertyA: DINT END_PROPERTY
    END_INTERFACE

    FUNCTION_BLOCK B
        PROPERTY_GET propertyB: DINT END_PROPERTY
        PROPERTY_SET propertyB: INT END_PROPERTY
    END_FUNCTION_BLOCK

    CLASS C
        PROPERTY_GET propertyC: SINT END_PROPERTY
        PROPERTY_SET propertyC: DINT END_PROPERTY
    END_CLASS
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `propertyB` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
      ┌─ <internal>:8:22
      │
    8 │         PROPERTY_GET propertyB: DINT END_PROPERTY
      │                      ^^^^^^^^^  ---- see also
      │                      │
      │                      Property `propertyB` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
    9 │         PROPERTY_SET propertyB: INT END_PROPERTY
      │                                 --- see also

    error[E112]: Property `propertyC` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
       ┌─ <internal>:13:22
       │
    13 │         PROPERTY_GET propertyC: SINT END_PROPERTY
       │                      ^^^^^^^^^  ---- see also
       │                      │
       │                      Property `propertyC` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
    14 │         PROPERTY_SET propertyC: DINT END_PROPERTY
       │                                 ---- see also

    error[E112]: Property `propertyA` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
      ┌─ <internal>:3:22
      │
    3 │         PROPERTY_GET propertyA: INT END_PROPERTY
      │                      ^^^^^^^^^  --- see also
      │                      │
      │                      Property `propertyA` has conflicting datatypes across PROPERTY_GET / PROPERTY_SET
    4 │         PROPERTY_SET propertyA: DINT END_PROPERTY
      │                                 ---- see also
    ");
}

#[test]
fn extending_interface_property_by_getter_with_same_datatype_is_ok() {
    let source = r"
    INTERFACE intf1
        PROPERTY_SET prop: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE intf2 EXTENDS intf1
        PROPERTY_GET prop: DINT END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @"");
}

#[test]
fn extending_interface_property_by_getter_with_different_datatype_is_not_ok() {
    let source = r"
    INTERFACE intf1
        PROPERTY_SET prop: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE intf2 EXTENDS intf1
        PROPERTY_GET prop: STRING END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `intf2` and `intf1` have different datatypes
      ┌─ <internal>:6:15
      │
    3 │         PROPERTY_SET prop: DINT END_PROPERTY
      │                            ---- see also
      ·
    6 │     INTERFACE intf2 EXTENDS intf1
      │               ^^^^^ Property `prop` defined in interface `intf2` and `intf1` have different datatypes
    7 │         PROPERTY_GET prop: STRING END_PROPERTY
      │                            ------ see also
    ");
}

#[test]
fn extending_interface_with_interfaces_with_conflicting_signatures_is_not_ok() {
    let source = r"
    INTERFACE A
        PROPERTY_GET prop: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE B
        PROPERTY_GET prop: STRING END_PROPERTY
    END_INTERFACE

    INTERFACE C EXTENDS A, B
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `prop` defined in interface `A` and `B` have different datatypes
       ┌─ <internal>:10:15
       │
     3 │         PROPERTY_GET prop: DINT END_PROPERTY
       │                            ---- see also
       ·
     7 │         PROPERTY_GET prop: STRING END_PROPERTY
       │                            ------ see also
       ·
    10 │     INTERFACE C EXTENDS A, B
       │               ^ Property `prop` defined in interface `A` and `B` have different datatypes
    ");
}

#[test]
fn multiple_levels() {
    let source = r"
    INTERFACE A
        PROPERTY_GET propA: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE B
        PROPERTY_GET propB: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE C EXTENDS A
        PROPERTY_GET propC: DINT END_PROPERTY
    END_INTERFACE

    INTERFACE DD EXTENDS C
        PROPERTY_GET propD: DINT END_PROPERTY
    END_INTERFACE

    // All of these are overrides with different signatures
    INTERFACE E EXTENDS B, C, A
        PROPERTY_GET propA: REAL END_PROPERTY
        PROPERTY_SET propA: REAL END_PROPERTY

        PROPERTY_GET propB: STRING END_PROPERTY
        PROPERTY_SET propB: STRING END_PROPERTY

        PROPERTY_GET propC: INT END_PROPERTY
        PROPERTY_SET propC: INT END_PROPERTY
    END_INTERFACE

    // These on the other hand are overrides, but with the same signature and hence OK
    INTERFACE F EXTENDS B, C, A
        PROPERTY_GET propA: DINT END_PROPERTY
        PROPERTY_SET propA: DINT END_PROPERTY

        PROPERTY_GET propB: DINT END_PROPERTY
        PROPERTY_SET propB: DINT END_PROPERTY

        PROPERTY_GET propC: DINT END_PROPERTY
        PROPERTY_SET propC: DINT END_PROPERTY
    END_INTERFACE
    ";

    insta::assert_snapshot!(test_utils::parse_and_validate_buffered(source), @r"
    error[E112]: Property `propA` defined in interface `E` and `A` have different datatypes
       ┌─ <internal>:19:15
       │
     3 │         PROPERTY_GET propA: DINT END_PROPERTY
       │                             ---- see also
       ·
    19 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propA` defined in interface `E` and `A` have different datatypes
    20 │         PROPERTY_GET propA: REAL END_PROPERTY
       │                             ---- see also

    error[E112]: Property `propA` defined in interface `A` and `E` have different datatypes
       ┌─ <internal>:19:15
       │
     3 │         PROPERTY_GET propA: DINT END_PROPERTY
       │                             ---- see also
       ·
    19 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propA` defined in interface `A` and `E` have different datatypes
       ·
    26 │         PROPERTY_GET propC: INT END_PROPERTY
       │                             --- see also

    error[E112]: Property `propC` defined in interface `E` and `C` have different datatypes
       ┌─ <internal>:19:15
       │
    11 │         PROPERTY_GET propC: DINT END_PROPERTY
       │                             ---- see also
       ·
    19 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propC` defined in interface `E` and `C` have different datatypes
       ·
    26 │         PROPERTY_GET propC: INT END_PROPERTY
       │                             --- see also

    error[E112]: Property `propC` defined in interface `C` and `E` have different datatypes
       ┌─ <internal>:19:15
       │
    11 │         PROPERTY_GET propC: DINT END_PROPERTY
       │                             ---- see also
       ·
    19 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propC` defined in interface `C` and `E` have different datatypes
       ·
    23 │         PROPERTY_GET propB: STRING END_PROPERTY
       │                             ------ see also

    error[E112]: Property `propB` defined in interface `E` and `B` have different datatypes
       ┌─ <internal>:19:15
       │
     7 │         PROPERTY_GET propB: DINT END_PROPERTY
       │                             ---- see also
       ·
    19 │     INTERFACE E EXTENDS B, C, A
       │               ^ Property `propB` defined in interface `E` and `B` have different datatypes
       ·
    23 │         PROPERTY_GET propB: STRING END_PROPERTY
       │                             ------ see also
    ");
}

#[test]
fn undefined_references_inheritance() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            PROPERTY_GET myProp: DINT
            END_PROPERTY

            myProp;         // Ok, this represents PROPERTY_GET
            myProp := 5;    // Error, this represents a PROPERTY_SET which is not defined in here
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            PROPERTY_SET myProp: DINT
            END_PROPERTY

            myProp := 5;            // Ok, this represents a PROPERTY_GET which is inherited from the parent
            myProp := myProp  + 1;  // Ok, this represents a PROPERTY_SET that is overriden here
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                parent_fb: parent;
                child_fb: child;
            END_VAR

            parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a PROPERTY_SET
            child_fb.myProp := 5;                   // Ok, the `child` FB does define a PROPERTY_SET
            child_fb.myProp := parent_fb.myProp;    // Ok, the `child` FB does define a PROPERTY_SET, the `parent` a PROPERTY_GET
            child_fb.myProp := child_fb.myProp + 1; // Ok, the `child` FB does define a PROPERTY_SET, inherits the PROPERTY_GET from the parent
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E048]: PROPERTY_SET for property `myProp` is not defined
      ┌─ <internal>:7:13
      │
    7 │             myProp := 5;    // Error, this represents a PROPERTY_SET which is not defined in here
      │             ^^^^^^ PROPERTY_SET for property `myProp` is not defined

    error[E048]: PROPERTY_SET for property `myProp` is not defined
       ┌─ <internal>:24:23
       │
    24 │             parent_fb.myProp := 5;                  // Error, the `parent` FB does not define a PROPERTY_SET
       │                       ^^^^^^ PROPERTY_SET for property `myProp` is not defined
    ");
}

#[test]
fn missing_property_get_reports_property_diagnostic() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
            PROPERTY_SET foo: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance : fb;
                x : DINT;
            END_VAR

            x := instance.foo;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E048]: PROPERTY_GET for property `foo` is not defined
       ┌─ <internal>:12:27
       │
    12 │             x := instance.foo;
       │                           ^^^ PROPERTY_GET for property `foo` is not defined
    ");
}

#[test]
fn missing_property_set_reports_property_diagnostic() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
            PROPERTY_GET foo: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance : fb;
                x : DINT;
            END_VAR

            instance.foo := 5;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @"
    error[E048]: PROPERTY_SET for property `foo` is not defined
       ┌─ <internal>:12:22
       │
    12 │             instance.foo := 5;
       │                      ^^^ PROPERTY_SET for property `foo` is not defined
    ");
}

#[test]
fn conflicting_signatures_in_head_and_tail_inheritance_chain() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        "
        FUNCTION_BLOCK fbA
            PROPERTY_GET myProp: DINT END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbB EXTENDS fbA
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbC EXTENDS fbB
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fbD EXTENDS fbC
            PROPERTY_SET myProp: STRING END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E112]: Overridden property `myProp` has different signatures in POU `fbD` and `fbA`
       ┌─ <internal>:13:26
       │
     3 │             PROPERTY_GET myProp: DINT END_PROPERTY
       │                          ------ see also
       ·
    13 │             PROPERTY_SET myProp: STRING END_PROPERTY
       │                          ^^^^^^ Overridden property `myProp` has different signatures in POU `fbD` and `fbA`
    ");
}

#[test]
fn direct_property_assignment_is_allowed() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
                value: Position;
            END_VAR

            instance.position := value;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn property_self_assignment_remains_allowed() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
            END_VAR

            instance.position := instance.position;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn property_get_followed_by_member_access_remains_allowed() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
                value: DINT;
            END_VAR

            value := instance.position.x;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn property_get_inside_lhs_index_expression_remains_allowed() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
                arr: ARRAY[1..10] OF DINT;
            END_VAR

            arr[instance.position.x] := 5;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"");
}

#[test]
fn nested_member_assignment_through_property_is_rejected() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
            END_VAR

            instance.position.x := 5;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E128]: Properties can only be assigned as a whole, not through member or index access
       ┌─ <internal>:19:31
       │
    19 │             instance.position.x := 5;
       │                               ^ Properties can only be assigned as a whole, not through member or index access

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:19:31
       │
    19 │             instance.position.x := 5;
       │                               ^ Could not resolve reference to x
    ");
}

#[test]
fn property_on_target_chain_but_property_get_in_index_expression_is_still_rejected() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET values: ARRAY[1..10] OF DINT END_PROPERTY
            PROPERTY_SET values: ARRAY[1..10] OF DINT END_PROPERTY

            PROPERTY_GET position: Position END_PROPERTY
            PROPERTY_SET position: Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
            END_VAR

            instance.values[instance.position.x] := 5;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E128]: Properties can only be assigned as a whole, not through member or index access
       ┌─ <internal>:22:22
       │
    22 │             instance.values[instance.position.x] := 5;
       │                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ Properties can only be assigned as a whole, not through member or index access
    ");
}

#[test]
fn property_returning_array_of_structs_followed_by_index_and_member_assignment_is_rejected() {
    let diagnostics = test_utils::parse_and_validate_buffered(
        r"
        TYPE Position:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            PROPERTY_GET positions: ARRAY[1..5] OF Position END_PROPERTY
            PROPERTY_SET positions: ARRAY[1..5] OF Position END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
            END_VAR

            instance.positions[1].x := 5;
        END_FUNCTION
        ",
    );

    insta::assert_snapshot!(diagnostics, @r"
    error[E128]: Properties can only be assigned as a whole, not through member or index access
       ┌─ <internal>:19:35
       │
    19 │             instance.positions[1].x := 5;
       │                                   ^ Properties can only be assigned as a whole, not through member or index access

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:19:35
       │
    19 │             instance.positions[1].x := 5;
       │                                   ^ Could not resolve reference to x
    ");
}
