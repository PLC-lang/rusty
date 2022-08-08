use crate::cli::CompileParameters;
use crate::get_test_file;
use rusty::{build_with_params, build_with_subcommand};
use std::env::temp_dir;
use std::path::Path;

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker not available for Windows")]
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
#[cfg_attr(target_os = "windows", ignore = "linker not available for Windows")]
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

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker not available for Windows")]
fn build_with_cc_linker() {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/plc8.json"),
        "--target",
        "x86_64-unkown-linux-gnu",
        "--build-location",
        dir.display(),
        "--linker",
        "cc"
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(Path::new(&format!("{}/cc_proj.so", dir.display())).is_file());
}

#[test]
fn build_with_clang_linker_windows() {
    let dir = temp_dir();

    let first_parameters = CompileParameters::parse(vec_of_strings!(
        "-c",
        get_test_file("json/simple_program.st"),
        "-o",
        dir.join("test.lib").display()
    ))
    .unwrap();
    build_with_params(first_parameters).unwrap();

    assert!(dir.join("test.lib").is_file());

    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/plc10.json"),
        "--build-location",
        dir.display(),
        "--linker",
        "clang"
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir.join("clang_proj.so").is_file());
}
