use crate::cli::CompileParameters;
use crate::diagnostics::Diagnostic;
use crate::get_test_file;
use rusty::build_with_subcommand;
use std::env::temp_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;

macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec!["rustyc".to_string(), $($x.to_string()),*]);
    }

#[test]
fn build_to_temp() -> Result<(), Diagnostic> {
    let dir = temp_dir();
    let parameters = CompileParameters::parse(vec_of_strings!(
        "build",
        get_test_file("json/build_description_file.json"),
        "--build-location",
        dir.display()
    ))
    .unwrap();
    if let Err(msg) = build_with_subcommand(parameters) {
        eprintln!("Error: {:?}", msg);
        std::process::exit(1);
    }

    let mut new_build = File::open(&format!("{}/proj.so", dir.display()))?;
    let mut old_build = File::open(&get_test_file("json/test_build/proj.so"))?;

    assert!(diff_files(&mut new_build, &mut old_build));
    assert!(Path::new(&format!("{}/libcopy.so", dir.display())).is_file());
    assert!(Path::new(&format!("{}/proj.so", dir.display())).is_file());

    Ok(())
}

pub fn diff_files(f1: &mut File, f2: &mut File) -> bool {
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
