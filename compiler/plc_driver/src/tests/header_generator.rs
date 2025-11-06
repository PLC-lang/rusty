use std::{
    env,
    path::{Path, PathBuf},
};

use ast::ast::CompilationUnit;
use insta::{assert_snapshot, internals::SnapshotContents, Snapshot};
use source_code::SourceCode;

use crate::{
    pipelines::{AnnotatedProject, IndexedProject, ParsedProject},
    tests::{
        progress_pipeline_to_step_annotated, progress_pipeline_to_step_indexed,
        progress_pipeline_to_step_parsed,
    },
};

// ----------------- //
// -- Test Case 1 -- //
// ----------------- //

fn get_source_code_for_case_1_global_primitives() -> SourceCode {
    SourceCode::new(
        "
    VAR_GLOBAL
        gVarBool: BOOL;

        gVarSInt: SINT;
        gVarInt: INT;
        gVarDInt: DINT;
        gVarLInt: LINT;

        gVarByte: BYTE;
        gVarWord: WORD;
        gVarDWord: DWORD;
        gVarLWord: LWORD;

        gVarReal: REAL;
        gVarLReal: LREAL;

        gVarDate: DATE;
        gVarDateAndTime: DATE_AND_TIME;
    END_VAR
    ",
        "global_primitives.pli",
    )
}

#[test]
fn case_1_global_primitives_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_1_global_primitives());
    assert_snapshot!(json);
}

#[test]
fn case_1_global_primitives_indexed_content() {
    let json = get_indexed_content(
        "case_1_global_primitives_parsed_content",
        get_source_code_for_case_1_global_primitives(),
    );
    assert_snapshot!(json);
}

#[test]
fn case_1_global_primitives_annotated_content() {
    let annotated_project = get_annotated_project(
        "case_1_global_primitives_indexed_content",
        get_source_code_for_case_1_global_primitives(),
    );

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_1_global_primitives_compilation_units(&annotated_project);
}

fn case_1_global_primitives_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// ----------------- //
// -- Test Case 2 -- //
// ----------------- //

fn get_source_code_for_case_2_global_complex_types() -> SourceCode {
    SourceCode::new(
        "
    VAR_GLOBAL
        gVarString: STRING[255];
        gVarWString: WSTRING[6000];

        gVarIntArray: ARRAY[0..11] OF INT;

        gVarRefToInt: REFERENCE TO INT;
        gVarPointerToInt: REF_TO INT;
        gVarRefToDate: REFERENCE TO DATE;
        gVarPointerToDate: REF_TO DATE;

        gVarIntWithRange: INT(0..100);
    END_VAR
    ",
        "global_complex_types.pli",
    )
}

#[test]
fn case_2_global_complex_types_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_2_global_complex_types());
    assert_snapshot!(json);
}

#[test]
fn case_2_global_complex_types_indexed_content() {
    let json = get_indexed_content(
        "case_2_global_complex_types_parsed_content",
        get_source_code_for_case_2_global_complex_types(),
    );
    assert_snapshot!(json);
}

#[test]
fn case_2_global_complex_types_annotated_content() {
    let annotated_project = get_annotated_project(
        "case_2_global_complex_types_indexed_content",
        get_source_code_for_case_2_global_complex_types(),
    );

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_2_global_complex_types_compilation_units(&annotated_project);
}

fn case_2_global_complex_types_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// ----------------- //
// -- Test Case 3 -- //
// ----------------- //

fn get_source_code_for_case_3_enum_types() -> SourceCode {
    SourceCode::new(
        "
    TYPE SimpleEnumType : (
            red,
            green,
            blue
        );
    END_TYPE

    TYPE ComplexEnumType : (
            orange := 10,
            yellow := 20,
            purple := 30
        );
    END_TYPE
    ",
        "enum_types.pli",
    )
}

#[test]
fn case_3_enum_types_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_3_enum_types());
    assert_snapshot!(json);
}

#[test]
fn case_3_enum_types_indexed_content() {
    let json =
        get_indexed_content("case_3_enum_types_parsed_content", get_source_code_for_case_3_enum_types());
    assert_snapshot!(json);
}

#[test]
fn case_3_enum_types_annotated_content() {
    let annotated_project =
        get_annotated_project("case_3_enum_types_indexed_content", get_source_code_for_case_3_enum_types());

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_3_enum_types_compilation_units(&annotated_project);
}

fn case_3_enum_types_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// ----------------- //
// -- Test Case 4 -- //
// ----------------- //

fn get_source_code_for_case_4_structs() -> SourceCode {
    SourceCode::new(
        "
    TYPE ComplexEnumType : (
            orange := 10,
            yellow := 20,
            purple := 30
        );
    END_TYPE

    TYPE StructWithPrimitiveTypes:
        STRUCT
            Field1 : BYTE;
            Field2 : INT;
            Field3 : DINT;
        END_STRUCT
    END_TYPE

    TYPE StructWithComplexTypes:
        STRUCT
            byteField : BYTE;
            intField : INT;
            dIntField : DINT;
            stringField : STRING[255];
            wStringField : WSTRING[6000];
            complexEnumTypeField : ComplexEnumType;
            intArrayField: ARRAY[0..9] OF INT;
        END_STRUCT
    END_TYPE
    ",
        "structs.pli",
    )
}

#[test]
fn case_4_structs_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_4_structs());
    assert_snapshot!(json);
}

#[test]
fn case_4_structs_indexed_content() {
    let json = get_indexed_content("case_4_structs_parsed_content", get_source_code_for_case_4_structs());
    assert_snapshot!(json);
}

#[test]
fn case_4_structs_annotated_content() {
    let annotated_project =
        get_annotated_project("case_4_structs_indexed_content", get_source_code_for_case_4_structs());

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_4_structs_compilation_units(&annotated_project);
}

fn case_4_structs_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// ----------------- //
// -- Test Case 5 -- //
// ----------------- //

fn get_source_code_for_case_5_functions_with_primitive_types() -> SourceCode {
    SourceCode::new(
        "
    FUNCTION fnThatIsVoid
    VAR_INPUT
        varIntInput: INT;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatReturnsInt: INT
    VAR_INPUT
        varRealInputToFunc: REAL;
        varIntInputToFunc: INT;
        varDateAndTimeInputToFunc: DATE_AND_TIME;
    END_VAR
    VAR_IN_OUT
        varRealInOutFunc: REAL;
    END_VAR
    VAR_OUTPUT
        varIntOutputFunc: INT;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatReturnsTimeOfDay : TOD
    VAR_INPUT
        varDateTimeInput : DT;
    END_VAR
    END_FUNCTION
    ",
        "functions_with_primitive_types.pli",
    )
}

#[test]
fn case_5_functions_with_primitive_types_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_5_functions_with_primitive_types());
    assert_snapshot!(json);
}

#[test]
fn case_5_functions_with_primitive_types_indexed_content() {
    let json = get_indexed_content(
        "case_5_functions_with_primitive_types_parsed_content",
        get_source_code_for_case_5_functions_with_primitive_types(),
    );
    assert_snapshot!(json);
}

#[test]
fn case_5_functions_with_primitive_types_annotated_content() {
    let annotated_project = get_annotated_project(
        "case_5_functions_with_primitive_types_indexed_content",
        get_source_code_for_case_5_functions_with_primitive_types(),
    );

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_5_functions_with_primitive_types_compilation_units(&annotated_project);
}

fn case_5_functions_with_primitive_types_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// ----------------- //
// -- Test Case 6 -- //
// ----------------- //

fn get_source_code_for_case_6_functions_with_complex_types() -> SourceCode {
    SourceCode::new(
        "
    TYPE ComplexEnumType : (
            orange := 10,
            yellow := 20,
            purple := 30
        );
    END_TYPE

    TYPE StructWithPrimitiveTypes:
        STRUCT
            Field1 : BYTE;
            Field2 : INT;
            Field3 : DINT;
        END_STRUCT
    END_TYPE

    TYPE StructWithComplexTypes:
        STRUCT
            byteField : BYTE;
            intField : INT;
            dIntField : DINT;
            stringField : STRING[255];
            wStringField : WSTRING[6000];
            complexEnumTypeField : ComplexEnumType;
            intArrayField: ARRAY[0..9] OF INT;
        END_STRUCT
    END_TYPE

    FUNCTION fnThatUsesStructWithPrimitiveTypes: StructWithPrimitiveTypes
    VAR_INPUT
        varStruct: StructWithPrimitiveTypes;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatUsesStructWithComplexTypes: StructWithComplexTypes
    VAR_INPUT
        varStruct: StructWithComplexTypes;
        varEnum: ComplexEnumType;
        varString: STRING[200];
        varIntArray: ARRAY[0..14] OF INT;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatUsesPrimitiveTypesAndReferences
    VAR_INPUT
        varInt: INT;
        varRefToInt: REFERENCE TO INT;
        varPointerToInt: REF_TO INT;
        varRefToDate: REFERENCE TO DATE;
        varPointerToDate: REF_TO DATE;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasIntWithRange
    VAR_INPUT
        varIntWithRange: INT(0..99);
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariadicIntInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicInt: {sized} INT...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariadicStringInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicString: {sized} STRING...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariadicComplexTypeInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicStruct: {sized} StructWithPrimitiveTypes...;
    END_VAR
    END_FUNCTION
    ",
        "functions_with_complex_types.pli",
    )
}

#[test]
fn case_6_functions_with_complex_types_parsed_content() {
    let json = get_parsed_content(get_source_code_for_case_6_functions_with_complex_types());
    assert_snapshot!(json);
}

#[test]
fn case_6_functions_with_complex_types_indexed_content() {
    let json = get_indexed_content(
        "case_6_functions_with_complex_types_parsed_content",
        get_source_code_for_case_6_functions_with_complex_types(),
    );
    assert_snapshot!(json);
}

#[test]
fn case_6_functions_with_complex_types_annotated_content() {
    let annotated_project = get_annotated_project(
        "case_6_functions_with_complex_types_indexed_content",
        get_source_code_for_case_6_functions_with_complex_types(),
    );

    let json = serde_json::to_string_pretty(&annotated_project).expect("Failed to serialize item!");
    assert_snapshot!(json);

    case_6_functions_with_complex_types_compilation_units(&annotated_project);
}

fn case_6_functions_with_complex_types_compilation_units(annotated_project: &AnnotatedProject) {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    for unit in &annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    let json = serde_json::to_string_pretty(&compilation_units).expect("Failed to serialize item!");
    assert_snapshot!(json);
}

// -------------------------------- //
// -- Re-usable pipeline methods -- //
// -------------------------------- //

/// Returns a json string of the parsed source code.
fn get_parsed_content(source_code: SourceCode) -> String {
    let result = progress_pipeline_to_step_parsed(vec![source_code], vec![]);
    assert!(result.is_ok());

    serde_json::to_string_pretty(&result.unwrap()).expect("Failed to serialize item!")
}

/// Returns a json string of the indexed project.
///
/// ---
///
/// The parsed project that is passed as part of this step is loaded based on the given test name.
///
/// ### Example:
/// ```rust
///     let json = get_indexed_content("case_1_global_primitives_parsed_content", get_source_code_for_case_1_global_primitives());
/// ```
/// Will load the snapshot that was generated during the run of the `fn case_1_global_primitives_parsed_content()` test.
fn get_indexed_content(test_name: &str, source_code: SourceCode) -> String {
    let snapshot_string = extract_string_item_from_snapshot(test_name);
    let parsed_project = serde_json::from_str::<ParsedProject>(snapshot_string)
        .expect("Failed to deserialize snapshot content into a ParsedProject!");

    let result = progress_pipeline_to_step_indexed(vec![source_code], vec![], parsed_project);
    assert!(result.is_ok());

    serde_json::to_string_pretty(&result.unwrap()).expect("Failed to serialize item!")
}

/// Returns the annotated project.
///
/// ---
///
/// The indexed project that is passed as part of this step is loaded based on the given test name.
///
/// ### Example:
/// ```rust
///     let annotated_project = get_annotated_project("case_1_global_primitives_indexed_content", get_source_code_for_case_1_global_primitives());
/// ```
/// Will load the snapshot that was generated during the run of the `fn case_1_global_primitives_indexed_content()` test.
fn get_annotated_project(test_name: &str, source_code: SourceCode) -> AnnotatedProject {
    let snapshot_string = extract_string_item_from_snapshot(test_name);
    let parsed_project = serde_json::from_str::<IndexedProject>(snapshot_string)
        .expect("Failed to deserialize snapshot content into an IndexedProject!");

    let result = progress_pipeline_to_step_annotated(vec![source_code], vec![], parsed_project);
    assert!(result.is_ok());

    result.unwrap()
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
    let current_dir = env::current_dir().expect("Unable to determine the current directory!");
    let path = format!("src/tests/snapshots/plc_driver__tests__header_generator__{test_name}.snap");

    let path = current_dir.join(Path::new(&path));

    path
}
