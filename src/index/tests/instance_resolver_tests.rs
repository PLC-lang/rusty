use crate::{index::instance_iterator::Instance, test_utils::tests::index};

#[test]
fn global_vars_are_retrieved() {
    let (_, index) = index(
        "
    VAR_GLOBAL
    a,b : INT;
    ",
    );

    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn programs_are_retrieved() {
    let (_, index) = index(
        "
        PROGRAM MainProg
        END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn functions_are_not_retrieved() {
    let (_, index) = index(
        "
    FUNCTION main : DINT
    END_FUNCTION
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn functions_blocks_are_not_retrieved() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    END_FUNCTION_BLOCK
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn program_variables_are_retrieved() {
    let (_, index) = index(
        "
        PROGRAM MainProg
        VAR
            a,b : DINT;
        END_VAR
        END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn global_struct_variables_are_retrieved() {
    let (_, index) = index(
        "
    TYPE str : STRUCT
        a,b : DINT;
    END_STRUCT
    END_TYPE
    VAR_GLOBAL
        gStr : str;
    END_VAR
    PROGRAM MainProg
    VAR
        pStr : str;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn nested_global_struct_variables_are_retrieved() {
    let (_, index) = index(
        "
    TYPE str : STRUCT
        a,b : str2;
    END_STRUCT
    END_TYPE
    TYPE str2 : STRUCT
        c,d : DINT;
    END_STRUCT
    END_TYPE
    VAR_GLOBAL
        gStr : str;
    END_VAR
    PROGRAM MainProg
    VAR
        pStr : str;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn global_fb_variables_are_retrieved() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    VAR
        a,b : DINT;
    END_VAR
    END_FUNCTION_BLOCK
    VAR_GLOBAL
        gFb : fb;
    END_VAR
    PROGRAM MainProg
    VAR
        pFb : fb;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn array_instances_are_repeated() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    VAR
        c,b : DINT;
    END_VAR
    END_FUNCTION_BLOCK
    PROGRAM MainProg
    VAR
        aFb : ARRAY[0..2] OF fb;
        aFb1 : ARRAY[0..2,0..1] OF fb;
        aFb3 : ARRAY[0..2] OF ARRAY[0..1] of fb;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn array_with_const_instances_are_repeated() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    VAR
        c,b : DINT;
    END_VAR
    END_FUNCTION_BLOCK
    PROGRAM MainProg
    VAR CONSTANT
        size : DINT := 2;
    END_VAR
    VAR
        aFb : ARRAY[0..SIZE] OF fb;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn pointer_variables_are_not_retrieved() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    VAR
        a,b : DINT;
    END_VAR
    END_FUNCTION_BLOCK
    PROGRAM MainProg
    VAR
        rFb : REF_TO fb;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>());
}

#[test]
fn filter_on_variables_are_applied() {
    let (_, index) = index(
        "
    FUNCTION_BLOCK fb
    VAR
        a,b : DINT;
    END_VAR
    END_FUNCTION_BLOCK
    PROGRAM MainProg
    VAR
        rFb : fb;
    END_VAR
    END_PROGRAM
    VAR_GLOBAL CONSTANT
        gFb : fb;
    END_VAR
    ",
    );
    insta::assert_debug_snapshot!(index
        .filter_instances(|it, _| !it.is_constant())
        .collect::<Vec<Instance<'_>>>());
}

#[test]
fn aliased_structs_are_retrieved() {
    let (_, index) = index(
        "
    TYPE str : STRUCT
        a,b : DINT;
    END_STRUCT
    END_TYPE

    TYPE strAlias : str;
    END_TYPE

    VAR_GLOBAL
        gStr : strAlias;
    END_VAR

    PROGRAM MainProg
    VAR
        pStr : strAlias;
    END_VAR
    END_PROGRAM
    ",
    );
    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>(), @r#"
    [
        (
            ExpressionPath {
                names: [
                    Name(
                        "gStr",
                    ),
                ],
            },
            VariableIndexEntry {
                name: "gStr",
                qualified_name: "gStr",
                initial_value: None,
                argument_type: ByVal(
                    Global,
                ),
                is_constant: false,
                is_var_external: false,
                is_retain: false,
                data_type_name: "strAlias",
                location_in_parent: 0,
                linkage: Internal,
                binding: None,
                source_location: SourceLocation {
                    span: Range(10:8 - 10:12),
                    file: Some(
                        "<internal>",
                    ),
                },
                varargs: None,
            },
        ),
        (
            ExpressionPath {
                names: [
                    Name(
                        "MainProg",
                    ),
                ],
            },
            VariableIndexEntry {
                name: "MainProg_instance",
                qualified_name: "MainProg",
                initial_value: None,
                argument_type: ByVal(
                    Global,
                ),
                is_constant: false,
                is_var_external: false,
                is_retain: false,
                data_type_name: "MainProg",
                location_in_parent: 0,
                linkage: Internal,
                binding: None,
                source_location: SourceLocation {
                    span: Range(13:12 - 13:20),
                    file: Some(
                        "<internal>",
                    ),
                },
                varargs: None,
            },
        ),
        (
            ExpressionPath {
                names: [
                    Name(
                        "MainProg",
                    ),
                    Name(
                        "pStr",
                    ),
                ],
            },
            VariableIndexEntry {
                name: "pStr",
                qualified_name: "MainProg.pStr",
                initial_value: None,
                argument_type: ByVal(
                    Local,
                ),
                is_constant: false,
                is_var_external: false,
                is_retain: false,
                data_type_name: "strAlias",
                location_in_parent: 0,
                linkage: Internal,
                binding: None,
                source_location: SourceLocation {
                    span: Range(15:8 - 15:12),
                    file: Some(
                        "<internal>",
                    ),
                },
                varargs: None,
            },
        ),
    ]
    "#);
}

#[test]
fn enum_variables_are_not_recursed() {
    let (_, index) = index(
        "
    TYPE enumType : (
        element0,
        element1,
    );
    END_TYPE
    VAR_GLOBAL
        myEnum : enumType;
    END_VAR
    ",
    );

    insta::assert_debug_snapshot!(index.find_instances().collect::<Vec<Instance<'_>>>(), @r#"
    [
        (
            ExpressionPath {
                names: [
                    Name(
                        "myEnum",
                    ),
                ],
            },
            VariableIndexEntry {
                name: "myEnum",
                qualified_name: "myEnum",
                initial_value: None,
                argument_type: ByVal(
                    Global,
                ),
                is_constant: false,
                is_var_external: false,
                is_retain: false,
                data_type_name: "enumType",
                location_in_parent: 0,
                linkage: Internal,
                binding: None,
                source_location: SourceLocation {
                    span: Range(7:8 - 7:14),
                    file: Some(
                        "<internal>",
                    ),
                },
                varargs: None,
            },
        ),
    ]
    "#);
}
