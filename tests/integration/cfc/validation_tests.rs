use std::sync::{Arc, RwLock};

use driver::parse_and_annotate;
use insta::assert_snapshot;
use plc_diagnostics::{diagnostician::Diagnostician, reporter::DiagnosticReporter};
use plc_source::SourceContainer;

use crate::get_test_file;

#[test]
fn duplicate_label_validation() {
    let cfc_file = get_test_file("cfc/duplicate_label.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let diagnostician = Arc::new(RwLock::new(Diagnostician::buffered()));
    diagnostician.write().unwrap().register_file("<internal>.cfc".to_string(), "".into());
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, diagnostician.clone()).expect_err("Expecting a validation problem");
    let buffer = diagnostician.read().unwrap().buffer().unwrap();
    assert_snapshot!(buffer)
}

#[test]
fn multiple_labels_in_file_are_no_error() {
    let cfc_file = get_test_file("cfc/multi_labels.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let diagnostician = Arc::new(RwLock::new(Diagnostician::buffered()));
    diagnostician.write().unwrap().register_file("<internal>.cfc".to_string(), "".into());
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, diagnostician.clone()).unwrap();
    assert!(diagnostician.read().unwrap().buffer().unwrap().trim().is_empty())
}

#[test]
fn jump_with_missing_label_validation() {
    let cfc_file = get_test_file("cfc/jump_missing_label.cfc");
    let mut cfc_file = cfc_file.load_source(None).unwrap();
    //Remove the path
    cfc_file.path.replace("<internal>.cfc".into());

    let diagnostician = Arc::new(RwLock::new(Diagnostician::buffered()));
    diagnostician.write().unwrap().register_file("<internal>.cfc".to_string(), "".into());
    let (ctxt, project) = parse_and_annotate("plc", vec![cfc_file]).unwrap();
    project.validate(&ctxt, diagnostician.clone()).unwrap_err();
    assert_snapshot!(diagnostician.clone().read().unwrap().buffer().unwrap())
}
