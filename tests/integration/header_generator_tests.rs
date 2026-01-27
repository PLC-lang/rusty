use std::fs;

use driver::compile;
use insta::assert_snapshot;

use crate::{get_test_directory, get_test_file};

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

#[test]
fn test_header_generator_output_directory() {
    let dir = tempfile::tempdir().unwrap();
    let gen_dir = dir.path().join("code_gen");

    let working_file = get_test_file("header_generator/colour_tracker.pli");
    let test_file_pli = dir.path().join("colour_tracker.pli").to_str().unwrap().to_string();
    fs::copy(&working_file, &test_file_pli)
        .unwrap_or_else(|_| panic!("Unable to copy file from '{working_file}' to '{test_file_pli}'"));

    let parameters =
        &["plc", "--header-output", gen_dir.to_str().unwrap(), "--generate-headers", &test_file_pli];
    compile(parameters).unwrap();

    assert!(gen_dir.join("colour_tracker.h").is_file());
    assert_snapshot!(fs::read_to_string(gen_dir.join("colour_tracker.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_async() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("Async").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "Async",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("Async.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("Async.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_config() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("Config").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "config_base",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("config_base.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("config_base.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_file() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("File").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "file",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("file.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("file.h").to_str().unwrap()).unwrap());
}

#[test]
#[ignore = "There is an error in the library that causes it to fail on build. This will be ignored for now."]
fn test_header_generator_library_file_async() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern =
        test_dir.join("header_generator").join("libs").join("FileAsync").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");
    let common_behaviour_model_file_pattern =
        test_dir.join("header_generator").join("libs").join("CommonBehaviourModel").join("**").join("*.pli");
    let async_file_pattern =
        test_dir.join("header_generator").join("libs").join("Async").join("**").join("*.pli");
    let file_file_pattern =
        test_dir.join("header_generator").join("libs").join("File").join("**").join("*.pli");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "file_async",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "-i",
        common_behaviour_model_file_pattern.to_str().unwrap(),
        "-i",
        async_file_pattern.to_str().unwrap(),
        "-i",
        file_file_pattern.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("file_async.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("file_async.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_log() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("Log").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "log",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("log.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("log.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_time() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("Time").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "time",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("time.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("time.h").to_str().unwrap()).unwrap());
}

#[test]
fn test_header_generator_library_vpool() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern = test_dir.join("header_generator").join("libs").join("VPool").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "vpool",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("vpool.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("vpool.h").to_str().unwrap()).unwrap());
}

#[test]
#[ignore = "There is an error in the library that causes it to fail on build. This will be ignored for now."]
fn test_header_generator_library_vpool_async() {
    let dir = tempfile::tempdir().unwrap();
    let test_dir = get_test_directory();
    let file_pattern =
        test_dir.join("header_generator").join("libs").join("VPoolAsync").join("**").join("*.pli");
    let error_file_pattern_pli =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.pli");
    let error_file_pattern_gvl =
        test_dir.join("header_generator").join("libs").join("Error").join("**").join("*.gvl");
    let common_behaviour_model_file_pattern =
        test_dir.join("header_generator").join("libs").join("CommonBehaviourModel").join("**").join("*.pli");
    let async_file_pattern =
        test_dir.join("header_generator").join("libs").join("Async").join("**").join("*.pli");
    let vpool_file_pattern =
        test_dir.join("header_generator").join("libs").join("VPool").join("**").join("*.pli");

    let parameters = &[
        "plc",
        "--header-output",
        dir.path().to_str().unwrap(),
        "-o",
        "file_async",
        "-i",
        error_file_pattern_pli.to_str().unwrap(),
        "-i",
        error_file_pattern_gvl.to_str().unwrap(),
        "-i",
        common_behaviour_model_file_pattern.to_str().unwrap(),
        "-i",
        async_file_pattern.to_str().unwrap(),
        "-i",
        vpool_file_pattern.to_str().unwrap(),
        "--generate-headers",
        file_pattern.to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("file_async.h").is_file());
    assert_snapshot!(fs::read_to_string(dir.path().join("file_async.h").to_str().unwrap()).unwrap());
}
