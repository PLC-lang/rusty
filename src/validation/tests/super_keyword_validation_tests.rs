use insta::assert_snapshot;
use test_utils::parse_and_validate_buffered;

#[test]
fn accessing_derived_member_through_dereferenced_super_is_no_illegal_access() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.x := 2;
            SUPER^.y := SUPER^.x + 1;
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn chaining_super_is_invalid() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:16:13
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │             ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:16:32
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:16:39
       │
    16 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                       ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
    ");
}

#[test]
fn chained_super_references_still_report_unresolved_references() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS greatgrandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:12:13
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │             ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:12:27
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                           ^ Could not resolve reference to x

    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:12:32
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:12:39
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                       ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E048]: Could not resolve reference to y
       ┌─ <internal>:12:53
       │
    12 │             SUPER^.SUPER^.x := SUPER^.SUPER^.SUPER^.y;
       │                                                     ^ Could not resolve reference to y
    ");
}

#[test]
fn super_accessor_used_in_non_extended_function_block_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK fb
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:3:13
      │
    3 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:4:13
      │
    4 │             SUPER;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:5:13
      │
    5 │             SUPER^;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.
    ");
}

#[test]
fn super_keyword_used_in_non_extendable_pous_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION foo
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_FUNCTION

        PROGRAM prog
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_PROGRAM

        ACTION prog.act
            SUPER^.x := 2;
            SUPER;
            SUPER^;
        END_ACTION
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:3:13
      │
    3 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:4:13
      │
    4 │             SUPER;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:5:13
      │
    5 │             SUPER^;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
      ┌─ <internal>:9:13
      │
    9 │             SUPER^.x := 2;
      │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:10:13
       │
    10 │             SUPER;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:11:13
       │
    11 │             SUPER^;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:15:13
       │
    15 │             SUPER^.x := 2;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:16:13
       │
    16 │             SUPER;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:17:13
       │
    17 │             SUPER^;
       │             ^^^^^ `SUPER` can only be used in POUs that extend another POU.
    ");
}

#[test]
fn super_keyword_is_not_assignable() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK child EXTENDS parent
        VAR
            super_inst: parent;
            super_ptr: REF_TO parent;
        END_VAR
            SUPER^ := super_inst;
            SUPER := super_ptr;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E050]: Expression SUPER is not assignable.
      ┌─ <internal>:9:13
      │
    9 │             SUPER^ := super_inst;
      │             ^^^^^ Expression SUPER is not assignable.

    error[E050]: Expression SUPER is not assignable.
       ┌─ <internal>:10:13
       │
    10 │             SUPER := super_ptr;
       │             ^^^^^ Expression SUPER is not assignable.
    ");
}

#[test]
fn super_accessor_cannot_be_accessed_from_outside_of_its_pou() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                x: INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK main
            VAR
                fb: child;
            END_VAR
            fb.SUPER^.x := 2;
            fb.SUPER.x := 2;
            fb.SUPER^ := 2;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:15:16
       │
    15 │             fb.SUPER^.x := 2;
       │                ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:15:23
       │
    15 │             fb.SUPER^.x := 2;
       │                       ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:16:16
       │
    16 │             fb.SUPER.x := 2;
       │                ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E048]: Could not resolve reference to x
       ┌─ <internal>:16:22
       │
    16 │             fb.SUPER.x := 2;
       │                      ^ Could not resolve reference to x

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:17:16
       │
    17 │             fb.SUPER^ := 2;
       │                ^^^^^ `SUPER` can only be used in POUs that extend another POU.
    ")
}

#[test]
fn super_reference_can_be_assigned_to_a_variable() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                x: INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                fb: parent;
                super_ref: REF_TO parent;
            END_VAR
            super_ref := SUPER;
            fb := SUPER^;
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn derefed_super_assigned_to_ptr_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
        // If this changes to LINT (i.e. 64-bit), the error goes away.
        // tracked in #1441
            VAR
                x: DINT; 
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                fb: REF_TO parent;
            END_VAR
            fb := SUPER^;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E065]: The type parent 32 is too small to be stored in a Pointer
       ┌─ <internal>:14:13
       │
    14 │             fb := SUPER^;
       │             ^^^^^^^^^^^ The type parent 32 is too small to be stored in a Pointer
    ");
}

#[test]
fn super_ref_assigned_to_value_type_is_an_error() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
          // If this changes to LINT (i.e. 64-bit), the error goes away.
          // tracked in #1441
            VAR
                x: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                fb: parent;
            END_VAR
            fb := SUPER;
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E065]: The type parent 32 is too small to hold a Pointer
       ┌─ <internal>:14:13
       │
    14 │             fb := SUPER;
       │             ^^^^^^^^^^^ The type parent 32 is too small to hold a Pointer
    ");
}

#[test]
fn super_accessing_private_methods() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR

            // I don't think the `PRIVATE` keyword does anything at this time,
            // but it can't hurt to have this covered anyway 
            METHOD PRIVATE do_something : INT
                do_something := x * 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test : INT
                // Should be able to access parent's private method through SUPER^
                test := SUPER^.do_something();
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_with_typed_methods() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            METHOD process : ARRAY[0..1] OF INT
                process[0] := 42;
                process[1] := 43;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD process : ARRAY[0..1] OF INT
                // Override with different implementation
                process[0] := 100;
                process[1] := 200;
            END_METHOD
            
            METHOD test : INT
                // Access parent's process method which returns an array
                test := SUPER^.process()[0] + SUPER^.process()[1];
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_with_mixed_access_patterns() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                arr : ARRAY[0..5] OF INT := [1,2,3,4,5,6];
                ptr : REF_TO INT;
            END_VAR
            
            METHOD get_value : INT
                get_value := 42;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                local_idx : INT := 2;
            END_VAR
            
            METHOD test
                // Complex expression with SUPER^, array access, and method call
                local_idx := SUPER^.arr[SUPER^.get_value() MOD 6];
                
                // Using SUPER^ with pointer operations
                SUPER^.ptr := REF(SUPER^.arr[0]);
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_in_multi_level_inheritance() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK grandparent
            VAR
                g_val : INT := 10;
            END_VAR
            
            METHOD gp_method : INT
                gp_method := g_val;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            VAR
                p_val : INT := 20;
            END_VAR
            
            METHOD p_method : INT
                p_method := p_val + SUPER^.gp_method();
            END_METHOD
            
            METHOD gp_method : INT  // Override grandparent's method
                gp_method := p_val * 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                c_val : INT := 30;
            END_VAR
            
            METHOD test : INT
                // Access parent's implementation which itself uses SUPER^
                test := SUPER^.p_method();
            END_METHOD
            
            METHOD p_method : INT  // Override parent's method
                p_method := c_val * 3;
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
#[ignore = "needs #1436 to be merged"]
fn super_with_property_access() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                _prop_val : INT := 10;
            END_VAR
            
            PROPERTY prop : INT
                GET
                    prop := _prop_val;
                END_GET
                SET
                    _prop_val := prop;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                local : INT;
            END_VAR
            
            PROPERTY prop : INT // Override property
                GET
                    prop := _prop_val * 2;
                END_GET
                SET
                    _prop_val := prop / 2;
                END_SET
            END_PROPERTY
            
            METHOD test
                // Get using parent's property getter
                local := SUPER^.prop;
                
                // Set using parent's property setter
                SUPER^.prop := 42;
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_in_variable_initialization() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                value : INT := 10;
            END_VAR
            
            METHOD get_init_value : INT
                get_init_value := 42;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Try to use SUPER in initialization expressions
            VAR
                val1 : INT := SUPER^.value;
                val2 : INT := SUPER^.get_init_value();
            END_VAR
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r#"
    error[E033]: Unresolved constant `val1` variable: `value` is no const reference
       ┌─ <internal>:15:31
       │
    15 │                 val1 : INT := SUPER^.value;
       │                               ^^^^^^^^^^^^ Unresolved constant `val1` variable: `value` is no const reference

    error[E033]: Unresolved constant `val2` variable: Cannot resolve constant: CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "get_init_value",
                },
            ),
            base: Some(
                Super(derefed),
            ),
        },
        parameters: None,
    }
       ┌─ <internal>:16:31
       │
    16 │                 val2 : INT := SUPER^.get_init_value();
       │                               ^^^^^^^^^^^^^^^^^^^^^^^^ Unresolved constant `val2` variable: Cannot resolve constant: CallStatement {
        operator: ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "get_init_value",
                },
            ),
            base: Some(
                Super(derefed),
            ),
        },
        parameters: None,
    }
    "#);
}

#[test]
fn const_super_variable_in_child_variable_initialization() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR CONSTANT
                value : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                val1 : INT := SUPER^.value;
            END_VAR
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_with_interface_implementations() {
    let diagnostics = parse_and_validate_buffered(
        r"
        INTERFACE ICounter
            METHOD increment : INT END_METHOD
            METHOD get_count : INT END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK parent IMPLEMENTS ICounter
            VAR
                count : INT := 0;
            END_VAR
            
            METHOD increment : INT
                count := count + 1;
                increment := count;
            END_METHOD
            
            METHOD get_count : INT
                get_count := count;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD increment : INT // Override interface method
                count := count + 10;
                increment := count;
            END_METHOD
            
            METHOD test : INT
                // Use parent's interface implementation
                SUPER^.increment();
                test := SUPER^.get_count();
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    // XXX: we should probably revisit the illegal access warning, at least for accessing derived members
    assert_snapshot!(diagnostics, @r"
    warning[E049]: Illegal access to private member parent.count
       ┌─ <internal>:24:17
       │
    24 │                 count := count + 10;
       │                 ^^^^^ Illegal access to private member parent.count

    warning[E049]: Illegal access to private member parent.count
       ┌─ <internal>:24:26
       │
    24 │                 count := count + 10;
       │                          ^^^^^ Illegal access to private member parent.count

    warning[E049]: Illegal access to private member parent.count
       ┌─ <internal>:25:30
       │
    25 │                 increment := count;
       │                              ^^^^^ Illegal access to private member parent.count
    ");
}

#[test]
fn super_in_nested_conditionals() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                threshold : INT := 50;
                value : INT := 10;
            END_VAR
            
            METHOD check_value : BOOL
                check_value := value > threshold;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test
                // Use SUPER^ in conditionals
                IF SUPER^.value > 0 THEN
                    IF SUPER^.check_value() THEN
                        SUPER^.value := SUPER^.value + 1;
                    ELSE
                        SUPER^.value := SUPER^.value - 1;
                    END_IF;
                END_IF;
                
                // In CASE statement
                CASE SUPER^.value OF
                    10: SUPER^.threshold := 40;
                    20: SUPER^.threshold := 60;
                    ELSE
                        SUPER^.threshold := 50;
                END_CASE;
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_in_fb_instance_array() {
    let diagnostics = parse_and_validate_buffered(
        r"
        FUNCTION_BLOCK parent
            VAR
                value : INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK array_container
            VAR
                children : ARRAY[0..2] OF child;
            END_VAR
            
            METHOD test
                // Should fail - SUPER is only available inside the POU that extends another
                children[0].SUPER^.value := 20;
                children[1].SUPER^.value := 30;
            END_METHOD
        END_FUNCTION_BLOCK
    ",
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:18:29
       │
    18 │                 children[0].SUPER^.value := 20;
       │                             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E048]: Could not resolve reference to value
       ┌─ <internal>:18:36
       │
    18 │                 children[0].SUPER^.value := 20;
       │                                    ^^^^^ Could not resolve reference to value

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:19:29
       │
    19 │                 children[1].SUPER^.value := 30;
       │                             ^^^^^ `SUPER` can only be used in POUs that extend another POU.

    error[E048]: Could not resolve reference to value
       ┌─ <internal>:19:36
       │
    19 │                 children[1].SUPER^.value := 30;
       │                                    ^^^^^ Could not resolve reference to value
    ");
}

#[test]
fn invalid_super_dereferencing_patterns() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Multiple dereferencing of SUPER (should be invalid)
            SUPER^^.x := 20; 
            
            // Missing dot between derefs
            SUPER^^ := 30;
            
            // Invalid chain with wrong syntax
            SUPER^.SUPER.x := 40;
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E068]: Dereferencing requires a pointer-value.
       ┌─ <internal>:10:13
       │
    10 │             SUPER^^.x := 20; 
       │             ^^^^^^^ Dereferencing requires a pointer-value.

    error[E068]: Dereferencing requires a pointer-value.
       ┌─ <internal>:13:13
       │
    13 │             SUPER^^ := 30;
       │             ^^^^^^^ Dereferencing requires a pointer-value.

    error[E119]: Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU
       ┌─ <internal>:16:13
       │
    16 │             SUPER^.SUPER.x := 40;
       │             ^^^^^^^^^^^^ Chaining multiple `SUPER` accessors is not allowed, use a single `SUPER` to access the parent POU

    error[E119]: `SUPER` can only be used in POUs that extend another POU.
       ┌─ <internal>:16:20
       │
    16 │             SUPER^.SUPER.x := 40;
       │                    ^^^^^ `SUPER` can only be used in POUs that extend another POU.
    ");
}

#[test]
fn super_in_paren_expressions() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                result: INT;
            END_VAR
            
            METHOD test
                // Using SUPER in parentheses
                result := (SUPER^.x + 5) * 2;
                
                // Using SUPER in a nested expression
                result := (SUPER^.x + (SUPER^.x * 2)) / 3;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn invalid_super_dereferencing_patterns_parenthesized() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                result: INT;
            END_VAR
            
            METHOD test
                // Multiple dereferencing of SUPER
                (SUPER^)^.x := 20; // FIXME: this is currently a bug, not just an issue with super. https://github.com/PLC-lang/rusty/issues/1448
                
                (SUPER^)^ := 30;

                // // Valid deref in parentheses
                // result := (SUPER)^.x + 5;
                
                // // Invalid chain with wrong syntax
                // (SUPER^).SUPER.x := 40;
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E068]: Dereferencing requires a pointer-value.
       ┌─ <internal>:15:17
       │
    15 │                 (SUPER^)^.x := 20; // FIXME: this is currently a bug, not just an issue with super. https://github.com/PLC-lang/rusty/issues/1448
       │                 ^^^^^^^^^ Dereferencing requires a pointer-value.

    warning[E049]: Illegal access to private member parent.x
       ┌─ <internal>:15:27
       │
    15 │                 (SUPER^)^.x := 20; // FIXME: this is currently a bug, not just an issue with super. https://github.com/PLC-lang/rusty/issues/1448
       │                           ^ Illegal access to private member parent.x

    error[E068]: Dereferencing requires a pointer-value.
       ┌─ <internal>:17:17
       │
    17 │                 (SUPER^)^ := 30;
       │                 ^^^^^^^^^ Dereferencing requires a pointer-value.
    ");
}

#[test]
fn incorrect_super_usage_with_ref_to_parameters() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD test
                // Call functions with wrong SUPER form
                expect_parent(SUPER);       // Should be SUPER^
                expect_ref_to_parent(SUPER^); // Should be SUPER
            END_METHOD
        END_FUNCTION_BLOCK
        
        FUNCTION expect_parent
        VAR_INPUT
            p: parent;
        END_VAR
        END_FUNCTION
        
        FUNCTION expect_ref_to_parent
        VAR_INPUT
            p: REF_TO parent;
        END_VAR
        END_FUNCTION
        "#,
    );

    // This test should verify that when passing SUPER to functions, it's used correctly:
    // - When function expects parent, we should use SUPER^ (dereferenced)
    // - When function expects REF_TO parent, we should use SUPER (reference form)
    assert_snapshot!(diagnostics, @r"
    error[E065]: The type parent 16 is too small to hold a Pointer
       ┌─ <internal>:11:31
       │
    11 │                 expect_parent(SUPER);       // Should be SUPER^
       │                               ^^^^^ The type parent 16 is too small to hold a Pointer

    error[E065]: The type parent 16 is too small to be stored in a Pointer
       ┌─ <internal>:12:38
       │
    12 │                 expect_ref_to_parent(SUPER^); // Should be SUPER
       │                                      ^^^^^ The type parent 16 is too small to be stored in a Pointer
    ");
}

#[test]
fn super_with_pointer_operations() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                val : INT := 10;
                ptr : REF_TO INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Pointer operations with SUPER^
            SUPER^.ptr := REF(SUPER^.val);
            // Dereferencing pointer from parent
            SUPER^.val := SUPER^.ptr^ + 5;
        END_FUNCTION_BLOCK
        "#,
    );
    assert!(diagnostics.is_empty(), "Expected no diagnostics, but found: {diagnostics:?}");
}

#[test]
fn super_with_invalid_operations() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                result: BOOL;
                p1: REF_TO parent;
                p2: parent;
            END_VAR
            
            // Invalid operations on SUPER
            p1 := SUPER + SUPER;    // Can't add references
            result := SUPER = SUPER; // Ref comparison (might be allowed but semantically wrong)
            p2 := SUPER;            // Type mismatch (expecting parent, got REF_TO parent)
            p1 := SUPER^.x;         // Type mismatch (expecting REF_TO parent, got INT)
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E065]: The type parent 16 is too small to hold a Pointer
       ┌─ <internal>:18:13
       │
    18 │             p2 := SUPER;            // Type mismatch (expecting parent, got REF_TO parent)
       │             ^^^^^^^^^^^ The type parent 16 is too small to hold a Pointer

    error[E065]: The type INT 16 is too small to be stored in a Pointer
       ┌─ <internal>:19:13
       │
    19 │             p1 := SUPER^.x;         // Type mismatch (expecting REF_TO parent, got INT)
       │             ^^^^^^^^^^^^^^ The type INT 16 is too small to be stored in a Pointer

    error[E037]: Invalid assignment: cannot assign 'INT' to 'REF_TO parent'
       ┌─ <internal>:19:13
       │
    19 │             p1 := SUPER^.x;         // Type mismatch (expecting REF_TO parent, got INT)
       │             ^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'INT' to 'REF_TO parent'
    ");
}

#[test]
fn super_dereferencing_with_method_calls() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            METHOD get_value : INT
                get_value := 42;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            x: INT;
            p1: REF_TO parent;
            p2: parent;
        END_VAR
            METHOD test
                // These should be valid:
                x := SUPER^.get_value();
                
                // These should be invalid:
                x := SUPER.get_value();    // Trying to call method on pointer
                p2 := SUPER^.get_value;    // Method call missing ()
                // ^ this validation is currently missing, tracked in https://github.com/PLC-lang/rusty/issues/1449
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` must be dereferenced to access its members.
       ┌─ <internal>:19:28
       │
    19 │                 x := SUPER.get_value();    // Trying to call method on pointer
       │                            ^^^^^^^^^ `SUPER` must be dereferenced to access its members.
    ");
}

#[test]
fn super_without_deref_accessing_members() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                x: INT := 10;
                ptr: REF_TO INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Trying to access member without dereferencing SUPER
            SUPER.x := 20; // Should be SUPER^.x
            
            // Trying to access pointer member without dereferencing SUPER
            SUPER.ptr^ := 30; // Should be SUPER^.ptr^
            
            // Double dereferencing
            SUPER^.ptr^^ := 40; // Error - can't double-deref
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` must be dereferenced to access its members.
       ┌─ <internal>:11:19
       │
    11 │             SUPER.x := 20; // Should be SUPER^.x
       │                   ^ `SUPER` must be dereferenced to access its members.

    error[E119]: `SUPER` must be dereferenced to access its members.
       ┌─ <internal>:14:19
       │
    14 │             SUPER.ptr^ := 30; // Should be SUPER^.ptr^
       │                   ^^^ `SUPER` must be dereferenced to access its members.

    error[E068]: Dereferencing requires a pointer-value.
       ┌─ <internal>:17:13
       │
    17 │             SUPER^.ptr^^ := 40; // Error - can't double-deref
       │             ^^^^^^^^^^^^ Dereferencing requires a pointer-value.
    ");
}

#[test]
fn super_with_property_access_errors() {
    let diagnostics = parse_and_validate_buffered(
        r#"
        FUNCTION_BLOCK parent
            VAR
                _value: INT;
            END_VAR
            
            PROPERTY prop : INT
                GET
                    prop := _value;
                END_GET
                SET
                    _value := prop;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                x: INT;
            END_VAR

            // Invalid property access
            SUPER.prop := 10;    // Should be SUPER^.prop
            x := SUPER.prop;     // Should be SUPER^.prop
            
            // Invalid function-style property access
            SUPER^.prop();
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(diagnostics, @r"
    error[E119]: `SUPER` must be dereferenced to access its members.
       ┌─ <internal>:23:19
       │
    23 │             SUPER.prop := 10;    // Should be SUPER^.prop
       │                   ^^^^ `SUPER` must be dereferenced to access its members.

    error[E119]: `SUPER` must be dereferenced to access its members.
       ┌─ <internal>:24:24
       │
    24 │             x := SUPER.prop;     // Should be SUPER^.prop
       │                        ^^^^ `SUPER` must be dereferenced to access its members.

    error[E007]: Properties cannot be called like functions. Remove `()`
       ┌─ <internal>:27:13
       │
    27 │             SUPER^.prop();
       │             ^^^^^^^^^^^ Properties cannot be called like functions. Remove `()`
    ");
}

#[test]
fn pointer_arithmetic_with_super() {
    let diagnostics = parse_and_validate_buffered(
        r#"
    FUNCTION_BLOCK parent
    VAR
        x : LINT := 10;
        y : LINT := 20;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK child EXTENDS parent
    VAR
        a : INT;
    END_VAR
        // Pointer arithmetic with SUPER
        a := (SUPER + 1)^ + 5;
    END_FUNCTION_BLOCK
    "#,
    );
    assert_snapshot!(diagnostics, @r#""#);
}
