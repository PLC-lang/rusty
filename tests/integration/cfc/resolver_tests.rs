use driver::parse_and_annotate;
use insta::assert_debug_snapshot;
use plc_ast::ast::{Assignment, AstStatement, CallStatement};
use plc_source::SourceContainer;
use rusty::resolver::AnnotationMap;

use crate::get_test_file;

#[test]
fn label_added_to_index_as_annotation() {
    let cfc_file = get_test_file("cfc/jump_true.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let index = parse_and_annotate("plc", vec![cfc_file]).unwrap().index;
    assert_debug_snapshot!(index.get_label("main", "lbl").unwrap());
}

#[test]
fn jumps_annotated_with_label_annoations() {
    let cfc_file = get_test_file("cfc/jump_true.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    let annotations = &annotated_project.annotations;
    let (unit, ..) = &annotated_project.units[0];
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

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    let annotations = &annotated_project.annotations;
    let (unit, ..) = &annotated_project.units[0];
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

    let annotated_project = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    let annotations = &annotated_project.annotations;
    let (unit, ..) = &annotated_project.units[0];

    //Action 1 and 2 calls annotated
    let act1 = &unit.implementations[0].statements[1];
    assert_debug_snapshot!(annotations.get(act1));
    let act2 = &unit.implementations[0].statements[2];
    assert_debug_snapshot!(annotations.get(act2));
    //In action 1 a is annotated
    let AstStatement::Assignment(Assignment { left, .. }) =  &unit.implementations[1].statements[0].get_stmt() else {
        unreachable!("Statement must be an assingment");
    };
    assert_debug_snapshot!(annotations.get(left));
    //In action 2 b is annotated
    let AstStatement::Assignment(Assignment { left, .. }) =  &unit.implementations[2].statements[0].get_stmt() else {
        unreachable!("Statement must be an assingment");
    };
    assert_debug_snapshot!(annotations.get(left));
}
