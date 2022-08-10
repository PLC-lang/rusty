use std::{env, process::Command};

use crate::cli::CompileParameters;
use crate::get_test_file;
use lazy_static::__Deref;
use rusty::{build_with_params, build_with_subcommand};

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
fn build_to_temp() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_to_temp.json"),
        "--target",
        "x86_64-linux-gnu",
        "--sysroot",
        "sysroot",
        "--build-location",
        dir.path().display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .path()
        .join("x86_64-linux-gnu")
        .join("proj.so")
        .is_file());
    assert!(dir.path().join("libcopy.so").is_file());
}

#[test]
fn build_with_separate_lib_folder() {
    let dir = tempfile::tempdir().unwrap();
    let lib_dir = tempfile::tempdir().unwrap();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/separate_build_and_lib.json"),
        "--target",
        "x86_64-linux-gnu",
        "--build-location",
        dir.path().display(),
        "--lib-location",
        lib_dir.path().display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .path()
        .join("x86_64-linux-gnu")
        .join("proj.so")
        .is_file());
    assert!(lib_dir.path().join("libcopy2.so").is_file());
}

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
fn build_with_target_but_without_sysroot() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_without_sysroot.json"),
        "--target",
        "x86_64-unknown-linux-gnu",
        "--build-location",
        dir.path().display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .path()
        .join("x86_64-unknown-linux-gnu")
        .join("proj.so")
        .is_file());
}

#[test]
fn build_for_multiple_targets_and_sysroots() {
    let dir = tempfile::tempdir().unwrap();
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
        dir.path().display()
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .path()
        .join("aarch64-linux-gnu")
        .join("proj.so")
        .is_file());
    assert!(dir
        .path()
        .join("x86_64-linux-gnu")
        .join("proj.so")
        .is_file());
}

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker not available for Windows")]
fn build_with_cc_linker() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_cc_linker.json"),
        "--target",
        "x86_64-unknown-linux-gnu",
        "--build-location",
        dir.path().display(),
        "--linker",
        "cc"
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir
        .path()
        .join("x86_64-unknown-linux-gnu")
        .join("cc_proj.so")
        .is_file());
}

#[test]
#[cfg_attr(target_os = "linux", ignore)]
fn build_with_clang_linker_windows() {
    let dir = tempfile::tempdir().unwrap();

    let first_parameters = CompileParameters::parse(vec_of_strings!(
        "-c",
        get_test_file("json/simple_program.st"),
        "-o",
        dir.path().join("test.lib").display()
    ))
    .unwrap();
    build_with_params(first_parameters).unwrap();

    env::set_var::<&str, String>("DIRPATH", dir.path().display().to_string());

    assert!(dir.path().join("test.lib").is_file());

    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_clang_windows.json"),
        "--build-location",
        dir.path().display(),
        "--linker",
        "clang"
    ))
    .unwrap();
    build_with_subcommand(parameters).unwrap();

    assert!(dir.path().join("clang_proj.so").is_file());
}
