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
    fn call_arg_same_interface_to_byref_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION consumer
                VAR_OUTPUT
                    out1 : IA;
                END_VAR
                VAR_IN_OUT
                    inout1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumerRef
                VAR_INPUT
                    ref1 : REFERENCE TO IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    refIA : IA;
                END_VAR
                consumer(refIA, refIA);
                consumer(out1 => refIA, inout1 := refIA);
                consumerRef(refIA);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn call_arg_child_interface_to_byref_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            INTERFACE IB EXTENDS IA
            END_INTERFACE

            FUNCTION consumer
                VAR_OUTPUT
                    out1 : IA;
                END_VAR
                VAR_IN_OUT
                    inout1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION consumerRef
                VAR_INPUT
                    ref1 : REFERENCE TO IA;
                END_VAR
            END_FUNCTION

            FUNCTION main
                VAR
                    refIB : IB;
                END_VAR
                consumer(refIB, refIB);
                consumer(out1 => refIB, inout1 := refIB);
                consumerRef(refIB);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"
        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:27:26
           │
        27 │                 consumer(refIB, refIB);
           │                          ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:27:33
           │
        27 │                 consumer(refIB, refIB);
           │                                 ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:28:34
           │
        28 │                 consumer(out1 => refIB, inout1 := refIB);
           │                                  ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:28:51
           │
        28 │                 consumer(out1 => refIB, inout1 := refIB);
           │                                                   ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter ref1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:29:29
           │
        29 │                 consumerRef(refIB);
           │                             ^^^^^ Parameter ref1 is passed by reference, it requires an argument of type IA but got IB
        ");
    }

    #[test]
    fn call_arg_parent_interface_to_byref_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            INTERFACE IB EXTENDS IA
            END_INTERFACE

            FUNCTION consumer
                VAR_IN_OUT
                    inout1 : IB;
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

        insta::assert_snapshot!(diagnostics, @"
        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IB but got IA
           ┌─ <internal>:18:26
           │
        18 │                 consumer(refIA);
           │                          ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IB but got IA
        ");
    }

    #[test]
    fn call_arg_child_interface_to_function_block_and_program_output() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            INTERFACE IB EXTENDS IA
            END_INTERFACE

            FUNCTION_BLOCK FbConsumer
                VAR_OUTPUT
                    out1 : IA;
                END_VAR
                VAR_IN_OUT
                    inout1 : IA;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbConsumerChild EXTENDS FbConsumer
                VAR_INPUT
                    in1 : DINT;
                END_VAR
            END_FUNCTION_BLOCK

            PROGRAM progConsumer
                VAR_OUTPUT
                    out1 : IA;
                END_VAR
            END_PROGRAM

            FUNCTION main
                VAR
                    fbConsumer : FbConsumer;
                    fbConsumerChild : FbConsumerChild;
                    refIA : IA;
                    refIB : IB;
                END_VAR
                fbConsumer(refIA, refIA);
                fbConsumer(refIB, refIB);
                fbConsumer(out1 => refIB, inout1 := refIB);
                progConsumer(refIA);
                progConsumer(refIB);
                progConsumer(out1 => refIB);
                fbConsumerChild(refIB, refIB, 1);
                fbConsumerChild(in1 := 1, out1 => refIB, inout1 := refIB);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"
        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:37:28
           │
        37 │                 fbConsumer(refIB, refIB);
           │                            ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:37:35
           │
        37 │                 fbConsumer(refIB, refIB);
           │                                   ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:38:36
           │
        38 │                 fbConsumer(out1 => refIB, inout1 := refIB);
           │                                    ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:38:53
           │
        38 │                 fbConsumer(out1 => refIB, inout1 := refIB);
           │                                                     ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:40:30
           │
        40 │                 progConsumer(refIB);
           │                              ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:41:38
           │
        41 │                 progConsumer(out1 => refIB);
           │                                      ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:42:33
           │
        42 │                 fbConsumerChild(refIB, refIB, 1);
           │                                 ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:42:40
           │
        42 │                 fbConsumerChild(refIB, refIB, 1);
           │                                        ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter out1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:43:51
           │
        43 │                 fbConsumerChild(in1 := 1, out1 => refIB, inout1 := refIB);
           │                                                   ^^^^^ Parameter out1 is passed by reference, it requires an argument of type IA but got IB

        error[E037]: Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
           ┌─ <internal>:43:68
           │
        43 │                 fbConsumerChild(in1 := 1, out1 => refIB, inout1 := refIB);
           │                                                                    ^^^^^ Parameter inout1 is passed by reference, it requires an argument of type IA but got IB
        ");
    }

    #[test]
    fn call_arg_call_result_to_byref_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION producer : FbA
            END_FUNCTION

            FUNCTION producerArray : ARRAY[0..1] OF FbA
            END_FUNCTION

            VAR_GLOBAL
                globalInstance : FbA;
            END_VAR

            FUNCTION getPtr : POINTER TO FbA
                getPtr := ADR(globalInstance);
            END_FUNCTION

            FUNCTION producerIA : IA
                producerIA := globalInstance;
            END_FUNCTION

            FUNCTION consumer
                VAR_INPUT
                    in1 : IA;
                END_VAR
                VAR_IN_OUT
                    inout1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION_BLOCK FbConsumer
                VAR_INPUT
                    ref1 : REFERENCE TO IA;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION main
                VAR
                    instance : FbA;
                    instances : ARRAY[0..1] OF FbA;
                    fbConsumer : FbConsumer;
                END_VAR
                consumer(producer(), instance);
                consumer(instance, producer());
                fbConsumer(producer());
                fbConsumer(producerArray()[0]);
                fbConsumer(FbA#(producerArray()[0]));
                fbConsumer(instances[0]);
                fbConsumer(getPtr()^);
                fbConsumer(FbA#(instance));
                fbConsumer(producerIA());
                consumer(producerIA(), producerIA());
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"
        error[E031]: Expected a reference for parameter inout1 because their type is InOut
           ┌─ <internal>:48:36
           │
        48 │                 consumer(instance, producer());
           │                                    ^^^^^^^^^^^ Expected a reference for parameter inout1 because their type is InOut

        error[E031]: Expected a reference for parameter ref1 because their type is REFERENCE TO
           ┌─ <internal>:49:28
           │
        49 │                 fbConsumer(producer());
           │                            ^^^^^^^^^^^ Expected a reference for parameter ref1 because their type is REFERENCE TO

        error[E031]: Expected a reference for parameter ref1 because their type is REFERENCE TO
           ┌─ <internal>:50:28
           │
        50 │                 fbConsumer(producerArray()[0]);
           │                            ^^^^^^^^^^^^^^^^ Expected a reference for parameter ref1 because their type is REFERENCE TO

        error[E031]: Expected a reference for parameter ref1 because their type is REFERENCE TO
           ┌─ <internal>:51:28
           │
        51 │                 fbConsumer(FbA#(producerArray()[0]));
           │                            ^^^^^^^^^^^^^^^^^^^^^^^^ Expected a reference for parameter ref1 because their type is REFERENCE TO

        error[E031]: Expected a reference for parameter ref1 because their type is REFERENCE TO
           ┌─ <internal>:55:28
           │
        55 │                 fbConsumer(producerIA());
           │                            ^^^^^^^^^^^^^ Expected a reference for parameter ref1 because their type is REFERENCE TO

        error[E031]: Expected a reference for parameter inout1 because their type is InOut
           ┌─ <internal>:56:40
           │
        56 │                 consumer(producerIA(), producerIA());
           │                                        ^^^^^^^^^^^^^ Expected a reference for parameter inout1 because their type is InOut
        ");
    }

    #[test]
    fn call_arg_this_backed_to_byref_param() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbConsumer
                VAR_INPUT
                    ref1 : REFERENCE TO IA;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK FbHolder IMPLEMENTS IA
                VAR
                    instance : FbA;
                    consumer : FbConsumer;
                END_VAR

                METHOD bind
                    consumer(THIS^.instance);
                    consumer(THIS^);
                    consumer(instance);
                END_METHOD
            END_FUNCTION_BLOCK
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"");
    }

    #[test]
    fn short_circuit_with_wrapped_argument_keeps_operand_check() {
        let source = r#"
            INTERFACE IA
            END_INTERFACE

            FUNCTION_BLOCK FbA IMPLEMENTS IA
            END_FUNCTION_BLOCK

            FUNCTION consume : BOOL
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION count : DINT
                VAR_INPUT
                    in1 : IA;
                END_VAR
            END_FUNCTION

            FUNCTION main : BOOL
                VAR
                    flag : BOOL;
                    instance : FbA;
                    ptr : POINTER TO BOOL;
                END_VAR
                main := 1 AND_THEN consume(instance);
                main := flag AND_THEN count(instance);
                ptr REF= flag AND_THEN consume(instance);
            END_FUNCTION
        "#;

        let diagnostics = parse_and_validate_buffered(source);

        insta::assert_snapshot!(diagnostics, @"
        error[E133]: `AND_THEN` requires boolean operands, use `AND` for bitwise operations on non-boolean types
           ┌─ <internal>:26:25
           │
        26 │                 main := 1 AND_THEN consume(instance);
           │                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `AND_THEN` requires boolean operands, use `AND` for bitwise operations on non-boolean types

        error[E133]: `AND_THEN` requires boolean operands, use `AND` for bitwise operations on non-boolean types
           ┌─ <internal>:27:25
           │
        27 │                 main := flag AND_THEN count(instance);
           │                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `AND_THEN` requires boolean operands, use `AND` for bitwise operations on non-boolean types

        warning[E067]: Implicit downcast from 'DINT' to 'BOOL'.
           ┌─ <internal>:27:39
           │
        27 │                 main := flag AND_THEN count(instance);
           │                                       ^^^^^^^^^^^^^^^ Implicit downcast from 'DINT' to 'BOOL'.

        error[E098]: Invalid assignment, expected a reference
           ┌─ <internal>:28:26
           │
        28 │                 ptr REF= flag AND_THEN consume(instance);
           │                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment, expected a reference
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
