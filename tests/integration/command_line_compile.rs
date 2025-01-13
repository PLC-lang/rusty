use std::fs;
use std::fs::File;
use std::io::Read;

use insta::assert_snapshot;

use crate::get_test_file;
use driver::compile;

#[test]
fn ir_generation_full_pass() {
    let file = get_test_file("command_line.st");

    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    compile(&["plc", file.as_str(), "-o", &path, "--ir"]).unwrap();

    //Verify file content
    let mut content = String::new();
    temp_file.as_file_mut().read_to_string(&mut content).unwrap();

    //Skip the module name since it is different on every system
    //We only need to test that an IR got generated, not which IR really
    let content: String = content.lines().skip(2).collect();

    assert_snapshot!(content);
}

#[test]
fn hardware_conf_full_pass_json() {
    let file = get_test_file("io.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    compile(&["plc", file.as_str(), "-o", &path, "--ir", "--hardware-conf", "json"]).unwrap();

    let mut f = File::open("json").expect("file named 'json' should have been generated");
    let mut content = String::new();
    let _foo = f.read_to_string(&mut content);
    //Verify file content

    assert_snapshot!(content);
    //clean up
    let _foo = fs::remove_file("json");
}

#[test]
fn hardware_conf_full_pass_toml() {
    let file = get_test_file("io.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    compile(&["plc", file.as_str(), "-o", &path, "--ir", "--hardware-conf", "toml"]).unwrap();

    let mut f = File::open("toml").expect("file named 'toml' should have been generated");
    let mut content = String::new();
    let _foo = f.read_to_string(&mut content);
    //Verify file content

    assert_snapshot!(content);
    //clean up
    let _foo = fs::remove_file("toml");
}

#[test]
fn stdlib_string_function_headers_compile_to_ir() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("libs");
    path.push("stdlib");
    path.push("iec61131-st");
    path.push("string_functions.st");
    let file = path.display().to_string();

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    assert!(
        compile(&["plc", file.as_str(), "-o", &path, "--ir"]).is_ok(),
        "Expected file to compile without errors"
    )
}

#[test]
fn generate_got_file() {
    let file = get_test_file("command_line.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    let name = "got.json";

    compile(&["plc", file.as_str(), "-o", &path, "--online-change", "--got-layout-file", name]).unwrap();

    //Verify file content
    let mut content = String::new();
    let mut data_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push(name);
    assert!(data_path.exists());
    let mut f = File::open(&data_path).expect("file named 'got.json' should have been generated");
    let _ = f.read_to_string(&mut content).unwrap();

    // Testing to see if the file contains the function name. Snapshots are not used here because the ordering changes upon each compilation
    assert!(content.contains("myfunc"));

    // clean up
    let _foo = fs::remove_file(data_path);
}
