use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use insta::assert_snapshot;

use crate::get_test_file;
use driver::compile;

fn contains_file_recursive(path: &Path) -> bool {
    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    entries.flatten().any(|entry| {
        let entry_path = entry.path();
        if entry_path.is_file() {
            true
        } else if entry_path.is_dir() {
            contains_file_recursive(&entry_path)
        } else {
            false
        }
    })
}

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

    plc_util::filtered_assert_snapshot!(content);
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
fn global_build_location_is_used_for_non_build_compile_temp_artifacts() {
    let file = get_test_file("json/simple_program.st");
    let build_dir = tempfile::tempdir().unwrap();
    let output_dir = tempfile::tempdir().unwrap();
    let output_file = output_dir.path().join("simple_program.o");
    let build_dir_str = build_dir.path().to_string_lossy().to_string();
    let output_file_str = output_file.to_string_lossy().to_string();

    compile(&["plc", file.as_str(), "--build-location", &build_dir_str, "-c", "-o", &output_file_str])
        .unwrap();

    assert!(output_file.is_file());
    assert!(contains_file_recursive(build_dir.path()));
}

#[test]
fn relative_output_with_build_location_lands_in_cwd_for_non_build() {
    // For non-`build` commands, `--build-location` must only govern intermediate
    // artifacts. A relative `-o` must be honored as cwd-relative and NOT be
    // rebased under `--build-location`.
    let file = get_test_file("json/simple_program.st");
    let build_dir = tempfile::tempdir().unwrap();
    let build_dir_str = build_dir.path().to_string_lossy().to_string();

    let unique_name = format!("relative_output_{}.ll", std::process::id());

    compile(&["plc", file.as_str(), "--build-location", &build_dir_str, "--ir", "-o", &unique_name]).unwrap();

    let cwd_output = std::env::current_dir().unwrap().join(&unique_name);
    let exists = cwd_output.is_file();

    // Walk the build dir and ensure the output filename is not present anywhere under it.
    fn walk(path: &Path, out: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    walk(&entry_path, out);
                } else {
                    out.push(entry_path);
                }
            }
        }
    }
    let mut entries = Vec::new();
    walk(build_dir.path(), &mut entries);
    let target_name = std::ffi::OsStr::new(unique_name.as_str());
    let relocated_under_build_dir = entries.iter().any(|p| p.file_name() == Some(target_name));

    // Always clean up before asserting so a failure does not leave artifacts behind.
    let _ = fs::remove_file(&cwd_output);

    assert!(exists, "expected `{unique_name}` to be created in cwd, not relocated under build-location");
    assert!(
        !relocated_under_build_dir,
        "`{unique_name}` was relocated under --build-location; expected cwd-relative placement",
    );
}

#[test]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
#[cfg_attr(target_os = "macos", ignore)]
fn generate_got_file() {
    let file = get_test_file("command_line.st");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let path = temp_file.path().to_string_lossy();
    let name = "got.json";

    compile(&["plc", file.as_str(), "-o", &path, "--online-change", "--got-layout-file", name, "--nocrt"])
        .unwrap();

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

/// Returns insta `Settings` with the given tempdir's path redacted to
/// `[tmp]`, so snapshots of `compile(...)` errors stay stable across runs.
fn settings_with_tempdir_filter(tempdir: &Path) -> insta::Settings {
    let mut settings = insta::Settings::clone_current();
    settings.add_filter(&plc_util::escape_regex_literal(&tempdir.to_string_lossy()), "[tmp]");
    settings
}

#[test]
fn missing_source_file_errors_with_path_in_message() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing.st");

    let err = compile(&["plc".to_string(), missing.to_string_lossy().into_owned()]).unwrap_err();
    settings_with_tempdir_filter(dir.path())
        .bind(|| insta::assert_snapshot!(err.to_string(), @"path '[tmp]/missing.st' does not exist"));
}

#[test]
fn missing_include_path_errors_with_path_in_message() {
    let dir = tempfile::tempdir().unwrap();
    let main = dir.path().join("main.st");
    fs::write(&main, "FUNCTION main : DINT main := 0; END_FUNCTION").unwrap();
    let bad_include = dir.path().join("no-such/*.st");

    let err = compile(&[
        "plc".to_string(),
        main.to_string_lossy().into_owned(),
        "-i".to_string(),
        bad_include.to_string_lossy().into_owned(),
    ])
    .unwrap_err();
    settings_with_tempdir_filter(dir.path())
        .bind(|| insta::assert_snapshot!(err.to_string(), @"path '[tmp]/no-such/*.st' does not exist"));
}

#[test]
fn missing_source_and_missing_include_surface_in_one_run() {
    // Directly demonstrates that path-validation errors are accumulated rather
    // than surfaced one-per-run (see PR #1713 review feedback).
    let dir = tempfile::tempdir().unwrap();
    let bad_source = dir.path().join("missing.st");
    let bad_include = dir.path().join("no-such/*.st");

    let err = compile(&[
        "plc".to_string(),
        bad_source.to_string_lossy().into_owned(),
        "-i".to_string(),
        bad_include.to_string_lossy().into_owned(),
    ])
    .unwrap_err();
    settings_with_tempdir_filter(dir.path()).bind(|| {
        insta::assert_snapshot!(err.to_string(), @r"
        path '[tmp]/missing.st' does not exist
        path '[tmp]/no-such/*.st' does not exist
        ")
    });
}

#[test]
fn glob_with_missing_parent_directory_errors() {
    let dir = tempfile::tempdir().unwrap();
    let typo_glob = dir.path().join("typo-dir/*.st");

    let err = compile(&["plc".to_string(), typo_glob.to_string_lossy().into_owned()]).unwrap_err();
    settings_with_tempdir_filter(dir.path())
        .bind(|| insta::assert_snapshot!(err.to_string(), @"path '[tmp]/typo-dir/*.st' does not exist"));
}
