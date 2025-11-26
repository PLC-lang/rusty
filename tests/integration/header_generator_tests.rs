use std::fs;

use driver::compile;
use insta::assert_snapshot;

use crate::get_test_file;

#[test]
fn test_header_generator_cli() {
    let dir = tempfile::tempdir().unwrap();

    let working_file = get_test_file("header_generator/colour_tracker.pli");
    let test_file_pli = dir.path().join("colour_tracker.pli").to_str().unwrap().to_string();
    fs::copy(&working_file, &test_file_pli)
        .unwrap_or_else(|_| panic!("Unable to copy file from '{working_file}' to '{test_file_pli}'"));

    let parameters = &["plc", "--generate-headers", &test_file_pli];
    compile(parameters).unwrap();

    assert!(dir.path().join("colour_tracker.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("colour_tracker.h").to_str().unwrap()).unwrap());
}
