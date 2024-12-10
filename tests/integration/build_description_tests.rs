use crate::get_test_file;
use driver::compile;

#[test]
#[serial]
fn build_to_temp() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_to_temp.json"),
        "--target",
        "x86_64-linux-gnu",
        "--sysroot",
        "sysroot",
        "--build-location",
        dir.path().to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("x86_64-linux-gnu").join("proj.so").is_file());
    assert!(dir.path().join("libcopy.so").is_file());
}

#[test]
#[serial]
#[ignore = "Arch can no longer be exported because of parallel builds, this will be removed before final commit"]
fn exports_env_variable() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_to_temp.json"),
        "--target",
        "x86_64-linux-gnu",
        "--build-location",
        dir.path().to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert_eq!(std::env::var("ARCH").unwrap(), "x86_64-linux-gnu");

    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_to_temp.json"),
        "--target",
        "aarch64-unknown-linux-gnu",
        "--build-location",
        dir.path().to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert_eq!(std::env::var("ARCH").unwrap(), "aarch64-unknown-linux-gnu");
}

#[test]
#[serial]
fn build_with_separate_lib_folder() {
    let dir = tempfile::tempdir().unwrap();
    let lib_dir = tempfile::tempdir().unwrap();
    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/separate_build_and_lib.json"),
        "--target",
        "x86_64-linux-gnu",
        "--build-location",
        dir.path().to_str().unwrap(),
        "--lib-location",
        lib_dir.path().to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("x86_64-linux-gnu").join("proj.so").is_file());
    assert!(lib_dir.path().join("libcopy2.so").is_file());
}

#[test]
#[serial]
#[cfg_attr(target_os = "windows", ignore = "linker is not available for windows")]
fn build_with_target_but_without_sysroot() {
    let dir = tempfile::tempdir().unwrap();
    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_without_sysroot.json"),
        "--target",
        "x86_64-unknown-linux-gnu",
        "--build-location",
        dir.path().to_str().unwrap(),
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("x86_64-unknown-linux-gnu").join("proj.so").is_file());
}

#[test]
#[serial]
#[cfg_attr(target_os = "windows", ignore = "linker not available for Windows")]
fn build_with_cc_linker() {
    let dir = tempfile::tempdir().unwrap();
    let arch = match () {
        #[cfg(target_arch = "x86_64")]
        _ => "x86_64-unknown-linux-gnu",

        #[cfg(target_arch = "aarch64")]
        _ => match () {
            #[cfg(target_os = "macos")]
            _ => "aarch64-apple-darwin",

            #[cfg(not(target_os = "macos"))]
            _ => "aarch64-unknown-linux-gnu",
        },
    };

    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_cc_linker.json"),
        "--target",
        arch,
        "--build-location",
        dir.path().to_str().unwrap(),
        "--linker",
        "cc",
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join(arch).join("cc_proj.so").is_file());
}

#[test]
#[serial]
#[cfg_attr(not(target_os = "windows"), ignore)]
fn build_with_clang_linker_windows() {
    let dir = tempfile::tempdir().unwrap();
    let test_lib = dir.path().join("test.lib");

    let first_parameters =
        &["plc", "-c", &get_test_file("json/simple_program.st"), "-o", &test_lib.to_string_lossy()];
    compile(first_parameters).unwrap();

    assert!(dir.path().join("test.lib").is_file());

    let parameters = &[
        "plc",
        "build",
        &get_test_file("json/build_clang_windows.json"),
        "--build-location",
        dir.path().to_str().unwrap(),
        "--linker",
        "clang",
    ];
    compile(parameters).unwrap();

    assert!(dir.path().join("clang_proj.so").is_file());
}
