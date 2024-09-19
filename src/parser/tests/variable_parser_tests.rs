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

    insta::assert_snapshot!(format!("{result:?}"));
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

    insta::assert_snapshot!(format!("{result:?}"));
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
    insta::assert_snapshot!(format!("{result:?}"));
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
        instance1.foo AT %IX3.1 : BOOL;
        instance2.bar AT %IX5.6 : BOOL;
    END_VAR
    ";
    let (result, diag) = parse(src);

    assert!(diag.is_empty());
    insta::assert_debug_snapshot!(result, @r###"
    CompilationUnit {
        global_vars: [],
        var_config: [
            ConfigVariable {
                name_segments: [
                    "instance1",
                    "foo",
                ],
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
                        span: Range(
                            TextLocation {
                                line: 2,
                                column: 22,
                                offset: 38,
                            }..TextLocation {
                                line: 2,
                                column: 31,
                                offset: 47,
                            },
                        ),
                    },
                },
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 2,
                            column: 8,
                            offset: 24,
                        }..TextLocation {
                            line: 2,
                            column: 21,
                            offset: 37,
                        },
                    ),
                },
            },
            ConfigVariable {
                name_segments: [
                    "instance2",
                    "bar",
                ],
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
                        span: Range(
                            TextLocation {
                                line: 3,
                                column: 22,
                                offset: 78,
                            }..TextLocation {
                                line: 3,
                                column: 31,
                                offset: 87,
                            },
                        ),
                    },
                },
                location: SourceLocation {
                    span: Range(
                        TextLocation {
                            line: 3,
                            column: 8,
                            offset: 64,
                        }..TextLocation {
                            line: 3,
                            column: 21,
                            offset: 77,
                        },
                    ),
                },
            },
        ],
        units: [],
        implementations: [],
        user_types: [],
        file_name: "test.st",
    }
    "###);
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
