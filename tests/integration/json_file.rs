use crate::cli::CompileParameters;
use crate::diagnostics::Diagnostic;
use rusty::build_with_subcommand;
use serial_test::serial;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
#[serial]
fn find_and_parse_default_plc() -> Result<(), Diagnostic> {
    env::set_current_dir("/workspaces/rusty/")?;

    if Path::new("tests/integration/data/json/build/proj.so").is_file() {
        Command::new("rm")
            .arg("tests/integration/data/json/build/proj.so")
            .output()?;
        Command::new("rm")
            .arg("tests/integration/data/json/build/libcopy.so")
            .output()?;
    }
    env::set_current_dir("tests/integration/data/json")?;

    let parameters = CompileParameters::parse(vec_of_strings!("build")).unwrap();
    if let Err(msg) = build_with_subcommand(parameters) {
        eprintln!("Error: {:?}", msg);
        std::process::exit(1);
    }

    let mut new_build = File::open("proj.so")?;
    let mut old_build = File::open("../test_build/proj.so")?;

    assert!(diff_files(&mut new_build, &mut old_build));
    assert!(Path::new("libcopy.so").is_file());

    env::set_current_dir("/workspaces/rusty/")?;
    Ok(())
}

#[test]
#[serial]
fn find_and_parse_given_build_path() -> Result<(), Diagnostic> {
    env::set_current_dir("/workspaces/rusty/")?;

    if Path::new("tests/integration/data/json/build_path/libcopy.so").is_file() {
        Command::new("rm")
            .arg("tests/integration/data/json/build_path/proj.so")
            .output()?;
        Command::new("rm")
            .arg("tests/integration/data/json/build_path/libcopy.so")
            .output()?;
    }

    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        "tests/integration/data/json/build_description_file.json",
        "--build-location",
        "tests/integration/data/json/build_path"
    ))
    .unwrap();
    if let Err(msg) = build_with_subcommand(parameters) {
        eprintln!("Error: {:?}", msg);
        std::process::exit(1);
    }

    let mut new_build = File::open("proj.so")?;
    let mut old_build = File::open("../test_build/proj.so")?;

    assert!(diff_files(&mut new_build, &mut old_build));
    assert!(Path::new("libcopy.so").is_file());

    env::set_current_dir("/workspaces/rusty/")?;
    Ok(())
}

fn diff_files(f1: &mut File, f2: &mut File) -> bool {
    let buff1 = &mut [0; 2048];
    let buff2 = &mut [0; 2048];

    loop {
        match f1.read(buff1) {
            Err(_) => return false,
            Ok(f1_read_len) => match f2.read(buff2) {
                Err(_) => return false,
                Ok(f2_read_len) => {
                    if f1_read_len != f2_read_len {
                        return false;
                    }
                    if f1_read_len == 0 {
                        return true;
                    }
                    if buff1[0..f1_read_len] != buff2[0..f2_read_len] {
                        return false;
                    }
                }
            },
        }
    }
}
