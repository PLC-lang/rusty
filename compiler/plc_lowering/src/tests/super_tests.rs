use insta::assert_debug_snapshot;
use plc_driver::parse_and_annotate;
use plc_source::SourceCode;

#[test]
fn super_qualified_reference_resolves_to_same_parent_var() {
    let src: SourceCode = "
        FUNCTION_BLOCK foo
            VAR
                x: DINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            x := 3;
            super^.x := 3;
        END_FUNCTION_BLOCK
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__foo",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 3,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__foo",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 3,
            },
        },
    ]
    "#);
}

#[test]
fn super_without_deref_lowered_to_ref_call_to_parent_instance() {
    let src: SourceCode = "
        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            super;
        END_FUNCTION_BLOCK
        "
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "REF",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "__foo",
                        },
                    ),
                    base: None,
                },
            ),
        },
    ]
    "#);
}

#[test]
fn super_expression_as_function_argument() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Use SUPER expression for function calls
            foo(SUPER, SUPER^);
        END_FUNCTION_BLOCK

        FUNCTION foo : INT
        VAR_INPUT
            input_ref : REF_TO parent;
            input : parent;
        END_VAR
        END_FUNCTION
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "foo",
                    },
                ),
                base: None,
            },
            parameters: Some(
                ExpressionList {
                    expressions: [
                        CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "REF",
                                    },
                                ),
                                base: None,
                            },
                            parameters: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ],
                },
            ),
        },
    ]
    "#);
}

#[test]
fn access_grandparent_through_super() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                x : INT := 10;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                // Access grandparent member through SUPER^
                SUPER^.x := 200;
            END_FUNCTION_BLOCK
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[2].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__grandparent",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 200,
            },
        },
    ]
    "#);
}

#[test]
fn access_great_grandparent_through_super() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK great_grandparent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK grandparent EXTENDS great_grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Access great_grandparent member through SUPER^
            SUPER^.x := 100;
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[3].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__great_grandparent",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__grandparent",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 100,
            },
        },
    ]
    "#);
}

#[test]
fn super_keyword_in_method_call() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
                METHOD base_method : INT
                    base_method := x;
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                METHOD test
                    // Call method on parent through SUPER^
                    SUPER^.base_method();
                END_METHOD
            END_FUNCTION_BLOCK
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[2].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "base_method",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: None,
        },
    ]
    "#);
}

#[test]
fn chained_super_keywords() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                x : INT := 10;
                y : INT := 20;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                // Chained SUPER access (technically invalid but we should handle it gracefully)
                SUPER^.SUPER^.x := SUPER^.SUPER^.y;
            END_FUNCTION_BLOCK
        "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[2].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super(derefed),
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super(derefed),
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn super_in_array_access_edge_cases() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            arr : ARRAY[0..5] OF INT := [1,2,3,4,5,6];
            idx : INT := 2;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            arr : ARRAY[0..5] OF INT := [10,20,30,40,50,60];
        END_VAR
            // Access parent array with SUPER^ using parent's index
            SUPER^.arr[SUPER^.idx] := 100;
            
            // Access parent array with SUPER^ using child's array element
            SUPER^.arr[arr[0]] := 200;
            
            // Access child array using parent's index
            arr[SUPER^.idx] := 300;
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Index(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "idx",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "arr",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 100,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Index(
                    ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 0,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "arr",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "arr",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 200,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Index(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "idx",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "arr",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 300,
            },
        },
    ]
    "#);
}

#[test]
fn super_in_complex_expressions() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
            y : INT := 20;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            z : INT := 30;
        END_VAR
            // Use SUPER^ in complex arithmetic expressions
            z := SUPER^.x + SUPER^.y * 2;
            
            // Use SUPER^ in condition
            IF SUPER^.x > SUPER^.y THEN
                z := 100;
            END_IF;
            
            // Use SUPER^ in mixed expression with child variables
            SUPER^.x := z - SUPER^.y;
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "z",
                    },
                ),
                base: None,
            },
            right: BinaryExpression {
                operator: Plus,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                right: BinaryExpression {
                    operator: Multiplication,
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "y",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 2,
                    },
                },
            },
        },
        IfStatement {
            blocks: [
                ConditionalBlock {
                    condition: BinaryExpression {
                        operator: Greater,
                        left: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "x",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                        right: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "y",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    },
                    body: [
                        Assignment {
                            left: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                            right: LiteralInteger {
                                value: 100,
                            },
                        },
                    ],
                },
            ],
            else_block: [],
        },
        EmptyStatement,
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: BinaryExpression {
                operator: Minus,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "z",
                        },
                    ),
                    base: None,
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        },
    ]
    "#);
}

#[test]
fn super_with_mixed_deref_patterns() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : REF_TO INT;
            y : INT := 20;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            // Mix of SUPER^ with deref operator ^
            SUPER^.x^ := 100;
            
            // Multiple ^ operators in sequence
            SUPER^.x^ := SUPER^.y;
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[1].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 100,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn super_to_access_overridden_methods() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
            METHOD calculate : INT
                calculate := x * 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            y : INT := 20;
        END_VAR
            METHOD calculate : INT // Override parent's method
                calculate := x + y;
            END_METHOD

            METHOD test : INT
                // Call parent's version of the overridden method
                test := SUPER^.calculate();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[3].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "test",
                    },
                ),
                base: None,
            },
            right: CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "calculate",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
        },
    ]
    "#);
}

#[test]
fn super_in_complex_expressions_with_method_calls() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
            y : INT := 20;
        END_VAR
            METHOD get_x : INT
                get_x := x;
            END_METHOD
            
            METHOD get_y : INT
                get_y := y;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            z : INT := 30;
        END_VAR
            METHOD get_x : INT // Override parent's method
                get_x := x * 2;
            END_METHOD
            
            METHOD test : INT
                // Use parent's methods in an expression
                test := SUPER^.get_x() + SUPER^.get_y();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[4].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "test",
                    },
                ),
                base: None,
            },
            right: BinaryExpression {
                operator: Plus,
                left: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "get_x",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: None,
                },
                right: CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "get_y",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: None,
                },
            },
        },
    ]
    "#);
}

#[test]
fn super_access_with_interface_methods() {
    let src: SourceCode = r#"
        INTERFACE ICounter
            METHOD increment : INT END_METHOD
        END_INTERFACE

        FUNCTION_BLOCK parent IMPLEMENTS ICounter
        VAR
            count : INT := 0;
        END_VAR
            METHOD increment : INT
                count := count + 1;
                increment := count;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD increment : INT // Override the interface method
                count := count + 10;
                increment := count;
            END_METHOD
            
            METHOD double_increment : INT
                // Call parent's implementation of the interface method
                SUPER^.increment();
                // Call our own implementation
                increment();
                double_increment := count;
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[3].statements;
    let statements_str = statements.iter().map(|statement| statement.as_string()).collect::<Vec<_>>();

    assert_debug_snapshot!(statements_str, @r#"
    [
        "__parent.increment()",
        "__vtable_child#(THIS^.__parent.__vtable^).increment^(child#(THIS^))",
        "double_increment := __parent.count",
    ]
    "#);
}

#[test]
fn super_with_multiple_overridden_methods_in_hierarchy() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK grandparent
            METHOD process : INT
                process := 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
            METHOD process : INT // Override grandparent method
                process := 2;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            METHOD process : INT // Override parent method
                process := 3;
            END_METHOD
            
            METHOD test : INT
                // Call parent's version of the method
                test := SUPER^.process();
                
                // We cannot access grandparent's version directly with SUPER^.SUPER^.process()
                // as chaining SUPER is not allowed, we'll still check if we handle it gracefully
                test := SUPER^.SUPER^.process();
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[5].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "test",
                    },
                ),
                base: None,
            },
            right: CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "process",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                parameters: None,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "test",
                    },
                ),
                base: None,
            },
            right: CallStatement {
                operator: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "process",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Super(derefed),
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                parameters: None,
            },
        },
    ]
    "#);
}

#[test]
fn super_in_constructor() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
            METHOD init
                x := 100;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            y : INT := 20;
        END_VAR
            METHOD init
                // Call parent's init method
                SUPER^.init();
                y := 200;
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    .into();

    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let statements = &project.units[0].get_unit().implementations[2].statements;
    assert_debug_snapshot!(statements, @r#"
    [
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "init",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: None,
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "y",
                    },
                ),
                base: None,
            },
            right: LiteralInteger {
                value: 200,
            },
        },
    ]
    "#);
}

#[test]
fn super_in_initializer() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
            VAR
                y : INT := SUPER^.x + 5;
            END_VAR
            END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let initializer = &project.units[0].get_unit().pous[1].variable_blocks[1].variables[0].initializer;
    assert_debug_snapshot!(initializer, @r#"
    Some(
        BinaryExpression {
            operator: Plus,
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 5,
            },
        },
    )
    "#);
}

#[test]
fn super_in_method_initializer() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                METHOD test
                VAR
                    y : INT := SUPER^.x + 5;
                END_VAR
                END_METHOD
            END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let initializer = &project.units[0].get_unit().pous[2].variable_blocks[0].variables[0].initializer;
    assert_debug_snapshot!(initializer, @r#"
    Some(
        BinaryExpression {
            operator: Plus,
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 5,
            },
        },
    )
    "#);
}

#[test]
fn super_method_called_in_initializer() {
    let src: SourceCode = r#"
            FUNCTION_BLOCK parent
            VAR
                x : INT := 10;
            END_VAR
            METHOD init : INT
                init := x + 1;
            END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                VAR
                    y : INT := SUPER^.init();
                END_VAR
            END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let initializer = &project.units[0].get_unit().pous[2].variable_blocks[1].variables[0].initializer;
    assert_debug_snapshot!(initializer, @r#"
    Some(
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "init",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: None,
        },
    )
    "#);
}

#[test]
fn super_ref_in_reference_access() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            p : parent;
        END_VAR
                // We don't want `SUPER` to be lowered to `REF(p)` here,
                // since it will lead to incomprehensible error messages later on
                // (.e.g. `p.REF(p).x` or p.REF(p)^.x)
                p.SUPER.x := 40;
        END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super,
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "p",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 40,
            },
        },
    ]
    "#);
}

#[test]
fn super_in_paren_expressions() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            y : INT;
        END_VAR
            // While these are errors that will be caught during validation,
            // we want to ensure we handle it gracefully
            
            // Multiple dereferencing of SUPER
            (SUPER^)^.x := 20;                
            (SUPER^)^ := 30;
            
            // Invalid chain with wrong syntax
            (SUPER^).SUPER.x := 40;
        END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ParenExpression {
                                expression: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 20,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ParenExpression {
                        expression: ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    },
                ),
            },
            right: LiteralInteger {
                value: 30,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super,
                        ),
                        base: Some(
                            ParenExpression {
                                expression: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 40,
            },
        },
    ]
    "#);
}

#[test]
fn valid_super_deref_in_paren_expression_edge_case() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
            y : INT := 20;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            z : INT;
            local: parent;
        END_VAR
            // Valid deref in parentheses
            z := (SUPER)^.x + y;
            local := (SUPER)^;
        END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "z",
                    },
                ),
                base: None,
            },
            right: BinaryExpression {
                operator: Plus,
                left: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "x",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Deref,
                            base: Some(
                                ParenExpression {
                                    expression: CallStatement {
                                        operator: ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "REF",
                                                },
                                            ),
                                            base: None,
                                        },
                                        parameters: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__parent",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                },
                            ),
                        },
                    ),
                },
                right: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "y",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "__parent",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "local",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Deref,
                base: Some(
                    ParenExpression {
                        expression: CallStatement {
                            operator: ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "REF",
                                    },
                                ),
                                base: None,
                            },
                            parameters: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "__parent",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    },
                ),
            },
        },
    ]
    "#);
}

#[test]
fn pointer_arithmetic_with_super() {
    let src: SourceCode = r#"
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
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "a",
                    },
                ),
                base: None,
            },
            right: BinaryExpression {
                operator: Plus,
                left: ReferenceExpr {
                    kind: Deref,
                    base: Some(
                        ParenExpression {
                            expression: BinaryExpression {
                                operator: Plus,
                                left: CallStatement {
                                    operator: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "REF",
                                            },
                                        ),
                                        base: None,
                                    },
                                    parameters: Some(
                                        ReferenceExpr {
                                            kind: Member(
                                                Identifier {
                                                    name: "__parent",
                                                },
                                            ),
                                            base: None,
                                        },
                                    ),
                                },
                                right: LiteralInteger {
                                    value: 1,
                                },
                            },
                        },
                    ),
                },
                right: LiteralInteger {
                    value: 5,
                },
            },
        },
    ]
    "#);
}

#[test]
fn global_accessor() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        VAR_GLOBAL
            p: parent;
        END_VAR

        FUNCTION_BLOCK child EXTENDS parent
            // the following statements are invalid but should be handled gracefully
            // accessing SUPER with global namespace operator
            .SUPER^.x := 0;
            // valid global access but invalid use of `SUPER` outside its POU/in non-extended POU
            .p.SUPER^.x := 0;
        END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Global(
                            Super(derefed),
                        ),
                        base: None,
                    },
                ),
            },
            right: LiteralInteger {
                value: 0,
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Super(derefed),
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Global(
                                    Identifier {
                                        name: "p",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            right: LiteralInteger {
                value: 0,
            },
        },
    ]
    "#);
}

#[test]
fn cast_statement() {
    let src: SourceCode = r#"
        FUNCTION_BLOCK parent
        VAR
            x : INT := 10;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        VAR
            p: parent;
        END_VAR
            // these are all invalid, but should be handled gracefully
            p := parent#SUPER^;
            p := parent#SUPER;
            p := parent#SUPER^.x;
            p := parent#SUPER.x;
        END_FUNCTION_BLOCK
        "#
    .into();
    let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
    let implementation = &project.units[0].get_unit().implementations[1];
    let statements = &implementation.statements;
    assert_debug_snapshot!(statements, @r#"
    [
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "p",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Cast(
                    Super(derefed),
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "p",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Cast(
                    Super,
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "parent",
                            },
                        ),
                        base: None,
                    },
                ),
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "p",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Cast(
                            Super(derefed),
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
        },
        Assignment {
            left: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "p",
                    },
                ),
                base: None,
            },
            right: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "x",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Cast(
                            Super,
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "parent",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
        },
    ]
    "#);
}
