use crate::cli::CompileParameters;
use crate::diagnostics::Diagnostic;
use crate::get_test_file;
use rusty::build_with_subcommand;
use std::env::temp_dir;
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
        "--target",
        "x86_64-unkown-linux-gnu",
        "--build-location",
        dir.display()
    ))
    .unwrap();
    if let Err(msg) = build_with_subcommand(parameters) {
        eprintln!("Error: {:?}", msg);
        std::process::exit(1);
    }

    assert!(Path::new(&format!("{}/libcopy.so", dir.display())).is_file());
    assert!(Path::new(&format!("{}/proj.so", dir.display())).is_file());

    Ok(())
}
