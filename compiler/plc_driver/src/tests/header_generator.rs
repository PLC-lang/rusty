use std::path::{Path, PathBuf};

use ast::ast::CompilationUnit;
use insta::{assert_snapshot, internals::SnapshotContents, Snapshot};
use serde::{Deserialize, Serialize};
use source_code::SourceCode;

use plc_header_generator::{
    header_generator::{
        get_empty_generated_header_from_options, prepare_template_data_for_header_generation,
        template_helper::TemplateData, GeneratedHeader,
    },
    GenerateHeaderOptions, GenerateLanguage,
};

use crate::tests::{
    progress_pipeline_to_step_annotated, progress_pipeline_to_step_indexed, progress_pipeline_to_step_parsed,
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
fn case_1_global_primitives_generated_header_file_template_data() {
    let generated_headers: Vec<Box<dyn GeneratedHeader>> =
        prepare_all_generated_header_contents(get_source_code_for_case_1_global_primitives());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "global_primitives.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_1_global_primitives_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_1_global_primitives_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
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
        gVarArrayOfIntArray: ARRAY[0..11] OF ARRAY[0..11] OF INT;
        gVarArrayOfArrayOfIntArray: ARRAY[0..11] OF ARRAY[0..11] OF ARRAY[0..11] OF INT;

        gVarAltSyntaxArrayOfIntArray: ARRAY[0..11, 0..11] OF INT;

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
fn case_2_global_complex_types_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_2_global_complex_types());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "global_complex_types.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_2_global_complex_types_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_2_global_complex_types_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
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

    TYPE CodesysStyleEnumType : (
            Crimson := 100,
            Emerald := 200,
            Topaz := 300
        ) INT := Emerald;
    END_TYPE

    TYPE ComplexCodesysStyleEnumType : (
            Black := 16#88000000,
            White := 16#FFFFFF00
        ) DWORD := Black;
    END_TYPE
    ",
        "enum_types.pli",
    )
}

#[test]
fn case_3_enum_types_generated_header_file_template_data() {
    let generated_headers = prepare_all_generated_header_contents(get_source_code_for_case_3_enum_types());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "enum_types.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_3_enum_types_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_3_enum_types_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
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
fn case_4_structs_generated_header_file_template_data() {
    let generated_headers = prepare_all_generated_header_contents(get_source_code_for_case_4_structs());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "structs.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_4_structs_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_4_structs_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
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
fn case_5_functions_with_primitive_types_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_5_functions_with_primitive_types());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "functions_with_primitive_types.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_5_functions_with_primitive_types_generated_header_file() {
    let generated_header = get_all_generated_header_contents(
        "case_5_functions_with_primitive_types_generated_header_file_template_data",
    );
    assert_snapshot!(&generated_header.get_contents());
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
            arrayOfIntArrayField: ARRAY[0..3] OF ARRAY[0..9] OF INT;
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
        varArrayOfIntArray: ARRAY[0..2] OF ARRAY[0..14] OF INT;
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
        varVariadicInt: INT...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasSizedVariadicIntInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicInt: {sized} INT...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariadicStringInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicString: STRING...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasSizedVariadicStringInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicString: {sized} STRING...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariadicComplexTypeInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicStruct: StructWithPrimitiveTypes...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasSizedVariadicComplexTypeInput
    VAR_INPUT
        varIntInput: INT;
        varVariadicStruct: {sized} StructWithPrimitiveTypes...;
    END_VAR
    END_FUNCTION

    FUNCTION fnThatHasVariableLengthArrayInput
    VAR_INPUT
        varVariableLengthArray: ARRAY [*] OF INT;
    END_VAR
    END_FUNCTION
    ",
        "functions_with_complex_types.pli",
    )
}

#[test]
fn case_6_functions_with_complex_types_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_6_functions_with_complex_types());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "functions_with_complex_types.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_6_functions_with_complex_types_generated_header_file() {
    let generated_header = get_all_generated_header_contents(
        "case_6_functions_with_complex_types_generated_header_file_template_data",
    );
    assert_snapshot!(&generated_header.get_contents());
}

// ----------------- //
// -- Test Case 7 -- //
// ----------------- //

fn get_source_code_for_case_7_function_blocks() -> SourceCode {
    SourceCode::new(
        "
    FUNCTION_BLOCK fbThatHasSimpleTypes
    VAR
        varInt : INT;
    END_VAR
    VAR_OUTPUT
        outVarInt : INT;
    END_VAR
    VAR_IN_OUT
        inOutVarInt : INT;
    END_VAR
        METHOD FB_INIT
        END_METHOD
    END_FUNCTION_BLOCK

    ACTIONS fbThatHasSimpleTypes
        ACTION fbThatHasSimpleTypes_Action END_ACTION
    END_ACTIONS
    ",
        "function_blocks.pli",
    )
}

#[test]
fn case_7_function_blocks_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_7_function_blocks());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_blocks.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_7_function_blocks_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_7_function_blocks_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
}

// ----------------- //
// -- Test Case 8 -- //
// ----------------- //

fn get_source_code_for_case_8_function_blocks_with_inheritance() -> SourceCode {
    SourceCode::new(
        "
    FUNCTION_BLOCK fbGreatGrandParent
    VAR
        varGreatGrandParentInt: INT;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fbGrandParent extends fbGreatGrandParent
    VAR
        varGrandParentInt: INT;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fbParent extends fbGrandParent
    VAR
        varParentInt: INT;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fbChild extends fbParent
    VAR
        varChildInt: INT;
    END_VAR
    END_FUNCTION_BLOCK
    ",
        "function_blocks_with_inheritance.pli",
    )
}

#[test]
fn case_8_function_blocks_with_inheritance_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_8_function_blocks_with_inheritance());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_blocks_with_inheritance.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_8_function_blocks_with_inheritance_generated_header_file() {
    let generated_header = get_all_generated_header_contents(
        "case_8_function_blocks_with_inheritance_generated_header_file_template_data",
    );
    assert_snapshot!(&generated_header.get_contents());
}

// ----------------- //
// -- Test Case 9 -- //
// ----------------- //

fn get_source_code_for_case_9_programs() -> SourceCode {
    SourceCode::new(
        "
    TYPE ComplexEnumType : (
            orange := 10,
            yellow := 20,
            purple := 30
        );
    END_TYPE

    PROGRAM prog
    VAR
        enumVarOne : ComplexEnumType;
        enumVarTwo : ComplexEnumType;
    END_VAR
    END_PROGRAM
    ",
        "programs.pli",
    )
}

#[test]
fn case_9_programs_generated_header_file_template_data() {
    let generated_headers = prepare_all_generated_header_contents(get_source_code_for_case_9_programs());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "programs.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_9_programs_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_9_programs_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
}

// ------------------ //
// -- Test Case 10 -- //
// ------------------ //

fn get_source_code_for_case_10_aliases() -> SourceCode {
    SourceCode::new(
        "
    TYPE T_String : STRING[50];
    END_TYPE

    TYPE T_DInt : DINT;
    END_TYPE

    TYPE T_Array : ARRAY[0..11] OF INT;
    END_TYPE

    TYPE T_Bool : BOOL;
    END_TYPE

    FUNCTION fnThatUsesAliases
    VAR_INPUT
        varStringInput: T_String;
        varDIntInput: T_DInt;
        varArrayOfIntInput: T_Array;
        varBoolInput: T_Bool;
    END_VAR
    END_FUNCTION
    ",
        "aliases.pli",
    )
}

#[test]
fn case_10_aliases_generated_header_file_template_data() {
    let generated_headers = prepare_all_generated_header_contents(get_source_code_for_case_10_aliases());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "aliases.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_10_aliases_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_10_aliases_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
}

// ------------------ //
// -- Test Case 11 -- //
// ------------------ //

fn get_source_code_for_case_11_function_pointers() -> SourceCode {
    SourceCode::new(
        "
    VAR_GLOBAL
        fnVoidPointer: __FPOINTER fnVoidThatWillBePointedTo;
        fnDIntPointer: __FPOINTER fnDIntThatWillBePointedTo;
        fnStructPrimitivePointer: __FPOINTER fnThatUsesStructWithPrimitiveTypes;
        fnStructComplexPointer: __FPOINTER fnThatUsesStructWithComplexTypes;
    END_VAR

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

    FUNCTION fnVoidThatWillBePointedTo
    VAR_INPUT
        varDIntInput: DINT;
    END_VAR
    END_FUNCTION

    FUNCTION fnDIntThatWillBePointedTo: DINT
    VAR_INPUT
        varDIntInput: DINT;
    END_VAR
    END_FUNCTION

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
    ",
        "function_pointers.pli",
    )
}

#[test]
fn case_11_function_pointers_generated_header_file_template_data() {
    let generated_headers =
        prepare_all_generated_header_contents(get_source_code_for_case_11_function_pointers());

    // This test case should only produce one header file
    assert!(generated_headers.len() == 1);

    // Ensure the path has been configured correctly
    assert!(generated_headers[0].get_path() == "function_pointers.h");

    let prepared_header_data = PreparedHeaderData {
        template_data: generated_headers[0].get_template_data().clone(),
        directory: generated_headers[0].get_directory().to_string(),
        path: generated_headers[0].get_path().to_string(),
        file_name: generated_headers[0].get_file_name().to_string(),
        formatted_path: generated_headers[0].get_formatted_path().to_string(),
    };

    assert_snapshot!(serde_json::to_string_pretty(&prepared_header_data).expect("Failed to serialize item!"));
}

#[test]
fn case_11_function_pointers_generated_header_file() {
    let generated_header =
        get_all_generated_header_contents("case_11_function_pointers_generated_header_file_template_data");
    assert_snapshot!(&generated_header.get_contents());
}

// -------------------------------- //
// -- Re-usable pipeline methods -- //
// -------------------------------- //

/// Returns the generated headers based on the given snapshot identifier.
///
/// ---
///
/// The generated header template data that is parsed as part of this step is loaded based on the given test name.
///
/// ### Example:
/// ```rust
///     let generated_headers = get_generated_header_contents("case_1_global_primitives_generated_header_file_template_data");
/// ```
/// Will load the snapshot that was generated during the run of the `fn case_1_global_primitives_generated_header_file_template_data()` test.
/// At present these tests are located in the [plc_driver](file:/workspaces/rusty/compiler/plc_driver/src/tests/header_generator.rs).
fn get_all_generated_header_contents(test_name: &str) -> Box<dyn GeneratedHeader> {
    let prepared_header_data =
        serde_json::from_str::<PreparedHeaderData>(extract_string_item_from_snapshot(test_name))
            .expect("Failed to deserialize snapshot content into TemplateData!");

    let mut generated_header =
        get_empty_generated_header_from_options(&get_default_generated_header_options())
            .expect("Unable to get empty generated header!");
    generated_header.set_template_data(prepared_header_data.template_data);
    generated_header.set_directory(&prepared_header_data.directory);
    generated_header.set_path(&prepared_header_data.path);
    generated_header.set_file_name(&prepared_header_data.file_name);
    generated_header.set_formatted_path(&prepared_header_data.formatted_path);
    generated_header.generate_headers().expect("Header generation failed!");

    generated_header
}

/// Returns the generated header template based on the given source code.
///
/// ---
///
/// The template data is generated by invoking the pipeline
fn prepare_all_generated_header_contents(source_code: SourceCode) -> Vec<Box<dyn GeneratedHeader>> {
    let mut compilation_units: Vec<&CompilationUnit> = Vec::new();

    // Fetch parsed project
    let parsed_project_wrapper = progress_pipeline_to_step_parsed(vec![source_code.clone()], vec![]);
    let diagnostic = parsed_project_wrapper.as_ref().err();

    assert!(parsed_project_wrapper.is_ok(), "{}", diagnostic.unwrap().message);
    let parsed_project_wrapper = parsed_project_wrapper.unwrap();

    // Fetch indexed project
    let indexed_project_wrapper =
        progress_pipeline_to_step_indexed(vec![source_code.clone()], vec![], parsed_project_wrapper);
    let diagnostic = indexed_project_wrapper.as_ref().err();

    assert!(indexed_project_wrapper.is_ok(), "{}", diagnostic.unwrap().message);
    let indexed_project_wrapper = indexed_project_wrapper.unwrap();

    // Fetch annotated project
    let annotated_project_wrapper =
        progress_pipeline_to_step_annotated(vec![source_code], vec![], indexed_project_wrapper);
    let diagnostic = annotated_project_wrapper.as_ref().err();

    assert!(annotated_project_wrapper.is_ok(), "{}", diagnostic.unwrap().message);
    let annotated_project_wrapper = annotated_project_wrapper.unwrap();

    // Collect compilation units from annotated project
    for unit in &annotated_project_wrapper.annotated_project.units {
        compilation_units.push(unit.get_unit());
    }

    // Fetch all of the headers
    let mut generated_headers: Vec<Box<dyn GeneratedHeader>> = Vec::new();

    for unit in compilation_units {
        let generated_header =
            prepare_template_data_for_header_generation(&get_default_generated_header_options(), unit)
                .expect("Header generation data preparation failed!");

        if !generated_header.is_empty() {
            generated_headers.push(generated_header);
        }
    }

    generated_headers
}

// -------------------- //
// -- Helper Methods -- //
// -------------------- //

#[derive(Serialize, Deserialize)]
pub struct PreparedHeaderData {
    pub template_data: TemplateData,
    pub directory: String,
    pub path: String,
    pub file_name: String,
    pub formatted_path: String,
}

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

/// Gets a default set of generation options for this test case
fn get_default_generated_header_options() -> GenerateHeaderOptions {
    GenerateHeaderOptions {
        include_stubs: false,
        language: GenerateLanguage::C,
        output_path: PathBuf::default(),
        prefix: String::new(),
    }
}
