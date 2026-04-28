use driver::{parse_and_annotate, pipelines::AnnotatedProject};
use plc_ast::ast::PouType;
use plc_source::SourceCode;

#[test]
fn function_block_init_fn_created() {
    // GIVEN a function block with a ref initializer
    // WHEN lowered
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
           FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_FUNCTION_BLOCK
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;

    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("foo__ctor").is_some());

    // AND we expect the constructor function to be added to the same compilation unit
    let unit = units[0].get_unit();

    // Find the foo__ctor implementation and verify its properties
    let implementation = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "foo__ctor")
        .expect("foo__ctor implementation not found");
    assert_eq!(implementation.pou_type, PouType::Init);

    // Find the foo__ctor POU and verify it has a "self" parameter
    let foo_ctor_pou = unit.pous.iter().find(|pou| pou.name == "foo__ctor").expect("foo__ctor POU not found");
    assert!(!foo_ctor_pou.variable_blocks.is_empty());
    assert!(!foo_ctor_pou.variable_blocks[0].variables.is_empty());
    assert_eq!(foo_ctor_pou.variable_blocks[0].variables[0].name, "self");

    // Verify the constructor has generated statements for initialization
    assert!(!implementation.statements.is_empty());
}

#[test]
fn program_init_fn_created() {
    // GIVEN a program with a ref initializer
    // WHEN lowered
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
   PROGRAM foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, index, .. } = annotated_project;

    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("foo__ctor").is_some());

    // AND we expect the constructor function to be added to the same compilation unit
    let unit = units[0].get_unit();

    // Find the foo__ctor implementation and verify its properties
    let implementation = unit
        .implementations
        .iter()
        .find(|impl_| impl_.name == "foo__ctor")
        .expect("foo__ctor implementation not found");
    assert_eq!(implementation.pou_type, PouType::Init);

    // Find the foo__ctor POU and verify it has a "self" parameter
    let foo_ctor_pou = unit.pous.iter().find(|pou| pou.name == "foo__ctor").expect("foo__ctor POU not found");
    assert!(!foo_ctor_pou.variable_blocks.is_empty());
    assert!(!foo_ctor_pou.variable_blocks[0].variables.is_empty());
    assert_eq!(foo_ctor_pou.variable_blocks[0].variables[0].name, "self");

    // Verify the constructor has generated statements for initialization
    assert!(!implementation.statements.is_empty());
}

#[test]
fn init_wrapper_function_created() {
    let (_, annotated_project) = parse_and_annotate(
        "Test",
        vec![SourceCode::from(
            "
        VAR_GLOBAL
            s : STRING;
            gs : REFERENCE TO STRING := REF(s);
        END_VAR

        FUNCTION_BLOCK bar
        VAR
            ps AT s : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        PROGRAM foo
        VAR
            fb: bar;
        END_VAR
        END_PROGRAM
            ",
        )],
    )
    .unwrap();
    let AnnotatedProject { units, .. } = annotated_project;

    // we expect to find a `__unit___internal__ctor` function in the compilation unit
    // Note: the name has multiple underscores because the source path is "<internal>"
    let unit = units[0].get_unit();
    let implementation = unit
        .implementations
        .iter()
        .find(|impl_| dbg!(&impl_.name) == "__unit___internal____ctor")
        .expect("__unit___internal____ctor implementation not found in unit");
    assert_eq!(implementation.pou_type, PouType::ProjectInit);

    // The ProjectInit function should have no parameters
    let project_init_pou = unit
        .pous
        .iter()
        .find(|pou| pou.name == "__unit___internal____ctor")
        .expect("__unit___internal____ctor POU not found");
    assert!(project_init_pou.variable_blocks.is_empty());

    // Verify it has initialization statements
    assert!(!implementation.statements.is_empty());

    // Verify that the POU-level constructors are also created
    assert!(unit.pous.iter().any(|pou| pou.name == "bar__ctor"));
    assert!(unit.pous.iter().any(|pou| pou.name == "foo__ctor"));
}
