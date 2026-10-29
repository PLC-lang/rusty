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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"");
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"");
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
        insta::assert_snapshot!(diagnostics, @"
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
        insta::assert_snapshot!(diagnostics, @"
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

        insta::assert_snapshot!(diagnostics, @"");
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
        insta::assert_snapshot!(diagnostics, @"
        error[E125]: Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
           ┌─ <internal>:18:26
           │
        18 │                 consumer(ADR(instanceX));
           │                          ^^^^^^^^^^^^^^ Invalid assignment: 'FbA' and 'FbX' are not related and cannot be used polymorphically
        ");
    }
}

mod interface {
    use test_utils::parse_and_validate_buffered;

    #[test]
    fn assign_implementor_to_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    ref      : IA;
                END_VAR
                ref := instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_child_implementor_to_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbB;
                    ref      : IA;
                END_VAR
                ref := instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_non_implementor_to_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbX;
                    ref      : IA;
                END_VAR
                ref := instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbX' does not implement interface 'IA'
           ┌─ <internal>:15:17
           │
        15 │                 ref := instance;
           │                 ^^^^^^^^^^^^^^^ Invalid assignment: 'FbX' does not implement interface 'IA'
        ");
    }

    #[test]
    fn assign_unrelated_implementor_to_wrong_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    ref      : IB;
                END_VAR
                ref := instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbA' does not implement interface 'IB'
           ┌─ <internal>:22:17
           │
        22 │                 ref := instance;
           │                 ^^^^^^^^^^^^^^^ Invalid assignment: 'FbA' does not implement interface 'IB'
        ");
    }

    #[test]
    fn assign_pou_implementing_child_interface_to_parent_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    refIA    : IA;
                    refIB    : IB;
                END_VAR
                refIA := instance;
                refIB := instance;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_same_interface_type() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    ref1 : IA;
                    ref2 : IA;
                END_VAR
                ref1 := ref2;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_child_interface_to_parent_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                    refIB : IB;
                END_VAR
                refIA := refIB;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_grandchild_interface_to_grandparent_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            INTERFACE IC EXTENDS IB
                METHOD baz
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IC
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD

                METHOD baz
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                    refIC : IC;
                END_VAR
                refIA := refIC;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn assign_parent_interface_to_child_interface_ref_downcast() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                    refIB : IB;
                END_VAR
                refIB := refIA;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'IB' and 'IA' are not related and cannot be used polymorphically
           ┌─ <internal>:25:17
           │
        25 │                 refIB := refIA;
           │                 ^^^^^^^^^^^^^^ Invalid assignment: 'IB' and 'IA' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn assign_unrelated_interface_to_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB IMPLEMENTS IB
                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                    refIB : IB;
                END_VAR
                refIA := refIB;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'IA' and 'IB' are not related and cannot be used polymorphically
           ┌─ <internal>:27:17
           │
        27 │                 refIA := refIB;
           │                 ^^^^^^^^^^^^^^ Invalid assignment: 'IA' and 'IB' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn assign_sibling_interface_to_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            INTERFACE IC EXTENDS IA
                METHOD baz
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbB IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbC IMPLEMENTS IC
                METHOD foo
                END_METHOD

                METHOD baz
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIB : IB;
                    refIC : IC;
                END_VAR
                refIB := refIC;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'IB' and 'IC' are not related and cannot be used polymorphically
           ┌─ <internal>:38:17
           │
        38 │                 refIB := refIC;
           │                 ^^^^^^^^^^^^^^ Invalid assignment: 'IB' and 'IC' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn call_arg_implementor_to_interface_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbA;
                END_VAR
                consumer(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_arg_named_implementor_to_interface_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbA;
                END_VAR
                consumer(in1 := instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_arg_non_implementor_to_interface_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbX;
                END_VAR
                consumer(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbX' does not implement interface 'IA'
           ┌─ <internal>:20:26
           │
        20 │                 consumer(instance);
           │                          ^^^^^^^^ Invalid assignment: 'FbX' does not implement interface 'IA'
        ");
    }

    #[test]
    fn call_arg_named_non_implementor_to_interface_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbX;
                END_VAR
                consumer(in1 := instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbX' does not implement interface 'IA'
           ┌─ <internal>:20:33
           │
        20 │                 consumer(in1 := instance);
           │                                 ^^^^^^^^ Invalid assignment: 'FbX' does not implement interface 'IA'
        ");
    }

    #[test]
    fn call_arg_child_implementor_to_interface_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbB EXTENDS FbA
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbB;
                END_VAR
                consumer(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_arg_wrong_interface_to_param() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IB;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbA;
                END_VAR
                consumer(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbA' does not implement interface 'IB'
           ┌─ <internal>:27:26
           │
        27 │                 consumer(instance);
           │                          ^^^^^^^^ Invalid assignment: 'FbA' does not implement interface 'IB'
        ");
    }

    #[test]
    fn call_arg_interface_to_interface_param_upcast() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    refIB : IB;
                END_VAR
                consumer(refIB);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_arg_interface_to_interface_param_downcast() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IB;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                consumer(refIA);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'IB' and 'IA' are not related and cannot be used polymorphically
           ┌─ <internal>:30:26
           │
        30 │                 consumer(refIA);
           │                          ^^^^^ Invalid assignment: 'IB' and 'IA' are not related and cannot be used polymorphically
        ");
    }

    #[test]
    fn call_arg_multiple_interfaces_mixed_validity() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                    in2 : IB;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    instance : FbA;
                END_VAR
                consumer(in1 := instance, in2 := instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E126]: Invalid assignment: 'FbA' does not implement interface 'IB'
           ┌─ <internal>:28:50
           │
        28 │                 consumer(in1 := instance, in2 := instance);
           │                                                  ^^^^^^^^ Invalid assignment: 'FbA' does not implement interface 'IB'
        ");
    }

    mod by_ref {
        use test_utils::parse_and_validate_buffered;

        const DECLARATIONS: &str = r#"
            INTERFACE IA
                METHOD foo END_METHOD
            END_INTERFACE
            INTERFACE IB EXTENDS IA
                METHOD bar END_METHOD
            END_INTERFACE
            INTERFACE IC
                METHOD baz END_METHOD
            END_INTERFACE
            INTERFACE ID EXTENDS IB, IC
            END_INTERFACE
            INTERFACE IM
                METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbA2 EXTENDS FbA
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbB IMPLEMENTS IB
                METHOD foo END_METHOD
                METHOD bar END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbAC IMPLEMENTS IA, IC
                METHOD foo END_METHOD
                METHOD baz END_METHOD
            END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbX
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbInOut VAR_IN_OUT x : IA; END_VAR END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbOut VAR_OUTPUT x : IA; END_VAR END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbOutB VAR_OUTPUT x : IB; END_VAR END_FUNCTION_BLOCK
            FUNCTION_BLOCK FbMethod
                METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
            FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
            FUNCTION f_ref : DINT VAR_INPUT x : REFERENCE TO IA; END_VAR END_FUNCTION
            FUNCTION f_ref_input : DINT VAR_INPUT {ref} x : IA; END_VAR END_FUNCTION
            FUNCTION f_val : DINT VAR_INPUT x : IA; END_VAR END_FUNCTION
            FUNCTION f_dint : DINT VAR_INPUT x : DINT; END_VAR END_FUNCTION
            FUNCTION f_inout_fb : DINT VAR_IN_OUT x : FbA; END_VAR END_FUNCTION
            FUNCTION f_forward : DINT VAR_IN_OUT x : IA; END_VAR f_inout(x); END_FUNCTION

            TYPE Holder : STRUCT r : IA; END_STRUCT END_TYPE
            VAR_GLOBAL CONSTANT cia : IA; END_VAR
        "#;

        /// Validates `body` in a function `main` that declares a variable of each type above.
        fn validate(body: &str) -> String {
            let source = format!(
                "{DECLARATIONS}
            FUNCTION main
                VAR
                    a : FbA; a2 : FbA2; b : FbB; ac : FbAC; xx : FbX; d : DINT;
                    ia : IA; ib : IB; ic : IC; id : ID; im : IM; p : REF_TO IA; q : POINTER TO IA;
                    arr : ARRAY[1..2] OF IA; h : Holder; rr : REFERENCE TO IA; rp : REFERENCE TO REF_TO IA;
                    fbInOut : FbInOut; fbOut : FbOut; fbOutB : FbOutB; fbMethod : FbMethod;
                END_VAR
                {body}
            END_FUNCTION
        "
            );
            parse_and_validate_buffered(&source)
        }

        // -------------------------------------------------------------------
        // Implementing instance into each kind of by-reference parameter
        // -------------------------------------------------------------------
        #[test]
        fn call_arg_by_ref_implementor_to_in_out() {
            insta::assert_snapshot!(validate("f_inout(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(a);
               │                         ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_named_in_out() {
            insta::assert_snapshot!(validate("f_inout(x := a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:30
               │
            59 │                 f_inout(x := a);
               │                              ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_output() {
            insta::assert_snapshot!(validate("f_out(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'f_out'
               ┌─ <internal>:59:23
               │
            59 │                 f_out(a);
               │                       ^ Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'f_out'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:41:46
               │
            41 │             FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_named_output() {
            insta::assert_snapshot!(validate("f_out(x => a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'f_out'
               ┌─ <internal>:59:28
               │
            59 │                 f_out(x => a);
               │                            ^ Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'f_out'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:41:46
               │
            41 │             FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_reference_to() {
            insta::assert_snapshot!(validate("f_ref(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_ref'
               ┌─ <internal>:59:23
               │
            59 │                 f_ref(a);
               │                       ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_ref'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:42:45
               │
            42 │             FUNCTION f_ref : DINT VAR_INPUT x : REFERENCE TO IA; END_VAR END_FUNCTION
               │                                             ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_ref_input() {
            insta::assert_snapshot!(validate("f_ref_input(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_ref_input'
               ┌─ <internal>:59:29
               │
            59 │                 f_ref_input(a);
               │                             ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_ref_input'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:43:57
               │
            43 │             FUNCTION f_ref_input : DINT VAR_INPUT {ref} x : IA; END_VAR END_FUNCTION
               │                                                         ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_method_in_out() {
            insta::assert_snapshot!(validate("fbMethod.m(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbMethod.m'
               ┌─ <internal>:59:28
               │
            59 │                 fbMethod.m(a);
               │                            ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbMethod.m'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:37:37
               │
            37 │                 METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
               │                                     ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_function_block_in_out() {
            insta::assert_snapshot!(validate("fbInOut(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbInOut'
               ┌─ <internal>:59:25
               │
            59 │                 fbInOut(a);
               │                         ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbInOut'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:33:47
               │
            33 │             FUNCTION_BLOCK FbInOut VAR_IN_OUT x : IA; END_VAR END_FUNCTION_BLOCK
               │                                               ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_named_function_block_in_out() {
            insta::assert_snapshot!(validate("fbInOut(x := a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbInOut'
               ┌─ <internal>:59:30
               │
            59 │                 fbInOut(x := a);
               │                              ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'fbInOut'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:33:47
               │
            33 │             FUNCTION_BLOCK FbInOut VAR_IN_OUT x : IA; END_VAR END_FUNCTION_BLOCK
               │                                               ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_to_in_out_of_interface_method() {
            insta::assert_snapshot!(validate("im.m(a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'im.m'
               ┌─ <internal>:59:22
               │
            59 │                 im.m(a);
               │                      ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'im.m'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:14:37
               │
            14 │                 METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
               │                                     ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_in_nested_call() {
            insta::assert_snapshot!(validate("f_dint(f_inout(a));"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:32
               │
            59 │                 f_dint(f_inout(a));
               │                                ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        // -------------------------------------------------------------------
        // Inheritance on the instance side
        // -------------------------------------------------------------------
        #[test]
        fn call_arg_by_ref_derived_implementor() {
            insta::assert_snapshot!(validate("f_inout(a2);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA2'; assign 'a2' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(a2);
               │                         ^^ Type mismatch: expected type 'IA' but received 'FbA2'; assign 'a2' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_of_child_interface() {
            insta::assert_snapshot!(validate("f_inout(b);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbB'; assign 'b' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(b);
               │                         ^ Type mismatch: expected type 'IA' but received 'FbB'; assign 'b' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_named_implementor_of_child_interface() {
            insta::assert_snapshot!(validate("f_inout(x := b);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbB'; assign 'b' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:30
               │
            59 │                 f_inout(x := b);
               │                              ^ Type mismatch: expected type 'IA' but received 'FbB'; assign 'b' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_implementor_of_multiple_interfaces() {
            insta::assert_snapshot!(validate("f_inout(ac);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbAC'; assign 'ac' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(ac);
               │                         ^^ Type mismatch: expected type 'IA' but received 'FbAC'; assign 'ac' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_non_implementor() {
            insta::assert_snapshot!(validate("f_inout(xx);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbX'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(xx);
               │                         ^^ Type mismatch: expected type 'IA' but received 'FbX'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_non_implementor_to_named_output() {
            insta::assert_snapshot!(validate("f_out(x => xx);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbX'; pass a variable of type 'IA' to 'f_out'
               ┌─ <internal>:59:28
               │
            59 │                 f_out(x => xx);
               │                            ^^ Type mismatch: expected type 'IA' but received 'FbX'; pass a variable of type 'IA' to 'f_out'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:41:46
               │
            41 │             FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        // -------------------------------------------------------------------
        // Interface hierarchy on the argument side
        // -------------------------------------------------------------------
        #[test]
        fn call_arg_by_ref_exact_interface_is_valid() {
            let body = "
                f_inout(ia); f_inout(x := ia); f_out(ia); f_out(x => ia); f_out(x => );
                f_ref(ia); f_ref_input(ia); f_inout(arr[1]); f_out(h.r); f_forward(ia);
                fbInOut(x := ia); fbOut(x => ia); fbOut(x => ); fbMethod.m(ia); im.m(ia);
                f_val(a); f_val(ib);
            ";
            insta::assert_snapshot!(validate(body), @"");
        }

        #[test]
        fn call_arg_by_ref_child_interface() {
            insta::assert_snapshot!(validate("f_inout(ib);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(ib);
               │                         ^^ Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_unrelated_interface() {
            insta::assert_snapshot!(validate("f_inout(ic);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IC'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(ic);
               │                         ^^ Type mismatch: expected type 'IA' but received 'IC'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_diamond_interface() {
            insta::assert_snapshot!(validate("f_inout(id);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'ID'; assign 'id' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(id);
               │                         ^^ Type mismatch: expected type 'IA' but received 'ID'; assign 'id' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_child_interface_to_reference_to() {
            insta::assert_snapshot!(validate("f_ref(ib);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'f_ref'
               ┌─ <internal>:59:23
               │
            59 │                 f_ref(ib);
               │                       ^^ Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'f_ref'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:42:45
               │
            42 │             FUNCTION f_ref : DINT VAR_INPUT x : REFERENCE TO IA; END_VAR END_FUNCTION
               │                                             ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_child_interface_to_in_out_of_interface_method() {
            insta::assert_snapshot!(validate("im.m(ib);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'im.m'
               ┌─ <internal>:59:22
               │
            59 │                 im.m(ib);
               │                      ^^ Type mismatch: expected type 'IA' but received 'IB'; assign 'ib' to a variable of type 'IA' before passing it to 'im.m'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:14:37
               │
            14 │                 METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
               │                                     ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_output_into_child_interface() {
            insta::assert_snapshot!(validate("f_out(x => ib);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; pass a variable of type 'IA' to 'f_out'
               ┌─ <internal>:59:28
               │
            59 │                 f_out(x => ib);
               │                            ^^ Type mismatch: expected type 'IA' but received 'IB'; pass a variable of type 'IA' to 'f_out'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:41:46
               │
            41 │             FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_function_block_output_into_child_interface() {
            insta::assert_snapshot!(validate("fbOut(x => ib);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; pass a variable of type 'IA' to 'fbOut'
               ┌─ <internal>:59:28
               │
            59 │                 fbOut(x => ib);
               │                            ^^ Type mismatch: expected type 'IA' but received 'IB'; pass a variable of type 'IA' to 'fbOut'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:34:45
               │
            34 │             FUNCTION_BLOCK FbOut VAR_OUTPUT x : IA; END_VAR END_FUNCTION_BLOCK
               │                                             ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_function_block_output_into_instance() {
            insta::assert_snapshot!(validate("fbOut(x => a);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'fbOut'
               ┌─ <internal>:59:28
               │
            59 │                 fbOut(x => a);
               │                            ^ Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'fbOut'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:34:45
               │
            34 │             FUNCTION_BLOCK FbOut VAR_OUTPUT x : IA; END_VAR END_FUNCTION_BLOCK
               │                                             ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_function_block_output_into_parent_interface() {
            insta::assert_snapshot!(validate("fbOutB(x => ia);"), @"
            error[E126]: Type mismatch: expected type 'IB' but received 'IA'; pass a variable of type 'IB' to 'fbOutB'
               ┌─ <internal>:59:29
               │
            59 │                 fbOutB(x => ia);
               │                             ^^ Type mismatch: expected type 'IB' but received 'IA'; pass a variable of type 'IB' to 'fbOutB'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IB'
               ┌─ <internal>:35:46
               │
            35 │             FUNCTION_BLOCK FbOutB VAR_OUTPUT x : IB; END_VAR END_FUNCTION_BLOCK
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IB'
            ");
        }

        // -------------------------------------------------------------------
        // Non-POU arguments and the reverse direction
        // -------------------------------------------------------------------
        #[test]
        fn call_arg_by_ref_literal() {
            insta::assert_snapshot!(validate("f_inout(0);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'DINT'
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(0);
               │                         ^ Type mismatch: expected type 'IA' but received 'DINT'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_argument_names() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo END_METHOD
                END_FUNCTION_BLOCK
                TYPE Holder : STRUCT a : FbA; END_STRUCT END_TYPE
                FUNCTION getA : FbA END_FUNCTION
                FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION

                FUNCTION main
                    VAR h : Holder; arr : ARRAY[1..2] OF FbA; p : REF_TO FbA; END_VAR
                    f_inout(h.a);
                    f_inout(arr[1]);
                    f_inout(p^);
                    f_inout(getA());
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'h.a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:14:29
               │
            14 │                     f_inout(h.a);
               │                             ^^^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'h.a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:52
               │
            10 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'arr[1]' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:15:29
               │
            15 │                     f_inout(arr[1]);
               │                             ^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'arr[1]' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:52
               │
            10 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'p^' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:16:29
               │
            16 │                     f_inout(p^);
               │                             ^^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'p^' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:52
               │
            10 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign the argument to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:17:29
               │
            17 │                     f_inout(getA());
               │                             ^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; assign the argument to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:52
               │
            10 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_this_and_super_names() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo END_METHOD
                END_FUNCTION_BLOCK
                FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION

                FUNCTION_BLOCK Base
                    VAR a : FbA; END_VAR
                    METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
                END_FUNCTION_BLOCK
                FUNCTION_BLOCK Derived EXTENDS Base
                    METHOD dm
                        f_inout(THIS^.a);
                        f_inout(SUPER^.a);
                        THIS^.m(a);
                        SUPER^.m(a);
                    END_METHOD
                END_FUNCTION_BLOCK
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'THIS^.a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:16:33
               │
            16 │                         f_inout(THIS^.a);
               │                                 ^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'THIS^.a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
              ┌─ <internal>:8:52
              │
            8 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
              │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'SUPER^.a' to a variable of type 'IA' before passing it to 'f_inout'
               ┌─ <internal>:17:33
               │
            17 │                         f_inout(SUPER^.a);
               │                                 ^^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'SUPER^.a' to a variable of type 'IA' before passing it to 'f_inout'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
              ┌─ <internal>:8:52
              │
            8 │                 FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
              │                                                    ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'THIS^.m'
               ┌─ <internal>:18:33
               │
            18 │                         THIS^.m(a);
               │                                 ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'THIS^.m'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:12:41
               │
            12 │                     METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
               │                                         ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'SUPER^.m'
               ┌─ <internal>:19:34
               │
            19 │                         SUPER^.m(a);
               │                                  ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'SUPER^.m'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:12:41
               │
            12 │                     METHOD m VAR_IN_OUT x : IA; END_VAR END_METHOD
               │                                         ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn ref_assign_by_ref_left_side_names() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo END_METHOD
                END_FUNCTION_BLOCK
                TYPE Holder : STRUCT r : REF_TO IA; END_STRUCT END_TYPE

                FUNCTION main
                    VAR a : FbA; h : Holder; arrp : ARRAY[0..1] OF REF_TO IA; END_VAR
                    h.r REF= a;
                    arrp[1] REF= a;
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; bind 'h.r' to a variable of type 'IA'
               ┌─ <internal>:12:21
               │
            12 │                     h.r REF= a;
               │                     ^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; bind 'h.r' to a variable of type 'IA'

            note[E159]: 'h.r' is a reference and needs a variable of type 'IA'
              ┌─ <internal>:8:38
              │
            8 │                 TYPE Holder : STRUCT r : REF_TO IA; END_STRUCT END_TYPE
              │                                      ^ 'h.r' is a reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; bind 'arrp[1]' to a variable of type 'IA'
               ┌─ <internal>:13:21
               │
            13 │                     arrp[1] REF= a;
               │                     ^^^^^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; bind 'arrp[1]' to a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_constant_interface() {
            insta::assert_snapshot!(validate("f_inout(cia);"), @"
            error[E031]: Parameter x is passed by reference and needs a variable with write access, but 'cia' is constant
               ┌─ <internal>:59:25
               │
            59 │                 f_inout(cia);
               │                         ^^^ Parameter x is passed by reference and needs a variable with write access, but 'cia' is constant
            ");
        }

        #[test]
        fn call_arg_by_ref_interface_to_function_block_in_out() {
            insta::assert_snapshot!(validate("f_inout_fb(ia);"), @"
            error[E126]: Type mismatch: expected type 'FbA' but received 'IA'
               ┌─ <internal>:59:28
               │
            59 │                 f_inout_fb(ia);
               │                            ^^ Type mismatch: expected type 'FbA' but received 'IA'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'FbA'
               ┌─ <internal>:46:51
               │
            46 │             FUNCTION f_inout_fb : DINT VAR_IN_OUT x : FbA; END_VAR END_FUNCTION
               │                                                   ^ Parameter 'x' is passed by reference and needs a variable of type 'FbA'
            ");
        }

        #[test]
        fn call_arg_by_ref_dint_to_named_output() {
            insta::assert_snapshot!(validate("f_out(x => d);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'DINT'; pass a variable of type 'IA' to 'f_out'
               ┌─ <internal>:59:28
               │
            59 │                 f_out(x => d);
               │                            ^ Type mismatch: expected type 'IA' but received 'DINT'; pass a variable of type 'IA' to 'f_out'

            note[E159]: Parameter 'x' is an output and needs a variable of type 'IA'
               ┌─ <internal>:41:46
               │
            41 │             FUNCTION f_out : DINT VAR_OUTPUT x : IA; END_VAR END_FUNCTION
               │                                              ^ Parameter 'x' is an output and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_dint_to_named_in_out() {
            insta::assert_snapshot!(validate("f_inout(x := d);"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'DINT'
               ┌─ <internal>:59:30
               │
            59 │                 f_inout(x := d);
               │                              ^ Type mismatch: expected type 'IA' but received 'DINT'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:40:48
               │
            40 │             FUNCTION f_inout : DINT VAR_IN_OUT x : IA; END_VAR END_FUNCTION
               │                                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_non_pou_to_interface_input() {
            insta::assert_snapshot!(validate("f_val(d);"), @"
            error[E126]: Invalid assignment: 'DINT' does not implement interface 'IA'
               ┌─ <internal>:59:23
               │
            59 │                 f_val(d);
               │                       ^ Invalid assignment: 'DINT' does not implement interface 'IA'
            ");
        }

        // -------------------------------------------------------------------
        // REFERENCE TO an interface
        // -------------------------------------------------------------------
        #[test]
        fn ref_assign_by_ref_exact_interface_is_valid() {
            insta::assert_snapshot!(validate("rr REF= ia;"), @"");
        }

        #[test]
        fn ref_assign_by_ref_to_interface_pointer_is_valid() {
            insta::assert_snapshot!(validate("p REF= ia; q REF= ia;"), @"");
        }

        #[test]
        fn ref_assign_by_ref_null_is_valid() {
            insta::assert_snapshot!(validate("rr REF= 0; p REF= 0;"), @"");
        }

        #[test]
        fn ref_assign_by_ref_to_pointer_parameter() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                VAR_GLOBAL g : IA; END_VAR

                FUNCTION set_pointer : DINT
                    VAR_IN_OUT x : REF_TO IA; END_VAR
                    x REF= g;
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'REF_TO IA' but received 'IA'; bind 'x' to a variable of type 'REF_TO IA'
              ┌─ <internal>:9:21
              │
            9 │                     x REF= g;
              │                     ^^^^^^^^ Type mismatch: expected type 'REF_TO IA' but received 'IA'; bind 'x' to a variable of type 'REF_TO IA'

            note[E159]: 'x' is a reference and needs a variable of type 'REF_TO IA'
              ┌─ <internal>:8:32
              │
            8 │                     VAR_IN_OUT x : REF_TO IA; END_VAR
              │                                ^ 'x' is a reference and needs a variable of type 'REF_TO IA'
            ");
        }

        #[test]
        fn ref_assign_by_ref_non_reference_and_alias_are_left_to_validator() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                INTERFACE IB EXTENDS IA
                    METHOD bar END_METHOD
                END_INTERFACE

                FUNCTION main
                    VAR ia : IA; ib : IB; p : REF_TO IA; al AT ia : IA; END_VAR
                    p REF= 1;
                    al REF= ib;
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E098]: Invalid assignment, expected a reference
               ┌─ <internal>:11:28
               │
            11 │                     p REF= 1;
               │                            ^ Invalid assignment, expected a reference

            error[E100]: al is an immutable alias variable, can not change the address
               ┌─ <internal>:12:21
               │
            12 │                     al REF= ib;
               │                     ^^^^^^^^^^ al is an immutable alias variable, can not change the address
            ");
        }

        #[test]
        fn ref_assign_by_ref_property_and_call_results() {
            let source = r#"
                INTERFACE IA
                    METHOD foo : DINT END_METHOD
                END_INTERFACE
                INTERFACE IB EXTENDS IA
                    METHOD bar END_METHOD
                END_INTERFACE
                FUNCTION_BLOCK FbB IMPLEMENTS IB
                    METHOD foo : DINT END_METHOD
                    METHOD bar END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK Holder
                    VAR ib : IB; END_VAR
                    PROPERTY_GET PIB : IB
                        PIB := ib;
                    END_PROPERTY
                END_FUNCTION_BLOCK

                FUNCTION getIB : IB END_FUNCTION
                FUNCTION getB : FbB END_FUNCTION

                FUNCTION main
                    VAR h : Holder; ia : IA; r : REFERENCE TO IA; p : REF_TO IA; END_VAR
                    r REF= h.PIB;
                    p REF= getIB();
                    r REF= getB();
                    r REF= ia.foo();
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; bind 'r' to a variable of type 'IA'
               ┌─ <internal>:25:21
               │
            25 │                     r REF= h.PIB;
               │                     ^^^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'IB'; bind 'r' to a variable of type 'IA'

            note[E159]: 'r' is a reference and needs a variable of type 'IA'
               ┌─ <internal>:24:46
               │
            24 │                     VAR h : Holder; ia : IA; r : REFERENCE TO IA; p : REF_TO IA; END_VAR
               │                                              ^ 'r' is a reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; bind 'p' to a variable of type 'IA'
               ┌─ <internal>:26:21
               │
            26 │                     p REF= getIB();
               │                     ^^^^^^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'IB'; bind 'p' to a variable of type 'IA'

            note[E159]: 'p' is a reference and needs a variable of type 'IA'
               ┌─ <internal>:24:67
               │
            24 │                     VAR h : Holder; ia : IA; r : REFERENCE TO IA; p : REF_TO IA; END_VAR
               │                                                                   ^ 'p' is a reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbB'; bind 'r' to a variable of type 'IA'
               ┌─ <internal>:27:21
               │
            27 │                     r REF= getB();
               │                     ^^^^^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'FbB'; bind 'r' to a variable of type 'IA'

            note[E159]: 'r' is a reference and needs a variable of type 'IA'
               ┌─ <internal>:24:46
               │
            24 │                     VAR h : Holder; ia : IA; r : REFERENCE TO IA; p : REF_TO IA; END_VAR
               │                                              ^ 'r' is a reference and needs a variable of type 'IA'

            error[E098]: Invalid assignment, expected a reference
               ┌─ <internal>:28:28
               │
            28 │                     r REF= ia.foo();
               │                            ^^^^^^^^^ Invalid assignment, expected a reference
            ");
        }

        #[test]
        fn ref_assign_by_ref_interface_to_reference_to_pointer() {
            insta::assert_snapshot!(validate("rp REF= ia;"), @"
            error[E126]: Type mismatch: expected type 'REF_TO IA' but received 'IA'; bind 'rp' to a variable of type 'REF_TO IA'
               ┌─ <internal>:59:17
               │
            59 │                 rp REF= ia;
               │                 ^^^^^^^^^^ Type mismatch: expected type 'REF_TO IA' but received 'IA'; bind 'rp' to a variable of type 'REF_TO IA'

            note[E159]: 'rp' is a reference and needs a variable of type 'REF_TO IA'
               ┌─ <internal>:56:80
               │
            56 │                     arr : ARRAY[1..2] OF IA; h : Holder; rr : REFERENCE TO IA; rp : REFERENCE TO REF_TO IA;
               │                                                                                ^^ 'rp' is a reference and needs a variable of type 'REF_TO IA'
            ");
        }

        #[test]
        fn ref_assign_by_ref_implementor() {
            insta::assert_snapshot!(validate("rr REF= a;"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; bind 'rr' to a variable of type 'IA'
               ┌─ <internal>:59:17
               │
            59 │                 rr REF= a;
               │                 ^^^^^^^^^ Type mismatch: expected type 'IA' but received 'FbA'; bind 'rr' to a variable of type 'IA'

            note[E159]: 'rr' is a reference and needs a variable of type 'IA'
               ┌─ <internal>:56:58
               │
            56 │                     arr : ARRAY[1..2] OF IA; h : Holder; rr : REFERENCE TO IA; rp : REFERENCE TO REF_TO IA;
               │                                                          ^^ 'rr' is a reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_to_action_parameters() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                FUNCTION_BLOCK FbA IMPLEMENTS IA
                    METHOD foo END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK FbP
                    VAR_IN_OUT x : IA; END_VAR
                    VAR_OUTPUT o : IA; END_VAR
                END_FUNCTION_BLOCK
                ACTIONS FbP
                    ACTION act END_ACTION
                END_ACTIONS

                PROGRAM prg
                    VAR_IN_OUT x : IA; END_VAR
                END_PROGRAM
                ACTIONS prg
                    ACTION pact END_ACTION
                END_ACTIONS

                FUNCTION main
                    VAR a : FbA; ia : IA; p : FbP; END_VAR
                    p.act(x := a, o => a);
                    prg.pact(x := a);
                    p.act(x := ia, o => ia);
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'p.act'
               ┌─ <internal>:26:32
               │
            26 │                     p.act(x := a, o => a);
               │                                ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'p.act'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:32
               │
            10 │                     VAR_IN_OUT x : IA; END_VAR
               │                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'p.act'
               ┌─ <internal>:26:40
               │
            26 │                     p.act(x := a, o => a);
               │                                        ^ Type mismatch: expected type 'IA' but received 'FbA'; pass a variable of type 'IA' to 'p.act'

            note[E159]: Parameter 'o' is an output and needs a variable of type 'IA'
               ┌─ <internal>:11:32
               │
            11 │                     VAR_OUTPUT o : IA; END_VAR
               │                                ^ Parameter 'o' is an output and needs a variable of type 'IA'

            error[E126]: Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'prg.pact'
               ┌─ <internal>:27:35
               │
            27 │                     prg.pact(x := a);
               │                                   ^ Type mismatch: expected type 'IA' but received 'FbA'; assign 'a' to a variable of type 'IA' before passing it to 'prg.pact'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:18:32
               │
            18 │                     VAR_IN_OUT x : IA; END_VAR
               │                                ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_type_names_as_declared() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE

                FUNCTION f_inout : DINT
                    VAR_IN_OUT x : ARRAY[0..1] OF DINT; s : STRING[10]; END_VAR
                END_FUNCTION
                FUNCTION f_ref : DINT
                    VAR_INPUT x : REFERENCE TO IA; END_VAR
                END_FUNCTION
                FUNCTION f_val : DINT
                    VAR_INPUT x : IA; END_VAR
                END_FUNCTION

                FUNCTION_BLOCK FbC IMPLEMENTS IA
                    METHOD foo END_METHOD
                    METHOD pass f_val(THIS); END_METHOD
                END_FUNCTION_BLOCK

                FUNCTION main
                    VAR ia : IA; pia : REF_TO IA; END_VAR
                    f_inout(ia, ia);
                    f_ref(pia);
                    f_val('abc');
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Invalid assignment: 'REF_TO FbC' does not implement interface 'IA'
               ┌─ <internal>:18:39
               │
            18 │                     METHOD pass f_val(THIS); END_METHOD
               │                                       ^^^^ Invalid assignment: 'REF_TO FbC' does not implement interface 'IA'

            error[E126]: Type mismatch: expected type 'ARRAY[0..1] OF DINT' but received 'IA'
               ┌─ <internal>:23:29
               │
            23 │                     f_inout(ia, ia);
               │                             ^^ Type mismatch: expected type 'ARRAY[0..1] OF DINT' but received 'IA'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'ARRAY[0..1] OF DINT'
              ┌─ <internal>:7:32
              │
            7 │                     VAR_IN_OUT x : ARRAY[0..1] OF DINT; s : STRING[10]; END_VAR
              │                                ^ Parameter 'x' is passed by reference and needs a variable of type 'ARRAY[0..1] OF DINT'

            error[E126]: Type mismatch: expected type 'STRING[10]' but received 'IA'
               ┌─ <internal>:23:33
               │
            23 │                     f_inout(ia, ia);
               │                                 ^^ Type mismatch: expected type 'STRING[10]' but received 'IA'

            note[E159]: Parameter 's' is passed by reference and needs a variable of type 'STRING[10]'
              ┌─ <internal>:7:57
              │
            7 │                     VAR_IN_OUT x : ARRAY[0..1] OF DINT; s : STRING[10]; END_VAR
              │                                                         ^ Parameter 's' is passed by reference and needs a variable of type 'STRING[10]'

            error[E126]: Type mismatch: expected type 'IA' but received 'REF_TO IA'
               ┌─ <internal>:24:27
               │
            24 │                     f_ref(pia);
               │                           ^^^ Type mismatch: expected type 'IA' but received 'REF_TO IA'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'IA'
               ┌─ <internal>:10:31
               │
            10 │                     VAR_INPUT x : REFERENCE TO IA; END_VAR
               │                               ^ Parameter 'x' is passed by reference and needs a variable of type 'IA'

            error[E126]: Invalid assignment: 'STRING[3]' does not implement interface 'IA'
               ┌─ <internal>:25:27
               │
            25 │                     f_val('abc');
               │                           ^^^^^ Invalid assignment: 'STRING[3]' does not implement interface 'IA'
            ");
        }

        #[test]
        fn call_arg_by_ref_pointer_target_and_alias_names() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE
                TYPE MyInt : DINT; END_TYPE

                FUNCTION f_reference : DINT
                    VAR_IN_OUT x : REFERENCE TO IA; END_VAR
                END_FUNCTION
                FUNCTION f_alias : DINT
                    VAR_IN_OUT x : ARRAY[0..1] OF MyInt; END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR ia : IA; lr : REFERENCE TO IA; END_VAR
                    lr REF= ia;
                    f_reference(lr);
                    f_alias(ia);
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E126]: Type mismatch: expected type 'ARRAY[0..1] OF MyInt' but received 'IA'
               ┌─ <internal>:18:29
               │
            18 │                     f_alias(ia);
               │                             ^^ Type mismatch: expected type 'ARRAY[0..1] OF MyInt' but received 'IA'

            note[E159]: Parameter 'x' is passed by reference and needs a variable of type 'ARRAY[0..1] OF MyInt'
               ┌─ <internal>:11:32
               │
            11 │                     VAR_IN_OUT x : ARRAY[0..1] OF MyInt; END_VAR
               │                                ^ Parameter 'x' is passed by reference and needs a variable of type 'ARRAY[0..1] OF MyInt'
            ");
        }

        #[test]
        fn call_arg_by_ref_interface_to_variable_length_array() {
            let source = r#"
                INTERFACE IA
                    METHOD foo END_METHOD
                END_INTERFACE

                FUNCTION f_vla : DINT
                    VAR_IN_OUT x : ARRAY[*] OF DINT; END_VAR
                END_FUNCTION

                FUNCTION main
                    VAR ia : IA; END_VAR
                    f_vla(ia);
                END_FUNCTION
            "#;
            insta::assert_snapshot!(parse_and_validate_buffered(source), @"
            error[E037]: Invalid assignment: cannot assign '__FATPOINTER' to 'ARRAY[*] OF DINT'
               ┌─ <internal>:12:27
               │
            12 │                     f_vla(ia);
               │                           ^^ Invalid assignment: cannot assign '__FATPOINTER' to 'ARRAY[*] OF DINT'
            ");
        }

        #[test]
        fn ref_assign_by_ref_child_interface() {
            insta::assert_snapshot!(validate("rr REF= ib;"), @"
            error[E126]: Type mismatch: expected type 'IA' but received 'IB'; bind 'rr' to a variable of type 'IA'
               ┌─ <internal>:59:17
               │
            59 │                 rr REF= ib;
               │                 ^^^^^^^^^^ Type mismatch: expected type 'IA' but received 'IB'; bind 'rr' to a variable of type 'IA'

            note[E159]: 'rr' is a reference and needs a variable of type 'IA'
               ┌─ <internal>:56:58
               │
            56 │                     arr : ARRAY[1..2] OF IA; h : Holder; rr : REFERENCE TO IA; rp : REFERENCE TO REF_TO IA;
               │                                                          ^^ 'rr' is a reference and needs a variable of type 'IA'
            ");
        }
    }

    #[test]
    fn call_method_declared_on_interface() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                refIA.foo();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_inherited_method_on_child_interface() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIB : IB;
                END_VAR
                refIB.foo();
                refIB.bar();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_method_not_on_interface() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD

                METHOD onlyOnFbA
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                refIA.onlyOnFbA();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E048]: Could not resolve reference to onlyOnFbA
           ┌─ <internal>:19:23
           │
        19 │                 refIA.onlyOnFbA();
           │                       ^^^^^^^^^ Could not resolve reference to onlyOnFbA
        ");
    }

    #[test]
    fn call_child_interface_method_through_parent_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            INTERFACE IB EXTENDS IA
                METHOD bar
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IB
                METHOD foo
                END_METHOD

                METHOD bar
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                refIA.bar();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E048]: Could not resolve reference to bar
           ┌─ <internal>:24:23
           │
        24 │                 refIA.bar();
           │                       ^^^ Could not resolve reference to bar
        ");
    }

    #[test]
    fn call_interface_ref_directly() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                refIA();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E129]: Interfaces cannot be called directly
           ┌─ <internal>:16:17
           │
        16 │                 refIA();
           │                 ^^^^^ Interfaces cannot be called directly
        ");
    }

    #[test]
    fn call_qualified_interface_ref_directly() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK Container
                VAR
                    refIA : IA;
                END_VAR

                THIS^.refIA();
            END_FUNCTION_BLOCK
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E129]: Interfaces cannot be called directly
           ┌─ <internal>:12:17
           │
        12 │                 THIS^.refIA();
           │                 ^^^^^^^^^^^ Interfaces cannot be called directly
        ");
    }

    #[test]
    fn call_interface_ref_array_element_directly() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refs : ARRAY[1..2] OF IA;
                    i    : DINT;
                END_VAR
                refs[i]();
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E129]: Interfaces cannot be called directly
           ┌─ <internal>:17:17
           │
        17 │                 refs[i]();
           │                 ^^^^^^^ Interfaces cannot be called directly
        ");
    }

    #[test]
    fn access_field_through_interface_ref() {
        // Interfaces only expose methods — field access is invalid
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                VAR
                    someField : DINT;
                END_VAR

                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                    x     : DINT;
                END_VAR
                x := refIA.someField;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E048]: Could not resolve reference to someField
           ┌─ <internal>:21:28
           │
        21 │                 x := refIA.someField;
           │                            ^^^^^^^^^ Could not resolve reference to someField
        ");
    }

    #[test]
    fn assign_to_field_through_interface_ref() {
        let source = r#"
            INTERFACE IA
                METHOD foo
                END_METHOD
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
                VAR
                    someField : DINT;
                END_VAR

                METHOD foo
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                refIA.someField := 42;
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);
        insta::assert_snapshot!(diagnostics, @r"
        error[E048]: Could not resolve reference to someField
           ┌─ <internal>:20:23
           │
        20 │                 refIA.someField := 42;
           │                       ^^^^^^^^^ Could not resolve reference to someField
        ");
    }
}
