use plc_ast::{
    ast::{Assignment, AstStatement, CallStatement, ReferenceAccess, ReferenceExpr},
    provider::IdProvider,
};

use crate::{
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn function_pointer_method_with_no_arguments() {
    let ids = IdProvider::default();
    let (unit, index) = index_with_ids(
        r"
        FUNCTION_BLOCK Fb
            METHOD echo: DINT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFb: Fb;
                echoPtr: POINTER TO Fb.echo := ADR(Fb.echo);
            END_VAR

            echoPtr^(instanceFb);
        END_FUNCTION
        ",
        ids.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ids);
    {
        let node = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements[0];
        let AstStatement::CallStatement(CallStatement { operator, .. }) = node.get_stmt() else {
            unreachable!();
        };

        // echoPtr^();
        // ^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(operator), @r#"
        Some(
            FunctionPointer {
                return_type: "DINT",
                qualified_name: "Fb.echo",
            },
        )
        "#);

        // echoPtr^();
        // ^^^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(node), @"None");
    }
}

#[test]
fn function_pointer_method_with_arguments() {
    let ids = IdProvider::default();
    let (unit, index) = index_with_ids(
        r"
        FUNCTION_BLOCK Fb
            METHOD echo: DINT
                VAR_INPUT
                    in: DINT;
                END_VAR

                VAR_OUTPUT
                    out: DINT;
                END_VAR

                VAR_IN_OUT
                    inout: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                echoPtr: POINTER TO Fb.echo := ADR(Fb.echo);
                localIn, localOut, localInOut: DINT;
                instanceFb: Fb;
            END_VAR

            echoPtr^(instanceFb);
            echoPtr^(instanceFb, localIn, localOut, localInOut);
            echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
        END_FUNCTION
        ",
        ids.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, ids);
    let statements = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements;

    // echoPtr^();
    {
        let AstStatement::CallStatement(CallStatement { operator, .. }) = statements[0].get_stmt() else {
            unreachable!();
        };

        // echoPtr^();
        // ^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(operator), @r#"
        Some(
            FunctionPointer {
                return_type: "DINT",
                qualified_name: "Fb.echo",
            },
        )
        "#);

        // echoPtr^();
        // ^^^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(&statements[0]), @"None");
    }

    // echoPtr^(instanceFb, localIn, localOut, localInOut);
    {
        let AstStatement::CallStatement(CallStatement { operator, parameters: Some(parameters) }) =
            statements[1].get_stmt()
        else {
            unreachable!();
        };

        // echoPtr^(instanceFb, localIn, localOut, localInOut);
        // ^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(operator), @r#"
        Some(
            FunctionPointer {
                return_type: "DINT",
                qualified_name: "Fb.echo",
            },
        )
        "#);

        // echoPtr^(instanceFb, localIn, localOut, localInOut);
        //                      ^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(annotations.get(&arguments[1]), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localIn",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        // echoPtr^(instanceFb, localIn, localOut, localInOut);
        //                               ^^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(annotations.get(&arguments[2]), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localOut",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        // echoPtr^(instanceFb, localIn, localOut, localInOut);
        //                                         ^^^^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(annotations.get(&arguments[3]), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localInOut",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);
    }

    // echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
    {
        let AstStatement::CallStatement(CallStatement { operator, parameters: Some(parameters) }) =
            statements[2].get_stmt()
        else {
            unreachable!();
        };

        // echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
        // ^^^^^^^^
        insta::assert_debug_snapshot!(annotations.get(operator), @r#"
        Some(
            FunctionPointer {
                return_type: "DINT",
                qualified_name: "Fb.echo",
            },
        )
        "#);

        // echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
        //                      ^^^^^^^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        let AstStatement::Assignment(Assignment { left, right }) = &arguments[1].stmt else {
            unreachable!();
        };
        insta::assert_debug_snapshot!(annotations.get(&arguments[1]), @"None");
        insta::assert_debug_snapshot!(annotations.get(&left), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "Fb.echo.in",
                constant: false,
                argument_type: ByVal(
                    Input,
                ),
                auto_deref: None,
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get(&right), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localIn",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        // echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
        //                                     ^^^^^^^^^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        let AstStatement::OutputAssignment(Assignment { left, right }) = &arguments[2].stmt else {
            unreachable!();
        };
        insta::assert_debug_snapshot!(annotations.get(&arguments[2]), @"None");
        insta::assert_debug_snapshot!(annotations.get(&left), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "Fb.echo.out",
                constant: false,
                argument_type: ByRef(
                    Output,
                ),
                auto_deref: Some(
                    Default,
                ),
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get(&right), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localOut",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        // echoPtr^(instanceFb, in := localIn, out => localOut, inout := localInOut);
        //                                                      ^^^^^^^^^^^^^^^^^^^
        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        let AstStatement::Assignment(Assignment { left, right }) = &arguments[3].stmt else {
            unreachable!();
        };
        insta::assert_debug_snapshot!(annotations.get(&arguments[3]), @"None");
        insta::assert_debug_snapshot!(annotations.get(&left), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "Fb.echo.inout",
                constant: false,
                argument_type: ByRef(
                    InOut,
                ),
                auto_deref: Some(
                    Default,
                ),
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get(&right), @r#"
        Some(
            Variable {
                resulting_type: "DINT",
                qualified_name: "main.localInOut",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);
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
            FunctionPointer {
                return_type: "VOID",
                qualified_name: "FbA.foo",
            },
        )
        "#);
        insta::assert_debug_snapshot!(annotations.get_hint(&call.operator), @"None");
    }
}

#[test]
fn function_pointer_arguments_have_correct_type_hint() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        r"
        FUNCTION_BLOCK A
            METHOD printArgs
                VAR_INPUT
                    message: STRING;
                    value: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                printArgs: POINTER TO A.printArgs := ADR(A.printArgs);
            END_VAR

            instanceA.printArgs('value =', 5);
            printArgs^(instanceA, 'value =', 5);
        END_FUNCTION
        ",
        id_provider.clone(),
    );

    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);

    // instanceA.printArgs('value =', 5);
    //           ^^^^^^^^^
    {
        let node = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements[0];

        let AstStatement::CallStatement(CallStatement { parameters: Some(parameters), .. }) = &node.stmt
        else {
            unreachable!();
        };

        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(&arguments[0], @r#"
        LiteralString {
            value: "value =",
            is_wide: false,
        }
        "#);

        insta::assert_debug_snapshot!(annotations.get(&arguments[0]), @r#"
        Some(
            Value {
                resulting_type: "__STRING_7",
            },
        )
        "#);

        insta::assert_debug_snapshot!(annotations.get_type(&arguments[0], &index), @r#"
        Some(
            DataType {
                name: "__STRING_7",
                initial_value: None,
                information: String {
                    size: LiteralInteger(
                        8,
                    ),
                    encoding: Utf8,
                },
                nature: String,
                location: SourceLocation {
                    span: None,
                },
            },
        )
        "#);

        insta::assert_debug_snapshot!(annotations.get_type_hint(&arguments[0], &index), @r#"
        Some(
            DataType {
                name: "STRING",
                initial_value: None,
                information: String {
                    size: LiteralInteger(
                        81,
                    ),
                    encoding: Utf8,
                },
                nature: String,
                location: SourceLocation {
                    span: None,
                },
            },
        )
        "#);
    }

    // printArgs^(instanceA, 'value =', 5);
    //                       ^^^^^^^^^
    {
        let node = &unit.implementations.iter().find(|imp| imp.name == "main").unwrap().statements[1];

        let AstStatement::CallStatement(CallStatement { parameters: Some(parameters), .. }) = &node.stmt
        else {
            unreachable!();
        };

        let AstStatement::ExpressionList(arguments) = &parameters.stmt else {
            unreachable!();
        };

        insta::assert_debug_snapshot!(&arguments[1], @r#"
        LiteralString {
            value: "value =",
            is_wide: false,
        }
        "#);

        insta::assert_debug_snapshot!(annotations.get(&arguments[1]), @r#"
        Some(
            Value {
                resulting_type: "__STRING_7",
            },
        )
        "#);

        insta::assert_debug_snapshot!(annotations.get_type(&arguments[1], &index), @r#"
        Some(
            DataType {
                name: "__STRING_7",
                initial_value: None,
                information: String {
                    size: LiteralInteger(
                        8,
                    ),
                    encoding: Utf8,
                },
                nature: String,
                location: SourceLocation {
                    span: None,
                },
            },
        )
        "#);

        insta::assert_debug_snapshot!(annotations.get_type_hint(&arguments[1], &index), @r#"
        Some(
            DataType {
                name: "STRING",
                initial_value: None,
                information: String {
                    size: LiteralInteger(
                        81,
                    ),
                    encoding: Utf8,
                },
                nature: String,
                location: SourceLocation {
                    span: None,
                },
            },
        )
        "#);
    }
}
