use crate::cli::CompileParameters;
use crate::get_test_file;
use rusty::build_with_subcommand;
use std::env::temp_dir;
use std::path::Path;

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
fn build_to_temp() {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/plc.json"),
        "--target",
        "x86_64-unkown-linux-gnu",
        "--build-location",
        dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(Path::new(&format!("{}/libcopy.so", dir.display())).is_file());
    assert!(Path::new(&format!("{}/proj.so", dir.display())).is_file());
}

#[test]
fn build_with_separate_lib_folder() {
    let dir = temp_dir();
    let lib_dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/plc2.json"),
        "--target",
        "x86_64-unkown-linux-gnu",
        "--build-location",
        dir.display(),
        "--lib-location",
        lib_dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(Path::new(&format!("{}/proj.so", dir.display())).is_file());
    assert!(Path::new(&format!("{}/libcopy2.so", lib_dir.display())).is_file());
}
