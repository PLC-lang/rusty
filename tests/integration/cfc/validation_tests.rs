use driver::parse_and_annotate;
use insta::assert_snapshot;
use plc_diagnostics::{diagnostician::Diagnostician, reporter::DiagnosticReporter};
use plc_source::SourceContainer;
use plc_xmlgen::xml_gen::GenerationParameters;

use crate::get_test_file;

#[test]
fn duplicate_label_validation() {
    let cfc_file = get_test_file("cfc/duplicate_label.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let mut diagnostician = Diagnostician::buffered();
    diagnostician.register_file("<internal>.cfc".to_string(), "".into());    
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, &mut diagnostician).expect_err("Expecting a validation problem");
    assert_snapshot!(diagnostician.buffer().unwrap())
}

#[test]
fn multiple_labels_in_file_are_no_error() {
    let cfc_file = get_test_file("cfc/multi_labels.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let mut diagnostician = Diagnostician::buffered();
    diagnostician.register_file("<internal>.cfc".to_string(), "".into());
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, &mut diagnostician).unwrap();
    assert!(diagnostician.buffer().unwrap().trim().is_empty())
}

#[test]
fn jump_with_missing_label_validation() {
    let cfc_file = get_test_file("cfc/jump_missing_label.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let mut diagnostician = Diagnostician::buffered();
    diagnostician.register_file("<internal>.cfc".to_string(), "".into());
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, &mut diagnostician).unwrap_err();
    assert_snapshot!(diagnostician.buffer().unwrap())
}
