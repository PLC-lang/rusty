use plc_ast::{
    ast::{Assignment, AstStatement, CallStatement},
    provider::IdProvider,
};

use crate::{
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn interface_as_variable_type() {
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

#[test]
fn interface_as_variable_input_type() {
    let ids = IdProvider::default();
    let (unit, index) = index_with_ids(
        r#"
        INTERFACE IntfA
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS IntfA
            VAR_INPUT
                refIntfA: IntfA;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
            END_VAR

            instanceFbA(refIntfA := instanceFbA);
        END_FUNCTION
        "#,
        ids.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ids);
    let statements = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements;

    // instanceFbA(refIntfA := instanceFbA);
    //             ^^^^^^^^^^^^^^^^^^^^^^^
    let AstStatement::CallStatement(CallStatement { parameters, .. }) = statements[0].get_stmt() else {
        unreachable!();
    };

    // refIntfA := instanceFbA;
    // ^^^^^^^^^^^^^^^^^^^^^^^
    let AstStatement::Assignment(Assignment { left, right }) = parameters.as_ref().unwrap().get_stmt() else {
        unreachable!();
    };

    // refIntfA := instanceFbA;
    // ^^^^^^^^
    insta::assert_debug_snapshot!((annotations.get(left), annotations.get_type(left, &index)), @r#"
    (
        Some(
            Variable {
                resulting_type: "IntfA",
                qualified_name: "FbA.refIntfA",
                constant: false,
                argument_type: ByVal(
                    Input,
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

#[test]
fn interface_as_array_type() {
    let ids = IdProvider::default();
    let (unit, index) = index_with_ids(
        r#"
        INTERFACE IntfA
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS IntfA
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instance: FbA;
                interfaces: ARRAY[1..5] OF IntfA;
            END_VAR

            interfaces[0] := instance;
        END_FUNCTION
        "#,
        ids.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ids);
    let statements = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements;

    // interfaces[0] := instance;
    // ^^^^^^^^^^^^^^^^^^^^^^^^^
    let AstStatement::Assignment(Assignment { left, right }) = statements[0].get_stmt() else {
        unreachable!();
    };

    // interfaces[0] := instance;
    // ^^^^^^^^^^^^^
    insta::assert_debug_snapshot!((annotations.get(left), annotations.get_type(left, &index)), @r#"
    (
        Some(
            Value {
                resulting_type: "IntfA",
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

    // interfaces[0] := instance;
    //                  ^^^^^^^^
    insta::assert_debug_snapshot!((annotations.get(right), annotations.get_type_hint(right, &index)), @r#"
    (
        Some(
            Variable {
                resulting_type: "FbA",
                qualified_name: "main.instance",
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

#[test]
fn interface_as_return_type() {
    let ids = IdProvider::default();
    let (unit, _) = index_with_ids(
        r#"
        INTERFACE IntfA
        END_INTERFACE

        FUNCTION foo: IntfA
        END_FUNCTION
        "#,
        ids.clone(),
    );

    insta::assert_debug_snapshot!(&unit.pous.iter().find(|imp| imp.name == "foo").unwrap().return_type, @r#"
    Some(
        DataTypeReference {
            referenced_type: "IntfA",
        },
    )
    "#);
}
