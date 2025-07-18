use plc_ast::{
    ast::{AstStatement, ReferenceAccess, ReferenceExpr},
    provider::IdProvider,
};

use crate::{
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn function_pointer_definition() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        r"
        FUNCTION echo : DINT
            VAR_INPUT
                value : INT;
            END_VAR

            echo := value;
        END_FUNCTION

        FUNCTION main
            VAR
                echoPtr : REF_TO echo;
            END_VAR

            echoPtr := REF(echo);
            echoPtr^(12345);
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);

    // echoPtr := REF_TO echo;
    {
        let node = &unit.implementations[1].statements[0];
        let AstStatement::Assignment(assignment) = node.get_stmt() else {
            unreachable!();
        };

        // echoPtr := REF(echo);
        // ^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(&assignment.left), @r#"
        Some(
            Variable {
                resulting_type: "__main_echoPtr",
                qualified_name: "main.echoPtr",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(&assignment.left), @"None");

        // echoPtr := REF(echo);
        //            ^^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(&assignment.right), @r#"
        Some(
            Value {
                resulting_type: "__POINTER_TO_echo",
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(&assignment.right), @r#"
        Some(
            Value {
                resulting_type: "__main_echoPtr",
            },
        )
        "#);
    }

    {
        let node = &unit.implementations[1].statements[1];
        let AstStatement::CallStatement(call) = node.get_stmt() else {
            unreachable!();
        };

        // echoPtr^(12345);
        // ^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(&call.operator), @r#"
        Some(
            Value {
                resulting_type: "__main_echoPtr",
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(&call.operator), @"None");
    }
}

#[test]
fn void_pointer_casting() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        r"
        VAR_GLOBAL
            vtable_FbA_instance: vtable_FbA;
        END_VAR

        TYPE vtable_FbA: STRUCT
            foo: POINTER TO FbA.foo := ADR(FbA.foo);
        END_STRUCT END_TYPE

        FUNCTION_BLOCK FbA
            VAR
                __vtable: POINTER TO __VOID;
            END_VAR

            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK


        FUNCTION main
            VAR
                instanceFbA: FbA;
            END_VAR

            vtable_FbA#(instanceFbA.__vtable);
            vtable_FbA#(instanceFbA.__vtable).foo^(instanceFbA);
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    {
        let node = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements[0];

        // vtable_FbA#(instanceFbA.__vtable)
        //             ^^^^^^^^^^^^^^^^^^^^
        let AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Cast(target), .. }) =
            node.get_stmt()
        else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(annotations.get(target), @r#"
        Some(
            Variable {
                resulting_type: "__FbA___vtable",
                qualified_name: "FbA.__vtable",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(target), @"None");

        // vtable_FbA#(instanceFbA.__vtable)
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(node), @r#"
        Some(
            Value {
                resulting_type: "vtable_FbA",
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(node), @"None");
    }

    {
        let node = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements[1];

        // vtable_FbA#(instanceFbA.__vtable).foo^(instanceFbA);
        // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        let AstStatement::CallStatement(call) = node.get_stmt() else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(annotations.get(&call.operator), @r#"
        Some(
            Value {
                resulting_type: "__vtable_FbA_foo",
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(&call.operator), @"None");
    }
}
