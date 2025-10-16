use crate::get_test_file;
use driver::compile;

#[test]
#[serial]
fn generate_header_for_empty_project() {
    let dir = tempfile::tempdir().unwrap();
    let header_dir = dir.path().join("headers").to_str().unwrap().to_string();

    let parameters = &[
        "plc",
        "generate",
        &get_test_file("empty_proj/conf/plc.json"),
        "headers",
        "--header-output",
        &header_dir,
    ];
    compile(parameters).unwrap();

    println!("{}", dir.path().join("headers").join("EmptyProject.h").to_str().unwrap());
    assert!(dir.path().join("headers").join("EmptyProject.h").is_file());
}
