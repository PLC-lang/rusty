use plc_ast::{ast::AstStatement, provider::IdProvider};

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
