use plc_ast::{
    ast::{Assignment, AstStatement},
    provider::IdProvider,
};

use crate::{
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn interface_can_be_used_as_variable_type() {
    let ids = IdProvider::default();
    let (unit, index) = index_with_ids(
        r#"
        INTERFACE IntfA
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS IntfA
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                refIntfA: IntfA;
                instanceFbA: FbA;
            END_VAR

            refIntfA := instanceFbA;
        END_FUNCTION
        "#,
        ids.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ids);
    let statements = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements;

    // refIntfA := instanceFbA;
    // ^^^^^^^^^^^^^^^^^^^^^^^
    let AstStatement::Assignment(Assignment { left, right }) = statements[0].get_stmt() else {
        unreachable!();
    };

    // refIntfA := instanceFbA;
    // ^^^^^^^^
    insta::assert_debug_snapshot!((annotations.get(left), annotations.get_type(left, &index)), @r#"
    (
        Some(
            Variable {
                resulting_type: "IntfA",
                qualified_name: "main.refIntfA",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        ),
        Some(
            DataType {
                name: "IntfA",
                initial_value: None,
                information: Interface {
                    name: "IntfA",
                },
                nature: Any,
                location: SourceLocation {
                    span: Range(1:18 - 1:23),
                    file: Some(
                        "<internal>",
                    ),
                },
            },
        ),
    )
    "#);

    // refIntfA := instanceFbA;
    //             ^^^^^^^^^^^
    insta::assert_debug_snapshot!((annotations.get(right), annotations.get_type_hint(right, &index)), @r#"
    (
        Some(
            Variable {
                resulting_type: "FbA",
                qualified_name: "main.instanceFbA",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        ),
        Some(
            DataType {
                name: "IntfA",
                initial_value: None,
                information: Interface {
                    name: "IntfA",
                },
                nature: Any,
                location: SourceLocation {
                    span: Range(1:18 - 1:23),
                    file: Some(
                        "<internal>",
                    ),
                },
            },
        ),
    )
    "#);
}
