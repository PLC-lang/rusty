use crate::cli::CompileParameters;
use crate::get_test_file;
use rusty::build_with_subcommand;
use std::env::temp_dir;

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
fn build_to_temp() {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_to_temp.json"),
        "--target",
        "x86_64-linux-gnu",
        "--sysroot",
        "sysroot",
        "--build-location",
        dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir.join("x86_64-linux-gnu").join("proj.so").is_file());
    assert!(dir.join("libcopy.so").is_file());
}

#[test]
fn build_with_separate_lib_folder() {
    let dir = temp_dir();
    let lib_dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/separate_build_and_lib.json"),
        "--target",
        "x86_64-linux-gnu",
        "--build-location",
        dir.display(),
        "--lib-location",
        lib_dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir.join("x86_64-linux-gnu").join("proj.so").is_file());
    assert!(dir.join("libcopy2.so").is_file());
}

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
fn build_with_target_but_without_sysroot() {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_without_sysroot.json"),
        "--target",
        "x86_64-unknown-linux-gnu",
        "--build-location",
        dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .join("x86_64-unknown-linux-gnu")
        .join("proj.so")
        .is_file());
}

#[test]
fn build_for_multiple_targets_and_sysroots() {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/multi_target_and_sysroot.json"),
        "--target",
        "aarch64-linux-gnu",
        "--target",
        "x86_64-linux-gnu",
        "--sysroot",
        "sysroot",
        "--sysroot",
        "sysroot",
        "--build-location",
        dir.display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir.join("aarch64-linux-gnu").join("proj.so").is_file());
    assert!(dir.join("x86_64-linux-gnu").join("proj.so").is_file());
}
