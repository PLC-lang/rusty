mod units_tests {
    use insta::assert_debug_snapshot;
    use plc_driver::parse_and_annotate;
    use plc_source::SourceCode;

    #[test]
    fn after_parsing_a_function_block_contains_ref_to_its_base() {
        let src: SourceCode = "
        FUNCTION_BLOCK foo
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().pous[1];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "bar",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__foo",
                            data_type: DataTypeReference {
                                referenced_type: "foo",
                            },
                        },
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: FunctionBlock,
            return_type: None,
            interfaces: [],
            properties: [],
        }
        "#);
    }

    #[test]
    fn write_to_parent_variable_qualified_access() {
        let src: SourceCode = "
            FUNCTION_BLOCK fb
            VAR
                x : INT;
                y : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK foo
            VAR
                myFb : fb2;
                x : INT;
            END_VAR
                myFb.x := 1; //myFb.__SUPER.x := 1;
                x := 2; // this should not have any bases added
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "foo",
            type_name: "foo",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
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
                                        name: "__fb",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "myFb",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 1,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "x",
                            },
                        ),
                        base: None,
                    },
                    right: LiteralInteger {
                        value: 2,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(16:16 - 17:23),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(11:27 - 11:30),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(18:12 - 18:30),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn write_to_parent_variable_in_instance() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK foo
            VAR
                s : STRING;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK bar EXTENDS foo
                s := 'world';
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[1];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "bar",
            type_name: "bar",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "s",
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
                    right: LiteralString {
                        value: "world",
                        is_wide: false,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(8:16 - 8:29),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(7:27 - 7:30),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(9:12 - 9:30),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn write_to_grandparent_variable_in_initializer() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                z : INT := 42;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent EXTENDS grandparent
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                z := 420;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        // let unit = &project.units[0].get_unit();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "z",
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
                        value: 420,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(11:16 - 11:25),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(10:27 - 10:32),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(12:12 - 12:30),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn test_array_access_in_nested_function_blocks_with_base_references() {
        let src: SourceCode = r#"
                FUNCTION_BLOCK grandparent
                VAR
                    y : ARRAY[0..5] OF INT;
                    a : INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK parent extends grandparent
                    VAR
                        x : ARRAY[0..10] OF INT;
                        b : INT;
                    END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK child EXTENDS parent
                    VAR
                        z : ARRAY[0..10] OF INT;
                    END_VAR
                    x[0] := 42; //__SUPER.x[0] := 42;
                    y[2]:= 5; //__SUPER.__SUPER.y[2] := 5;
                    z[3] := x[1] + y[2]; //z[3] := __SUPER.x[1] + __SUPER.__SUPER.y[2];
                    x[a] := 5; //__SUPER.x[__SUPER__.BASE__.a] := 5;
                    y[b] := 6; //__SUPER.__SUPER.y[__SUPER.b] := 6;
                    z[a+b] := 10; //z[__SUPER.__SUPER.a + __SUPER.b] := 10;
                END_FUNCTION_BLOCK
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 0,
                            },
                        ),
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
                        value: 42,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 2,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
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
                        value: 5,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 3,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: BinaryExpression {
                        operator: Plus,
                        left: ReferenceExpr {
                            kind: Index(
                                LiteralInteger {
                                    value: 1,
                                },
                            ),
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
                            kind: Index(
                                LiteralInteger {
                                    value: 2,
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "y",
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
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "a",
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
                        value: 5,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "b",
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
                                        name: "y",
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
                        value: 6,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            BinaryExpression {
                                operator: Plus,
                                left: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
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
                                right: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "b",
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
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(19:20 - 24:33),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(15:31 - 15:36),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(25:16 - 25:34),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn test_multi_level_reference_handling() {
        let src: SourceCode = "
            FUNCTION_BLOCK fb
            VAR
                x : INT;
                y : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK baz
            VAR
                myFb : fb2;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK foo EXTENDS baz
            VAR
                x : INT;
            END_VAR
                myFb.x := 1;
                // __SUPER.myFb.__SUPER.x
            END_FUNCTION_BLOCK
        "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "foo",
            type_name: "foo",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
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
                                        name: "__fb",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "myFb",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "__baz",
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
                        value: 1,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(21:16 - 21:28),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(17:27 - 17:30),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(23:12 - 23:30),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn test_array_of_objects() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                y : ARRAY[0..5] OF INT;
                a : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent extends grandparent
                VAR
                    x : ARRAY[0..10] OF INT;
                    b : INT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                VAR
                    z : ARRAY[0..10] OF INT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION main
            VAR
                arr: ARRAY[0..10] OF child;
            END_VAR
                arr[0].a := 10;
                arr[0].y[0] := 20;
                arr[1].b := 30;
                arr[1].x[1] := 40;
                arr[2].z[2] := 50;
            END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "main",
            type_name: "main",
            linkage: Internal,
            pou_type: Function,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "a",
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
                                        base: Some(
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
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 0,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
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
                                                base: Some(
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
                                            },
                                        ),
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
                        kind: Member(
                            Identifier {
                                name: "b",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__parent",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Index(
                                            LiteralInteger {
                                                value: 1,
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
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 30,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 1,
                            },
                        ),
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
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Index(
                                                    LiteralInteger {
                                                        value: 1,
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
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 40,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            LiteralInteger {
                                value: 2,
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "z",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Index(
                                            LiteralInteger {
                                                value: 2,
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
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 50,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(25:16 - 29:34),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(21:21 - 21:25),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(30:12 - 30:24),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#)
    }

    #[test]
    fn test_complex_array_access() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR
                y : ARRAY[0..5] OF INT;
                a : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent extends grandparent
                VAR
                    x : ARRAY[0..10] OF INT;
                    b : INT;
                END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
                VAR
                    z : ARRAY[0..10] OF INT;
                END_VAR
                y[b + z[b*2] - a] := 20;
            END_FUNCTION_BLOCK
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "child",
            type_name: "child",
            linkage: Internal,
            pou_type: FunctionBlock,
            statements: [
                Assignment {
                    left: ReferenceExpr {
                        kind: Index(
                            BinaryExpression {
                                operator: Minus,
                                left: BinaryExpression {
                                    operator: Plus,
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "b",
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
                                        kind: Index(
                                            BinaryExpression {
                                                operator: Multiplication,
                                                left: ReferenceExpr {
                                                    kind: Member(
                                                        Identifier {
                                                            name: "b",
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
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "z",
                                                    },
                                                ),
                                                base: None,
                                            },
                                        ),
                                    },
                                },
                                right: ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
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
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "y",
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
                        value: 20,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(19:16 - 19:40),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(15:27 - 15:32),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(20:12 - 20:30),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }

    #[test]
    fn pointer_deref_in_grandparent() {
        let src: SourceCode = r#"
                FUNCTION_BLOCK grandparent
                VAR
                    a : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK parent extends grandparent
                VAR
                    b : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION_BLOCK child EXTENDS parent
                VAR
                    c : REF_TO INT;
                END_VAR
                END_FUNCTION_BLOCK

                FUNCTION main
                VAR
                    fb: child;
                END_VAR
                    fb.c^ := 10;
                    fb.b^ := 20;
                    fb.a^ := 30;
                END_FUNCTION
            "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "main",
            type_name: "main",
            linkage: Internal,
            pou_type: Function,
            statements: [
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__init_child",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "fb",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__user_init_child",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "fb",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "c",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "fb",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    right: LiteralInteger {
                        value: 10,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "b",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__parent",
                                            },
                                        ),
                                        base: Some(
                                            ReferenceExpr {
                                                kind: Member(
                                                    Identifier {
                                                        name: "fb",
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
                        value: 20,
                    },
                },
                Assignment {
                    left: ReferenceExpr {
                        kind: Deref,
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "a",
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
                                                base: Some(
                                                    ReferenceExpr {
                                                        kind: Member(
                                                            Identifier {
                                                                name: "fb",
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
                        ),
                    },
                    right: LiteralInteger {
                        value: 30,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(23:20 - 25:32),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(19:25 - 19:29),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(26:16 - 26:28),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#)
    }

    #[test]
    fn base_type_in_initializer() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK grandparent
            VAR CONSTANT
                a : DINT := 3;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK parent extends grandparent
            VAR
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK child EXTENDS parent
            VAR
                b : DINT := a;
            END_VAR
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().pous[2];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "child",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "__parent",
                            data_type: DataTypeReference {
                                referenced_type: "parent",
                            },
                        },
                    ],
                    variable_block_type: Local,
                },
                VariableBlock {
                    variables: [
                        Variable {
                            name: "b",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                            initializer: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
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
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: FunctionBlock,
            return_type: None,
            interfaces: [],
            properties: [],
        }
        "#);
    }

    #[test]
    fn base_type_in_method_var_initializer() {
        let src: SourceCode = r#"
    FUNCTION_BLOCK grandparent
    VAR CONSTANT
        a : DINT := 3;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK parent extends grandparent
    VAR
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK child EXTENDS parent
        METHOD foo
        VAR
            b : DINT := a;
        END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
"#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().pous[3];
        assert_debug_snapshot!(unit, @r#"
        POU {
            name: "child.foo",
            variable_blocks: [
                VariableBlock {
                    variables: [
                        Variable {
                            name: "b",
                            data_type: DataTypeReference {
                                referenced_type: "DINT",
                            },
                            initializer: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "a",
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
                    ],
                    variable_block_type: Local,
                },
            ],
            pou_type: Method {
                parent: "child",
                property: None,
                declaration_kind: Concrete,
            },
            return_type: None,
            interfaces: [],
            properties: [],
        }
        "#);
    }

    #[test]
    fn assigning_to_base_type_in_method() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK foo
        VAR
            x : DINT := 50;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        METHOD set0 // TODO(volsa): https://github.com/PLC-lang/rusty/issues/1408
            x := 25;
        END_METHOD
        END_FUNCTION_BLOCK
        "#
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[1];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "bar.set0",
            type_name: "bar.set0",
            linkage: Internal,
            pou_type: Method {
                parent: "bar",
                property: None,
                declaration_kind: Concrete,
            },
            statements: [
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
                        value: 25,
                    },
                },
            ],
            location: SourceLocation {
                span: Range(9:12 - 9:20),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(8:15 - 8:19),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(10:8 - 10:18),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: Some(
                Protected,
            ),
        }
        "#);
    }

    #[test]
    fn fb_extension_with_output() {
        let src: SourceCode = r#"
        FUNCTION_BLOCK foo
            METHOD met1 : INT
            VAR_INPUT
            mandatoryInput : INT;
            optionalInput : INT := 5;
            END_VAR
            VAR_OUTPUT
            outputValue : INT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK foo2 EXTENDS foo
            met1(
                mandatoryInput := 0,
                optionalInput := 0,
                outputValue =>
            );
        END_FUNCTION_BLOCK"#
            .into();
        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[2];

        let statements = unit.statements.iter().map(|node| node.as_string()).collect::<Vec<_>>();
        assert_debug_snapshot!(statements, @r#"
        [
            "__vtable_foo2#(THIS^.__foo.__vtable^).met1^(foo#(THIS^), mandatoryInput := 0, optionalInput := 0, outputValue => )",
        ]
        "#);
    }

    #[test]
    fn function_with_output_used_in_main_by_extension() {
        let src: SourceCode = "
    FUNCTION_BLOCK foo
    METHOD met1 : INT
        VAR_INPUT
        mandatoryInput : INT;
        optionalInput : INT := 5;
        END_VAR
        VAR_OUTPUT
        outputValue : INT;
        END_VAR
        outputValue := mandatoryInput + optionalInput;
    END_METHOD
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK foo2 EXTENDS foo
    VAR
        x : INT;
    END_VAR
    met1(
        mandatoryInput := 0,
        optionalInput := 0,
        outputValue => x
    );
    met1(
        mandatoryInput := 5,
        outputValue => x
    );
    END_FUNCTION_BLOCK

    FUNCTION main : DINT
    VAR
        foo_inst: foo;
        foo2_inst : foo2;
        out : INT;
    END_VAR
    foo2_inst();

    foo_inst.met1(mandatoryInput:= 2, outputValue => out);
    foo2_inst.met1(mandatoryInput:= 1, outputValue => out);


    END_FUNCTION

    "
        .into();

        let (_, project) = parse_and_annotate("test", vec![src]).unwrap();
        let unit = &project.units[0].get_unit().implementations[3];
        assert_debug_snapshot!(unit, @r#"
        Implementation {
            name: "main",
            type_name: "main",
            linkage: Internal,
            pou_type: Function,
            statements: [
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__init_foo",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo_inst",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__init_foo2",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo2_inst",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__user_init_foo",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo_inst",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__user_init_foo2",
                            },
                        ),
                        base: None,
                    },
                    parameters: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo2_inst",
                                },
                            ),
                            base: None,
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "foo2_inst",
                            },
                        ),
                        base: None,
                    },
                    parameters: None,
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "met1",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "foo_inst",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                    parameters: Some(
                        ExpressionList {
                            expressions: [
                                Assignment {
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "mandatoryInput",
                                            },
                                        ),
                                        base: None,
                                    },
                                    right: LiteralInteger {
                                        value: 2,
                                    },
                                },
                                OutputAssignment {
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "outputValue",
                                            },
                                        ),
                                        base: None,
                                    },
                                    right: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "out",
                                            },
                                        ),
                                        base: None,
                                    },
                                },
                            ],
                        },
                    ),
                },
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "met1",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__foo",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "foo2_inst",
                                            },
                                        ),
                                        base: None,
                                    },
                                ),
                            },
                        ),
                    },
                    parameters: Some(
                        ExpressionList {
                            expressions: [
                                Assignment {
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "mandatoryInput",
                                            },
                                        ),
                                        base: None,
                                    },
                                    right: LiteralInteger {
                                        value: 1,
                                    },
                                },
                                OutputAssignment {
                                    left: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "outputValue",
                                            },
                                        ),
                                        base: None,
                                    },
                                    right: ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "out",
                                            },
                                        ),
                                        base: None,
                                    },
                                },
                            ],
                        },
                    ),
                },
            ],
            location: SourceLocation {
                span: Range(35:4 - 38:59),
                file: Some(
                    "<internal>",
                ),
            },
            name_location: SourceLocation {
                span: Range(29:13 - 29:17),
                file: Some(
                    "<internal>",
                ),
            },
            end_location: SourceLocation {
                span: Range(41:4 - 41:16),
                file: Some(
                    "<internal>",
                ),
            },
            overriding: false,
            generic: false,
            access: None,
        }
        "#);
    }
}

mod resolve_bases_tests {
    use std::ops::Deref;

    use insta::assert_debug_snapshot;
    use plc::resolver::AnnotationMap;
    use plc_ast::{
        ast::{Assignment, ReferenceExpr},
        try_from,
    };
    use plc_driver::{parse_and_annotate, pipelines::AnnotatedProject};
    use plc_source::SourceCode;

    #[test]
    fn base_types_resolved() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            VAR
                x : INT;
                y : INT;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK baz
            VAR
                myFb : fb2;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK foo EXTENDS baz
            VAR
                x : INT;
            END_VAR
                myFb.x := 1;
            END_FUNCTION_BLOCK
            "#
        .into();

        let (_, AnnotatedProject { units, index: _index, annotations }) =
            parse_and_annotate("test", vec![src]).unwrap();
        let unit = &units[0].get_unit().implementations[3];
        let statement = &unit.statements[0];
        let Some(Assignment { left, .. }) = try_from!(statement, Assignment) else { unreachable!() };
        assert_debug_snapshot!(annotations.get(left), @r#"
        Some(
            Variable {
                resulting_type: "INT",
                qualified_name: "fb.x",
                constant: false,
                argument_type: ByVal(
                    Local,
                ),
                auto_deref: None,
            },
        )
        "#);

        let Some(ReferenceExpr { base, .. }) = try_from!(left, ReferenceExpr) else { unreachable!() };
        let base1 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base1).unwrap(), @r#"
        Variable {
            resulting_type: "fb",
            qualified_name: "fb2.__fb",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);

        let Some(ReferenceExpr { base, .. }) = try_from!(base1, ReferenceExpr) else { unreachable!() };
        let base2 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base2).unwrap(), @r#"
        Variable {
            resulting_type: "fb2",
            qualified_name: "baz.myFb",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);

        let Some(ReferenceExpr { base, .. }) = try_from!(base2, ReferenceExpr) else { unreachable!() };
        let base3 = base.as_ref().unwrap().deref();
        assert_debug_snapshot!(annotations.get(base3).unwrap(), @r#"
        Variable {
            resulting_type: "baz",
            qualified_name: "foo.__baz",
            constant: false,
            argument_type: ByVal(
                Local,
            ),
            auto_deref: None,
        }
        "#);
    }
}

mod inherited_properties {
    use insta::assert_debug_snapshot;
    use plc_driver::{parse_and_annotate, pipelines::AnnotatedProject};
    use plc_source::SourceCode;

    #[test]
    fn reference_to_property_declared_in_parent_is_called_correctly() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
                foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[1];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r#"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__get_foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__fb",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: None,
        }
        "#);
    }

    #[test]
    fn reference_to_property_declared_in_grandparent_is_called_correctly() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb3 EXTENDS fb2
                foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[2];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r#"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__get_foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__fb",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__fb2",
                                    },
                                ),
                                base: None,
                            },
                        ),
                    },
                ),
            },
            parameters: None,
        }
        "#);
    }

    #[test]
    fn extended_prop() {
        let src: SourceCode = r#"
            FUNCTION_BLOCK fb
            PROPERTY foo : INT
                GET END_GET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb2 EXTENDS fb
            PROPERTY FOO : INT
                SET END_SET
            END_PROPERTY
            END_FUNCTION_BLOCK

            FUNCTION_BLOCK fb3 EXTENDS fb2
                // we expect the RHS to call the getter defined in the grandparent and
                // pass the result to the setter call in the grandparent
                foo := foo;
            END_FUNCTION_BLOCK
        "#
        .into();

        let (_, AnnotatedProject { units, .. }) = parse_and_annotate("test", vec![src]).unwrap();
        let implementation = &units[0].get_unit().implementations[2];
        let stmt = &implementation.statements[0];
        assert_debug_snapshot!(stmt, @r#"
        CallStatement {
            operator: ReferenceExpr {
                kind: Member(
                    Identifier {
                        name: "__set_foo",
                    },
                ),
                base: Some(
                    ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__fb2",
                            },
                        ),
                        base: None,
                    },
                ),
            },
            parameters: Some(
                CallStatement {
                    operator: ReferenceExpr {
                        kind: Member(
                            Identifier {
                                name: "__get_foo",
                            },
                        ),
                        base: Some(
                            ReferenceExpr {
                                kind: Member(
                                    Identifier {
                                        name: "__fb",
                                    },
                                ),
                                base: Some(
                                    ReferenceExpr {
                                        kind: Member(
                                            Identifier {
                                                name: "__fb2",
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
            ),
        }
        "#);
    }
}
