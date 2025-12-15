use std::path::{Path, PathBuf};

use insta::{assert_snapshot, internals::SnapshotContents, Snapshot};
use plc_ast::ast::CompilationUnit;

use crate::{
    header_generator::{get_generated_header, GeneratedHeader},
    GenerateHeaderOptions, GenerateLanguage,
};

// ----------------- //
// -- Test Case 1 -- //
// ----------------- //

#[test]
fn case_1_global_primitives_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_1_global_primitives_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "global_primitives.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 2 -- //
// ----------------- //

#[test]
fn case_2_global_complex_types_generated_header_file() {
    let generated_headers =
        get_all_generated_header_contents("case_2_global_complex_types_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "global_complex_types.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 3 -- //
// ----------------- //

#[test]
fn case_3_enum_types_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_3_enum_types_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "enum_types.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 4 -- //
// ----------------- //

#[test]
fn case_4_structs_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_4_structs_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "structs.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 5 -- //
// ----------------- //

#[test]
fn case_5_functions_with_primitive_types_generated_header_file() {
    let generated_headers =
        get_all_generated_header_contents("case_5_functions_with_primitive_types_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "functions_with_primitive_types.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 6 -- //
// ----------------- //

#[test]
fn case_6_functions_with_complex_types_generated_header_file() {
    let generated_headers =
        get_all_generated_header_contents("case_6_functions_with_complex_types_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "functions_with_complex_types.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 7 -- //
// ----------------- //

#[test]
fn case_7_function_blocks_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_7_function_blocks_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_blocks.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 8 -- //
// ----------------- //

#[test]
fn case_8_function_blocks_with_inheritance_generated_header_file() {
    let generated_headers =
        get_all_generated_header_contents("case_8_function_blocks_with_inheritance_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_blocks_with_inheritance.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ----------------- //
// -- Test Case 9 -- //
// ----------------- //

#[test]
fn case_9_programs_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_9_programs_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "programs.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ------------------ //
// -- Test Case 10 -- //
// ------------------ //

#[test]
fn case_10_aliases_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_10_aliases_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "aliases.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// ------------------ //
// -- Test Case 11 -- //
// ------------------ //

#[test]
fn case_11_function_pointers_generated_header_file() {
    let generated_headers = get_all_generated_header_contents("case_11_function_pointers_compilation_units");

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_pointers.h");

    assert_snapshot!(&generated_headers[0].get_contents());
}

// -------------------------------- //
// -- Re-usable pipeline methods -- //
// -------------------------------- //

/// Returns the generated headers based on the given snapshot identifier.
///
/// ---
///
/// The annotated project that is passed as part of this step is loaded based on the given test name.
///
/// ### Example:
/// ```rust
///     let generated_headers = get_all_generated_header_contents("case_1_global_primitives_compilation_units");
/// ```
/// Will load the snapshot that was generated during the run of the `fn case_1_global_primitives_annotated_content()` test.
/// At present these tests are located in the [plc_driver](file:/workspaces/rusty/compiler/plc_driver/src/tests/header_generator.rs).
fn get_all_generated_header_contents(test_name: &str) -> Vec<Box<dyn GeneratedHeader>> {
    let snapshot_string = extract_string_item_from_snapshot(test_name);
    let compilation_units = serde_json::from_str::<Vec<CompilationUnit>>(snapshot_string)
        .expect("Failed to deserialize snapshot content into the compilation units!");

    // Fetch all of the headers
    let generate_header_options = GenerateHeaderOptions {
        include_stubs: false,
        language: GenerateLanguage::C,
        output_path: PathBuf::default(),
        prefix: String::new(),
    };

    let mut generated_headers: Vec<Box<dyn GeneratedHeader>> = Vec::new();

    for unit in compilation_units {
        let generated_header =
            get_generated_header(&generate_header_options, &unit).expect("Header generation failed!");

        if !generated_header.is_empty() {
            generated_headers.push(generated_header);
        }
    }

    generated_headers
}

// -------------------- //
// -- Helper Methods -- //
// -------------------- //

/// Extracts the object from the path to the snapshot
fn extract_string_item_from_snapshot(test_name: &str) -> &'static str {
    let path = get_full_snapshot_path_to_test_with_test_name(test_name);
    let snapshot = Snapshot::from_file(path.as_path()).expect("Could not find the snapshot!");

    let contents = match snapshot.contents() {
        SnapshotContents::Text(text_snapshot_contents) => text_snapshot_contents.to_string(),
        _ => String::new(),
    };

    Box::leak(contents.into_boxed_str())
}

/// Gets the full snapshot path of the given test for this test instance
fn get_full_snapshot_path_to_test_with_test_name(test_name: &str) -> PathBuf {
    // TODO: The pathing should probably be re-evaluated
    let path =
        format!("../plc_driver/src/tests/snapshots/plc_driver__tests__header_generator__{test_name}.snap");
    Path::new(&path).to_owned()
}
