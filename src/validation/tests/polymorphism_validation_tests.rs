mod pou {
    use test_utils::parse_and_validate_buffered;

    // -------------------------------------------------------------------
    // Assignment compatibility: POINTER TO + ADR
    // -------------------------------------------------------------------

    #[test]
    fn assign_same_type_to_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    ptr      : POINTER TO FbA;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
          ┌─ <internal>:8:32
          │
        8 │                     ptr      : POINTER TO FbA;
          │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn assign_direct_child_to_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbB;
                    ptr      : POINTER TO FbA;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:32
           │
        11 │                     ptr      : POINTER TO FbA;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn assign_transitive_child_to_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceB : FbB;
                    instanceC : FbC;
                    ptr      : POINTER TO FbA;
                END_VAR
                ptr := ADR(instanceB);
                ptr := ADR(instanceC);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:15:32
           │
        15 │                     ptr      : POINTER TO FbA;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn assign_unrelated_type_to_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbX;
                    ptr      : POINTER TO FbA;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:32
           │
        11 │                     ptr      : POINTER TO FbA;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
           ┌─ <internal>:13:17
           │
        13 │                 ptr := ADR(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn assign_parent_to_child_pointer_downcast() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    ptr      : POINTER TO FbB;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:32
           │
        11 │                     ptr      : POINTER TO FbB;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
           ┌─ <internal>:13:17
           │
        13 │                 ptr := ADR(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn assign_sibling_to_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbC;
                    ptr      : POINTER TO FbB;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:14:32
           │
        14 │                     ptr      : POINTER TO FbB;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbB' and 'FbC' are not related and cannot be used polymorphically
           ┌─ <internal>:16:17
           │
        16 │                 ptr := ADR(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Invalid assignment: 'FbB' and 'FbC' are not related and cannot be used polymorphically
        ");
    }

    // -------------------------------------------------------------------
    // Pointer-to-pointer assignments
    // -------------------------------------------------------------------

    #[test]
    fn assign_child_pointer_to_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptrA : POINTER TO FbA;
                    ptrB : POINTER TO FbB;
                END_VAR
                ptrA := ptrB;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:28
           │
        10 │                     ptrA : POINTER TO FbA;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:28
           │
        11 │                     ptrB : POINTER TO FbB;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn assign_base_pointer_to_child_pointer_downcast() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptrA : POINTER TO FbA;
                    ptrB : POINTER TO FbB;
                END_VAR
                ptrB := ptrA;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:28
           │
        10 │                     ptrA : POINTER TO FbA;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:28
           │
        11 │                     ptrB : POINTER TO FbB;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
           ┌─ <internal>:13:17
           │
        13 │                 ptrB := ptrA;
           │                 ^^^^^^^^^^^^ Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn assign_unrelated_pointer_to_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptrA : POINTER TO FbA;
                    ptrX : POINTER TO FbX;
                END_VAR
                ptrA := ptrX;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:28
           │
        10 │                     ptrA : POINTER TO FbA;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:28
           │
        11 │                     ptrX : POINTER TO FbX;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
           ┌─ <internal>:13:17
           │
        13 │                 ptrA := ptrX;
           │                 ^^^^^^^^^^^^ Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
        ");
    }

    // -------------------------------------------------------------------
    // REF_TO variants
    // -------------------------------------------------------------------

    #[test]
    fn assign_child_to_base_ref_to() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbB;
                    ref      : REF_TO FbA;
                END_VAR
                ref := REF(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        // NOTE: Not a bug, REF(...) is type-safe and expects the exact type here. In terms of polymorphism
        // this would be valid per-se but due to REF(...) is not. The user needs to use ADR(...) here instead
        // as is intended in the norm and codesys (though, don't quote me on the latter).
        insta::assert_snapshot!(diagnostics, @r"
        warning[E090]: Pointers REF_TO FbA and FbB have different types
           ┌─ <internal>:13:17
           │
        13 │                 ref := REF(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Pointers REF_TO FbA and FbB have different types
        ");
    }

    #[test]
    fn assign_unrelated_to_ref_to() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbX;
                    ref      : REF_TO FbA;
                END_VAR
                ref := REF(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        // TODO: This should return an error, similar to ADR(...) cases
        insta::assert_snapshot!(diagnostics, @r"
        warning[E090]: Pointers REF_TO FbA and FbX have different types
           ┌─ <internal>:13:17
           │
        13 │                 ref := REF(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Pointers REF_TO FbA and FbX have different types
        ");
    }

    // -------------------------------------------------------------------
    // REFERENCE TO variants
    // -------------------------------------------------------------------

    #[test]
    fn assign_child_to_base_reference_to() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instanceA : FbA;
                    instanceB : FbB;
                    ref       : REFERENCE TO FbA;
                END_VAR
                ref REF= instanceB;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        // Valid: child→parent upcast. No errors.
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_unrelated_to_reference_to() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbX;
                    ref      : REFERENCE TO FbA;
                END_VAR
                ref REF= instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        error[E037]: Invalid assignment: cannot assign 'FbX' to 'FbA'
           ┌─ <internal>:13:17
           │
        13 │                 ref REF= instance;
           │                 ^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'FbX' to 'FbA'
        ");
    }

    // -------------------------------------------------------------------
    // Classes (same rules as function blocks)
    // -------------------------------------------------------------------

    #[test]
    fn assign_child_class_to_base_pointer() {
        let source = r#"
            CLASS ClA
            END_CLASS

            CLASS ClB EXTENDS ClA
            END_CLASS

            FUNCTION main
                VAR
                    instance : ClB;
                    ptr      : POINTER TO ClA;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:32
           │
        11 │                     ptr      : POINTER TO ClA;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn assign_unrelated_class_to_pointer() {
        let source = r#"
            CLASS ClA
            END_CLASS

            CLASS ClX
            END_CLASS

            FUNCTION main
                VAR
                    instance : ClX;
                    ptr      : POINTER TO ClA;
                END_VAR
                ptr := ADR(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:11:32
           │
        11 │                     ptr      : POINTER TO ClA;
           │                                ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'ClA' and 'ClX' are not related and cannot be used polymorphically
           ┌─ <internal>:13:17
           │
        13 │                 ptr := ADR(instance);
           │                 ^^^^^^^^^^^^^^^^^^^^ Invalid assignment: 'ClA' and 'ClX' are not related and cannot be used polymorphically
        ");
    }

    // -------------------------------------------------------------------
    // Method existence on static type
    // -------------------------------------------------------------------

    #[test]
    fn call_method_defined_on_static_type() {
        let source = r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptr : POINTER TO FbA;
                END_VAR
                ptr^.foo();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
          ┌─ <internal>:9:27
          │
        9 │                     ptr : POINTER TO FbA;
          │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn call_overridden_method_through_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptr : POINTER TO FbA;
                END_VAR
                ptr^.foo();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:14:27
           │
        14 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn call_child_only_method_through_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptr : POINTER TO FbA;
                END_VAR
                ptr^.bar();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:14:27
           │
        14 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E048]: Could not resolve reference to bar
           ┌─ <internal>:16:22
           │
        16 │                 ptr^.bar();
           │                      ^^^ Could not resolve reference to bar
        ");
    }

    #[test]
    fn call_grandchild_method_through_base_pointer() {
        let source = r#"
            FUNCTION_BLOCK FbA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC EXTENDS FbB
                METHOD baz
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ptr : POINTER TO FbA;
                END_VAR
                ptr^.baz();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:17:27
           │
        17 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E048]: Could not resolve reference to baz
           ┌─ <internal>:19:22
           │
        19 │                 ptr^.baz();
           │                      ^^^ Could not resolve reference to baz
        ");
    }

    // -------------------------------------------------------------------
    // Call arguments: POINTER TO as function/method parameters
    // -------------------------------------------------------------------

    #[test]
    fn call_arg_child_pointer_to_base_param() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ptr : POINTER TO FbA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    ptrB : POINTER TO FbB;
                END_VAR
                consumer(ptrB);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:27
           │
        10 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:16:28
           │
        16 │                     ptrB : POINTER TO FbB;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn call_arg_unrelated_pointer_to_param() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ptr : POINTER TO FbA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    ptrX : POINTER TO FbX;
                END_VAR
                consumer(ptrX);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:27
           │
        10 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:16:28
           │
        16 │                     ptrX : POINTER TO FbX;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
           ┌─ <internal>:18:26
           │
        18 │                 consumer(ptrX);
           │                          ^^^^ Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn call_arg_downcast_pointer_to_param() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ptr : POINTER TO FbB;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    ptrA : POINTER TO FbA;
                END_VAR
                consumer(ptrA);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:27
           │
        10 │                     ptr : POINTER TO FbB;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:16:28
           │
        16 │                     ptrA : POINTER TO FbA;
           │                            ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
           ┌─ <internal>:18:26
           │
        18 │                 consumer(ptrA);
           │                          ^^^^ Invalid assignment: 'FbB' and 'FbA' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn call_arg_adr_child_to_base_param() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ptr : POINTER TO FbA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instanceB : FbB;
                END_VAR
                consumer(ADR(instanceB));
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:27
           │
        10 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead
        ");
    }

    #[test]
    fn call_arg_adr_unrelated_to_param() {
        let source = r#"
            FUNCTION_BLOCK FbA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    ptr : POINTER TO FbA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instanceX : FbX;
                END_VAR
                consumer(ADR(instanceX));
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        warning[E015]: `POINTER TO` is type-unsafe, consider using `REF_TO` instead
           ┌─ <internal>:10:27
           │
        10 │                     ptr : POINTER TO FbA;
           │                           ^^^^^^^ `POINTER TO` is type-unsafe, consider using `REF_TO` instead

        error[E125]: Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
           ┌─ <internal>:18:26
           │
        18 │                 consumer(ADR(instanceX));
           │                          ^^^^^^^^^^^^^^ Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
        ");
    }
}
