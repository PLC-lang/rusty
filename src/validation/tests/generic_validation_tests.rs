use crate::{test_utils::tests::parse_and_validate, Diagnostic, ast::SourceRange};

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
            Diagnostic::invalid_type_nature("STRING", "Num", SourceRange::new(138..139,Some(2),Some(59),Some(2),Some(60))),
            Diagnostic::invalid_type_nature("WSTRING", "Num", SourceRange::new(215..216,Some(3),Some(61),Some(3),Some(62))),
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
            Diagnostic::invalid_type_nature("REAL", "Int", SourceRange::new(136..137,Some(2),Some(57),Some(2),Some(58))),
            Diagnostic::invalid_type_nature("LREAL", "Int", SourceRange::new(211..212,Some(3),Some(59),Some(3),Some(60))),
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
            Diagnostic::invalid_type_nature("STRING", "Int", SourceRange::new(138..139,Some(2),Some(59),Some(2),Some(60))),
            Diagnostic::invalid_type_nature("WSTRING", "Int", SourceRange::new(215..216,Some(3),Some(61),Some(3),Some(62))),
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
fn any_real_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("INT", "Real", SourceRange::new(136..137,Some(2),Some(56),Some(2),Some(57))),
            Diagnostic::invalid_type_nature("UINT", "Real", SourceRange::new(210..211,Some(3),Some(58),Some(3),Some(59))),
            Diagnostic::invalid_type_nature("BYTE", "Real", SourceRange::new(284..285,Some(4),Some(58),Some(4),Some(59))),
        ]
    );
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
            Diagnostic::invalid_type_nature("STRING", "Real", SourceRange::new(139..140,Some(2),Some(59),Some(2),Some(60))),
            Diagnostic::invalid_type_nature("WSTRING", "Real", SourceRange::new(216..217,Some(3),Some(61),Some(3),Some(62))),
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
            Diagnostic::invalid_type_nature("INT", "String", SourceRange::new(138..139,Some(2),Some(56),Some(2),Some(57))),
            Diagnostic::invalid_type_nature("UINT", "String", SourceRange::new(212..213,Some(3),Some(58),Some(3),Some(59))),
            Diagnostic::invalid_type_nature("BYTE", "String", SourceRange::new(286..287,Some(4),Some(58),Some(4),Some(59))),
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
            Diagnostic::invalid_type_nature("REAL", "String", SourceRange::new(139..140,Some(2),Some(57),Some(2),Some(58))),
            Diagnostic::invalid_type_nature("LREAL", "String", SourceRange::new(214..215,Some(3),Some(59),Some(3),Some(60))),
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
            SourceRange::new(94..101,Some(2),Some(31),Some(2),Some(38))
        ),]
    );
}
