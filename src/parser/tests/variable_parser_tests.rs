use plc_ast::ast::{LinkageType, VariableBlock};

use crate::test_utils::tests::parse;

#[test]
fn empty_global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn external_global_vars_can_be_parsed() {
    let src = "@EXTERNAL VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast);
    assert!(matches!(vars, VariableBlock { linkage: LinkageType::External, .. }));
}

#[test]
fn global_single_line_vars_can_be_parsed() {
    let src = "VAR_GLOBAL x, y,z : INT; f : BOOL; b, c : SINT; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars[0]; //globar_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"VariableBlock {
    variables: [
        Variable {
            name: "x",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "y",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "z",
            data_type: DataTypeReference {
                referenced_type: "INT",
            },
        },
        Variable {
            name: "f",
            data_type: DataTypeReference {
                referenced_type: "BOOL",
            },
        },
        Variable {
            name: "b",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
        Variable {
            name: "c",
            data_type: DataTypeReference {
                referenced_type: "SINT",
            },
        },
    ],
    variable_block_type: Global,
}"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn two_global_vars_can_be_parsed() {
    let src = "VAR_GLOBAL a: INT; END_VAR VAR_GLOBAL x : INT; y : BOOL; END_VAR";
    let result = parse(src).0;

    let vars = &result.global_vars; //global_vars
    let ast_string = format!("{vars:#?}");
    let expected_ast = r#"[
    VariableBlock {
        variables: [
            Variable {
                name: "a",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
        ],
        variable_block_type: Global,
    },
    VariableBlock {
        variables: [
            Variable {
                name: "x",
                data_type: DataTypeReference {
                    referenced_type: "INT",
                },
            },
            Variable {
                name: "y",
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
            },
        ],
        variable_block_type: Global,
    },
]"#;
    assert_eq!(ast_string, expected_ast)
}

#[test]
fn global_var_with_address() {
    let src = "VAR_GLOBAL
            a AT %I* : INT;
            b AT %Q* : INT;
            c AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_VAR ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);

    insta::assert_debug_snapshot!(result);
}

#[test]
fn pou_var_with_address() {
    let src = "PROGRAM main
    VAR
            a AT %I* : INT;
            b AT %Q* : INT;
            c,d AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_VAR
    END_PROGRAM
    ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);
    insta::assert_debug_snapshot!(result);
}

#[test]
fn struct_with_address() {
    let src = "TYPE t : STRUCT
            a AT %I* : INT;
            b AT %Q* : INT;
            c AT %M* : INT;
            aa AT %IX7 : INT;
            bb AT %QB5.5 : INT;
            cc AT %MD3.3.3 : INT;
            dd AT %GD4.3.3 : INT;
    END_STRUCT
    END_TYPE
    ";
    let (result, diag) = parse(src);

    assert_eq!(diag, vec![]);
    insta::assert_debug_snapshot!(result);
}

#[test]
fn date_and_time_constants_test() {
    let src = r#"
    VAR_GLOBAL CONSTANT
        cT          : TIME;
        cT_SHORT    : TIME;
        cLT         : LTIME;
        cLT_SHORT   : LTIME;
        cD          : DATE;
        cD_SHORT    : DATE;
        cLD         : LDATE;
        cLD_SHORT   : LDATE;
        cTOD        : TIME_OF_DAY;
        cTOD_SHORT  : TOD;
        cLTOD       : LTOD;
        cLTOD_SHORT : LTOD;
        cDT         : DATE_AND_TIME;
        cDT_SHORT   : DT;
        cLDT        : LDT;
        cLDT_SHORT  : LDT;
    END_VAR"#;

    let (result, diag) = parse(src);
    let vars = &result.global_vars[0]; //globar_vars
    assert_eq!(diag, vec![]);
    insta::assert_snapshot!(format!("{vars:#?}"));
}

#[test]
fn var_config_test() {
    let src = "
    VAR_CONFIG
        instance1.foo.qux AT %IX3.1 : BOOL;
        instance2.bar.qux AT %IX5.6 : BOOL;
    END_VAR
    ";
    let (result, diag) = parse(src);

    assert!(diag.is_empty());
    insta::assert_debug_snapshot!(result, @r#"
    CompilationUnit {
        global_vars: [],
        var_config: [
            ConfigVariable {
                reference: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "qux",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "foo",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instance1",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
                address: HardwareAccess {
                    direction: Input,
                    access: Bit,
                    address: [
                        LiteralInteger {
                            value: 3,
                        },
                        LiteralInteger {
                            value: 1,
                        },
                    ],
                    location: SourceLocation {
                        span: Range(2:26 - 2:35),
                    },
                },
                location: SourceLocation {
                    span: Range(2:8 - 2:25),
                },
            },
            ConfigVariable {
                reference: ReferenceExpr {
                    kind: Member(
                        Identifier {
                            name: "qux",
                        },
                    ),
                    base: Some(
                        ReferenceExpr {
                            kind: Member(
                                Identifier {
                                    name: "bar",
                                },
                            ),
                            base: Some(
                                ReferenceExpr {
                                    kind: Member(
                                        Identifier {
                                            name: "instance2",
                                        },
                                    ),
                                    base: None,
                                },
                            ),
                        },
                    ),
                },
                data_type: DataTypeReference {
                    referenced_type: "BOOL",
                },
                address: HardwareAccess {
                    direction: Input,
                    access: Bit,
                    address: [
                        LiteralInteger {
                            value: 5,
                        },
                        LiteralInteger {
                            value: 6,
                        },
                    ],
                    location: SourceLocation {
                        span: Range(3:26 - 3:35),
                    },
                },
                location: SourceLocation {
                    span: Range(3:8 - 3:25),
                },
            },
        ],
        pous: [],
        implementations: [],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "#);
}

#[test]
fn var_config_location() {
    let src = r#"
    VAR_CONFIG
        main.instance.foo AT %IX3.1 : BOOL;
    END_VAR
    "#;

    let (result, _) = parse(src);

    assert_eq!("main.instance.foo", &src[result.var_config[0].location.to_range().unwrap()]);
}

#[test]
fn var_external() {
    let src = r#"
    VAR_GLOBAL 
        arr: ARRAY [0..100] OF INT; 
    END_VAR

    FUNCTION foo
    VAR_EXTERNAL 
        arr : ARRAY [0..100] OF INT;
    END_VAR
    END_FUNCTION
    "#;

    let (result, _) = parse(src);

    insta::assert_debug_snapshot!(result, @r#"
    CompilationUnit {
        global_vars: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "arr",
                        data_type: DataTypeDefinition {
                            data_type: ArrayType {
                                name: None,
                                bounds: RangeStatement {
                                    start: LiteralInteger {
                                        value: 0,
                                    },
                                    end: LiteralInteger {
                                        value: 100,
                                    },
                                },
                                referenced_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                                is_variable_length: false,
                            },
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ],
        var_config: [],
        pous: [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "arr",
                                data_type: DataTypeDefinition {
                                    data_type: ArrayType {
                                        name: None,
                                        bounds: RangeStatement {
                                            start: LiteralInteger {
                                                value: 0,
                                            },
                                            end: LiteralInteger {
                                                value: 100,
                                            },
                                        },
                                        referenced_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        is_variable_length: false,
                                    },
                                },
                            },
                        ],
                        variable_block_type: External,
                    },
                ],
                pou_type: Function,
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "foo",
                type_name: "foo",
                linkage: Internal,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(9:4 - 8:11),
                },
                name_location: SourceLocation {
                    span: Range(5:13 - 5:16),
                },
                end_location: SourceLocation {
                    span: Range(9:4 - 9:16),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "#);
}

#[test]
fn var_external_constant() {
    let src = r#"
    VAR_GLOBAL 
        arr: ARRAY [0..100] OF INT; 
    END_VAR

    FUNCTION foo
    VAR_EXTERNAL CONSTANT
        arr : ARRAY [0..100] OF INT;
    END_VAR
    END_FUNCTION
    "#;

    let (result, _) = parse(src);

    insta::assert_debug_snapshot!(result, @r#"
    CompilationUnit {
        global_vars: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "arr",
                        data_type: DataTypeDefinition {
                            data_type: ArrayType {
                                name: None,
                                bounds: RangeStatement {
                                    start: LiteralInteger {
                                        value: 0,
                                    },
                                    end: LiteralInteger {
                                        value: 100,
                                    },
                                },
                                referenced_type: DataTypeReference {
                                    referenced_type: "INT",
                                },
                                is_variable_length: false,
                            },
                        },
                    },
                ],
                variable_block_type: Global,
            },
        ],
        var_config: [],
        pous: [
            POU {
                name: "foo",
                variable_blocks: [
                    VariableBlock {
                        variables: [
                            Variable {
                                name: "arr",
                                data_type: DataTypeDefinition {
                                    data_type: ArrayType {
                                        name: None,
                                        bounds: RangeStatement {
                                            start: LiteralInteger {
                                                value: 0,
                                            },
                                            end: LiteralInteger {
                                                value: 100,
                                            },
                                        },
                                        referenced_type: DataTypeReference {
                                            referenced_type: "INT",
                                        },
                                        is_variable_length: false,
                                    },
                                },
                            },
                        ],
                        variable_block_type: External,
                    },
                ],
                pou_type: Function,
                return_type: None,
                interfaces: [],
                properties: [],
            },
        ],
        implementations: [
            Implementation {
                name: "foo",
                type_name: "foo",
                linkage: Internal,
                pou_type: Function,
                statements: [],
                location: SourceLocation {
                    span: Range(9:4 - 8:11),
                },
                name_location: SourceLocation {
                    span: Range(5:13 - 5:16),
                },
                end_location: SourceLocation {
                    span: Range(9:4 - 9:16),
                },
                overriding: false,
                generic: false,
                access: None,
            },
        ],
        interfaces: [],
        user_types: [],
        file: File(
            "test.st",
        ),
    }
    "#);
}

#[test]
fn function_pointer() {
    let src = r#"
        TYPE Collection:
            STRUCT
                body:   __FPOINTER Fb;
                foo:    __FPOINTER Fb.foo;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK Fb
            METHOD foo
            END_METHOD

            METHOD bar: DINT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                bar: __FPOINTER Fb.bar;
            END_VAR
        END_FUNCTION
    "#;

    let (result, _) = parse(src);
    insta::assert_debug_snapshot!(result.user_types[0], @r#"
    UserTypeDeclaration {
        data_type: StructType {
            name: Some(
                "Collection",
            ),
            variables: [
                Variable {
                    name: "body",
                    data_type: DataTypeDefinition {
                        data_type: PointerType {
                            name: None,
                            referenced_type: DataTypeReference {
                                referenced_type: "Fb",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: true,
                        },
                    },
                },
                Variable {
                    name: "foo",
                    data_type: DataTypeDefinition {
                        data_type: PointerType {
                            name: None,
                            referenced_type: DataTypeReference {
                                referenced_type: "Fb.foo",
                            },
                            auto_deref: None,
                            type_safe: false,
                            is_function: true,
                        },
                    },
                },
            ],
        },
        initializer: None,
        scope: None,
    }
    "#);
    insta::assert_debug_snapshot!(result.pous.iter().find(|pou| pou.name == "main").unwrap(), @r#"
    POU {
        name: "main",
        variable_blocks: [
            VariableBlock {
                variables: [
                    Variable {
                        name: "bar",
                        data_type: DataTypeDefinition {
                            data_type: PointerType {
                                name: None,
                                referenced_type: DataTypeReference {
                                    referenced_type: "Fb.bar",
                                },
                                auto_deref: None,
                                type_safe: false,
                                is_function: true,
                            },
                        },
                    },
                ],
                variable_block_type: Local,
            },
        ],
        pou_type: Function,
        return_type: None,
        interfaces: [],
        properties: [],
    }
    "#);
}
