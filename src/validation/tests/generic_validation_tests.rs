use crate::{test_utils::tests::parse_and_validate, Diagnostic};

#[test]
fn any_allows_all_natures() {
    let src = r"
        TYPE str STRUCT x : INT; END_STRUCT END_TYPE
        FUNCTION test<T : ANY> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func   : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func3  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func5  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
        FUNCTION func7  : INT VAR x : str; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_number_allows_ints_reals_bits() {
    let src = r"
        FUNCTION test<T : ANY_NUMBER> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func   : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func3  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_number_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Num", (138..139).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Num", (215..216).into()),
        ]
    );
}

#[test]
fn any_int_allow_int_signed_unsigned_bit() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func   : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_int_does_not_allow_real() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Int", (136..137).into()),
            Diagnostic::invalid_type_nature("LREAL", "Int", (211..212).into()),
        ]
    );
}

#[test]
fn any_int_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Int", (138..139).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Int", (215..216).into()),
        ]
    );
}

#[test]
fn any_real_allow_real_lreal() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert!(diagnostics.is_empty());
}

#[test]
// with https://github.com/PLC-lang/rusty/issues/547 it should be possible to use ANY_REAL functions with INTs
fn any_real_does_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_real_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Real", (139..140).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Real", (216..217).into()),
        ]
    );
}

#[test]
fn any_string_allow_string_wstring() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_string_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("INT", "String", (138..139).into()),
            Diagnostic::invalid_type_nature("UINT", "String", (212..213).into()),
            Diagnostic::invalid_type_nature("BYTE", "String", (286..287).into()),
        ]
    );
}

#[test]
fn any_string_does_not_allow_real() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "String", (139..140).into()),
            Diagnostic::invalid_type_nature("LREAL", "String", (214..215).into()),
        ]
    );
}

#[test]
fn non_resolved_generics_reported() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : T END_VAR END_FUNCTION
        FUNCTION func  : INT  test(); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![Diagnostic::unresolved_generic_type(
            "T",
            "String",
            (94..101).into()
        ),]
    );
}
