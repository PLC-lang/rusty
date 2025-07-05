use plc_ast::{ast::AstStatement, provider::IdProvider};

use crate::{
    resolver::AnnotationMap,
    test_utils::tests::{annotate_with_ids, index_with_ids},
};

#[test]
fn function_pointer_initialization() {
    let ids = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"
        FUNCTION pointAtMe : DINT
            VAR_INPUT
                x : DINT;
                y : DINT;
            END_VAR
        END_FUNCTION

        FUNCTION main
            VAR
                myFunctionPointer : REF_TO pointAtMe := REF(pointAtMe);
            END_VAR
        END_FUNCTION
        ",
        ids.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, ids);

    // myFunctionPointer : REF_TO pointAtMe
    let initializer = unit.pous[1].variable_blocks[0].variables[0].initializer.as_ref().unwrap();
    insta::assert_debug_snapshot!(annotations.get(initializer), @r#"
    Some(
        Value {
            resulting_type: "__POINTER_TO_pointAtMe",
        },
    )
    "#);
    insta::assert_debug_snapshot!(annotations.get_hint(initializer), @r#"
    Some(
        Value {
            resulting_type: "__main_myFunctionPointer",
        },
    )
    "#);
}

#[test]
fn function_pointer_assignment() {
    let ids = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"
        FUNCTION pointAtMe : DINT
            VAR_INPUT
                x : DINT;
                y : DINT;
            END_VAR
        END_FUNCTION

        FUNCTION main
            VAR
                myFunctionPointer : REF_TO pointAtMe;
            END_VAR

            myFunctionPointer := REF(pointAtMe);
        END_FUNCTION
        ",
        ids.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, ids);

    // myFunctionPointer := REF(pointAtMe);
    let AstStatement::Assignment(assignment) = unit.implementations[1].statements[0].get_stmt() else {
        unreachable!();
    };

    // myFunctionPointer := REF(pointAtMe);
    // ^^^^^^^^^^^^^^^^^
    insta::assert_debug_snapshot!(annotations.get(&assignment.left), @r#"
        Some(
            Variable {
                resulting_type: "__main_myFunctionPointer",
                qualified_name: "main.myFunctionPointer",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
    "#);
    insta::assert_debug_snapshot!(annotations.get_hint(&assignment.left), @"None");

    // myFunctionPointer := REF(pointAtMe);
    //                      ^^^^^^^^^^^^^^
    insta::assert_debug_snapshot!(annotations.get(&assignment.right), @r#"
        Some(
            Value {
                resulting_type: "__POINTER_TO_pointAtMe",
            },
        )
    "#);
    insta::assert_debug_snapshot!(annotations.get_hint(&assignment.right), @r#"
        Some(
            Value {
                resulting_type: "__main_myFunctionPointer",
            },
        )
    "#);
}

#[test]
fn function_pointer_deref() {
    let ids = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"
        FUNCTION pointAtMe : DINT
            VAR_INPUT
                x : DINT;
                y : DINT;
            END_VAR
        END_FUNCTION

        FUNCTION main
            VAR
                myFunctionPointer : REF_TO pointAtMe;
            END_VAR

            myFunctionPointer^(1, 2);
        END_FUNCTION
        ",
        ids.clone(),
    );

    let annotations = annotate_with_ids(&unit, &mut index, ids);

    // myFunctionPointer^(1, 2);
    let AstStatement::CallStatement(call) = unit.implementations[1].statements[0].get_stmt() else {
        unreachable!();
    };

    // TODO(vosa): This should be myFunctionPointer?
    insta::assert_debug_snapshot!(annotations.get(&call.operator), @r#"
        Some(
            Value {
                resulting_type: "pointAtMe",
            },
        )
    "#);
    insta::assert_debug_snapshot!(annotations.get_hint(&call.operator), @"None");
}
