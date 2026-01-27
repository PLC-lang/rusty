use driver::parse_and_annotate;
use insta::assert_debug_snapshot;
use plc_ast::ast::{Assignment, AstStatement};
use plc_source::SourceContainer;
use rusty::resolver::AnnotationMap;

use crate::get_test_file;

#[test]
fn label_added_to_index_as_annotation() {
    let cfc_file = get_test_file("cfc/jump_true.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let index = parse_and_annotate("plc", vec![cfc_file]).unwrap().1.index;
    assert_debug_snapshot!(index.get_label("main", "lbl").unwrap());
}

#[test]
fn jumps_annotated_with_label_annoations() {
    let cfc_file = get_test_file("cfc/jump_true.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap().1;
    let annotations = &annotated_project.annotations;
    let unit = &annotated_project.units[0].get_unit();
    // Get the jump
    let jump = &unit.implementations[0].statements[1];
    assert_debug_snapshot!(annotations.get(jump))
}

#[test]
fn unbound_jumps_not_annotated() {
    let cfc_file = get_test_file("cfc/jump_no_label.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap().1;
    let annotations = &annotated_project.annotations;
    let unit = &annotated_project.units[0].get_unit();
    // Get the jump
    let jump = &unit.implementations[0].statements[1];
    assert!(annotations.get(jump).is_none())
}

#[test]
fn action_variables_annotated() {
    let cfc_file = get_test_file("cfc/actions.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap().1;
    let annotations = &annotated_project.annotations;
    let unit = &annotated_project.units[0].get_unit();

    //Action 1 and 2 calls annotated
    let act1 = &unit.implementations[0].statements[1];
    assert_debug_snapshot!(annotations.get(act1));
    let act2 = &unit.implementations[0].statements[2];
    assert_debug_snapshot!(annotations.get(act2));
    //In action 1 a is annotated
    let AstStatement::Assignment(Assignment { left, .. }) = &unit.implementations[1].statements[0].get_stmt()
    else {
        unreachable!("Statement must be an assingment");
    };
    assert_debug_snapshot!(annotations.get(left));
    //In action 2 b is annotated
    let AstStatement::Assignment(Assignment { left, .. }) = &unit.implementations[2].statements[0].get_stmt()
    else {
        unreachable!("Statement must be an assingment");
    };
    assert_debug_snapshot!(annotations.get(left));
}

#[test]
fn function_block_calls_are_annotated_correctly() {
    let main = get_test_file("cfc/function_block_call_main.cfc");
    let fb = get_test_file("cfc/function_block_call_fb.cfc");

    let main = main.load_source(None).unwrap();
    let fb = fb.load_source(None).unwrap();

    let annotated_project = parse_and_annotate("plc", vec![main, fb]).unwrap().1;
    let annotations = &annotated_project.annotations;
    let unit = &annotated_project.units[0].get_unit();

    let call_annotation = annotations.get(&unit.implementations[0].statements[0]).unwrap().clone();
    assert_debug_snapshot!(call_annotation, @r#"
    Variable {
        resulting_type: "myFb",
        qualified_name: "main.fb0",
        constant: false,
        argument_type: ByVal(
            Local,
        ),
        auto_deref: None,
    }
    "#);
}
