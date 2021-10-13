use crate::{
    typesystem::{DataTypeInformation, StructSource},
    validation::tests::parse_and_validate,
    Diagnostic,
};

// supported return types
#[test]
fn function_array_return_supported() {
    //GIVEN FUNCTION returning an ARRAY
    //WHEN parse_and_validate is done
    let diagnostics =
        parse_and_validate("FUNCTION foo : ARRAY[0..3] OF INT VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_subrange_return_supported() {
    //GIVEN FUNCTION returning a SubRange
    //WHEN parse_and_validate is done
    let diagnostics =
        parse_and_validate("FUNCTION foo : INT(0..10) OF INT VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_pointer_return_supported() {
    //GIVEN FUNCTION returning a POINTER
    //WHEN parse_and_validate is done
    let diagnostics =
        parse_and_validate("FUNCTION foo : REF_TO INT OF INT VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// STRING types
#[test]
fn function_string_return_supported() {
    //GIVEN FUNCTION returning a STRING
    //WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo : STRING VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_string_len_return_supported() {
    //GIVEN FUNCTION returning a STRING[10]
    //WHEN parse_and_validate is done
    let diagnostics =
        parse_and_validate("FUNCTION foo : STRING[10] VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_wstring_return_supported() {
    //GIVEN FUNCTION returning a WSTRING
    //WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo : WSTRING VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_wstring_len_return_supported() {
    //GIVEN FUNCTION returning a WSTRING[10]
    //WHEN parse_and_validate is done
    let diagnostics =
        parse_and_validate("FUNCTION foo : WSTRING[10] VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// SCALAR types
#[test]
fn function_int_return_supported() {
    //GIVEN FUNCTION returning an INT
    //WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo : INT VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn function_bool_return_supported() {
    //GIVEN FUNCTION returning a BOOL
    //WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo : BOOL VAR_INPUT END_VAR END_FUNCTION");
    //THEN there shouldn't be any diagnostics -> valid return type
    assert_eq!(diagnostics, vec![]);
}

// unsupported return types
#[test]
fn function_no_return_unsupported() {
    // GIVEN FUNCTION with no return type
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate("FUNCTION foo VAR_INPUT END_VAR END_FUNCTION");
    // THEN there should be one diagnostic -> missing return type
    assert_eq!(
        diagnostics,
        vec![Diagnostic::function_return_missing((0..43).into())]
    );
}

#[test]
fn function_enum_return_unsupported() {
    // GIVEN FUNCTION returning an ENUM
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        "FUNCTION foo : (green, yellow, red) foo VAR_INPUT END_VAR END_FUNCTION",
    );
    // THEN there should be one diagnostic -> unsupported return type
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unsupported_return_type(
            &DataTypeInformation::Enum {
                name: "__foo_return".into(),
                elements: vec!["green".into(), "yellow".into(), "red".into()]
            },
            (15..35).into()
        )]
    );
}

#[test]
fn function_struct_return_unsupported() {
    // GIVEN FUNCTION returning a STRUCT
    // WHEN parse_and_validate is done
    let diagnostics = parse_and_validate(
        "FUNCTION foo : STRUCT x : INT; y : INT; END_STRUCT foo VAR_INPUT END_VAR END_FUNCTION",
    );
    // THEN there should be one diagnostic -> unsupported return type
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unsupported_return_type(
            &DataTypeInformation::Struct {
                member_names: vec!["x".into(), "y".into()],
                name: "__foo_return".into(),
                source: StructSource::OriginalDeclaration,
                varargs: None
            },
            (15..50).into()
        )]
    );
}
