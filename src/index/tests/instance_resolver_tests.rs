use crate::{index::instance_resolver::Instance, test_utils::tests::index};

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
