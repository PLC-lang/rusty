use std::fs::File;
use std::io::Read;

use insta::assert_snapshot;
use rusty::build_with_params;

use crate::get_test_file;

#[test]
fn ir_generation_full_pass() {
    let file = get_test_file("command_line.st");

    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    build_with_params(
        rusty::cli::CompileParameters::parse(&["rustyc", file.as_str(), "-o", &path, "--ir"])
            .unwrap(),
    )
    .unwrap();

    //Verify file content
    let mut content = String::new();
    temp_file
        .as_file_mut()
        .read_to_string(&mut content)
        .unwrap();

    assert_snapshot!(content);
}

#[test]
fn hardware_conf_full_pass_json() {
    let file = get_test_file("io.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    build_with_params(
        rusty::cli::CompileParameters::parse(&[
            "rustyc",
            file.as_str(),
            "-o",
            &path,
            "--ir",
            "--hardware-conf",
            "json",
        ])
        .unwrap(),
    )
    .unwrap();

    let mut f = File::open("json").expect("file named 'json' should have been generated");
    let mut content = String::new();
    let _foo = f.read_to_string(&mut content);
    //Verify file content

    assert_snapshot!(content);
}

#[test]
fn hardware_conf_full_pass_toml() {
    let file = get_test_file("io.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    build_with_params(
        rusty::cli::CompileParameters::parse(&[
            "rustyc",
            file.as_str(),
            "-o",
            &path,
            "--ir",
            "--hardware-conf",
            "toml",
        ])
        .unwrap(),
    )
    .unwrap();

    let mut f = File::open("toml").expect("file named 'toml' should have been generated");
    let mut content = String::new();
    let _foo = f.read_to_string(&mut content);
    //Verify file content

    assert_snapshot!(content);
}
