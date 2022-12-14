use crate::{test_utils::tests::parse_and_validate, Diagnostic};

#[test]
fn any_allows_all_natures() {
    let src = r"
        TYPE str STRUCT x : INT; END_STRUCT END_TYPE
        FUNCTION test<T : ANY> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func2   : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func3   : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func4   : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func5   : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
        FUNCTION func6   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func7   : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
		FUNCTION func8   : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func9   : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
        FUNCTION func10  : INT VAR x : str; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
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

// ##########    ANY_MAGNITUDE    ##########

#[test]
fn any_magnitude_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_magnitude_allows_ints() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func7  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
        FUNCTION func8  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_magnitude_allows_time() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_magnitude_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Magnitude", (143..144).into()),
            Diagnostic::invalid_type_nature("BYTE", "Magnitude", (217..218).into()),
            Diagnostic::invalid_type_nature("WORD", "Magnitude", (285..286).into()),
            Diagnostic::invalid_type_nature("DWORD", "Magnitude", (360..361).into()),
            Diagnostic::invalid_type_nature("LWORD", "Magnitude", (429..430).into()),
        ]
    );
}

#[test]
fn any_magnitude_does_not_allow_strings() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Magnitude", (145..146).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Magnitude", (222..223).into()),
        ]
    );
}

#[test]
fn any_magnitude_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Magnitude", (143..144).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Magnitude", (218..219).into()),
        ]
    );
}

#[test]
fn any_magnitude_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Magnitude", (141..142).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Magnitude", (214..215).into()),
            Diagnostic::invalid_type_nature("DATE", "Magnitude", (282..283).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Magnitude", (355..356).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Magnitude", (423..424).into()),
        ]
    );
}

// ##########    ANY_NUMBER    ##########

#[test]
fn any_num_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_num_allows_ints() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func7  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
        FUNCTION func8  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_num_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Num", (137..138).into()),
            Diagnostic::invalid_type_nature("TIME", "Num", (212..213).into()),
        ]
    );
}

#[test]
fn any_num_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Num", (137..138).into()),
            Diagnostic::invalid_type_nature("BYTE", "Num", (211..212).into()),
            Diagnostic::invalid_type_nature("WORD", "Num", (279..280).into()),
            Diagnostic::invalid_type_nature("DWORD", "Num", (354..355).into()),
            Diagnostic::invalid_type_nature("LWORD", "Num", (423..424).into()),
        ]
    );
}

#[test]
fn any_num_does_not_allow_strings() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Num", (139..140).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Num", (216..217).into()),
        ]
    );
}

#[test]
fn any_num_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Num", (137..138).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Num", (212..213).into()),
        ]
    );
}

#[test]
fn any_num_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
        FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Num", (135..136).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Num", (208..209).into()),
            Diagnostic::invalid_type_nature("DATE", "Num", (276..277).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Num", (349..350).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Num", (417..418).into()),
        ]
    );
}

// ##########    ANY_REAL    ##########

#[test]
fn any_real_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
// with https://github.com/PLC-lang/rusty/issues/547 it should be possible to use ANY_REAL functions with INTs
fn any_real_allows_ints() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

        FUNCTION func1 : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_real_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Real", (138..139).into()),
            Diagnostic::invalid_type_nature("TIME", "Real", (207..208).into()),
        ]
    );
}

#[test]
fn any_real_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Real", (138..139).into()),
            Diagnostic::invalid_type_nature("BYTE", "Real", (206..207).into()),
            Diagnostic::invalid_type_nature("WORD", "Real", (274..275).into()),
            Diagnostic::invalid_type_nature("DWORD", "Real", (343..344).into()),
            Diagnostic::invalid_type_nature("LWORD", "Real", (412..413).into()),
        ]
    );
}

#[test]
fn any_real_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Real", (138..139).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Real", (207..208).into()),
        ]
    );
}

#[test]
fn any_real_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Real", (141..142).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Real", (218..219).into()),
        ]
    );
}

#[test]
fn any_real_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Real", (136..137).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Real", (203..204).into()),
            Diagnostic::invalid_type_nature("DATE", "Real", (271..272).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Real", (338..339).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Real", (406..407).into()),
        ]
    );
}

// ##########    ANY_INT    ##########

#[test]
fn any_int_does_not_allow_reals() {
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
fn any_int_allows_ints() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

        FUNCTION func1 : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_int_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Int", (137..138).into()),
            Diagnostic::invalid_type_nature("TIME", "Int", (206..207).into()),
        ]
    );
}

#[test]
fn any_int_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Int", (137..138).into()),
            Diagnostic::invalid_type_nature("BYTE", "Int", (205..206).into()),
            Diagnostic::invalid_type_nature("WORD", "Int", (273..274).into()),
            Diagnostic::invalid_type_nature("DWORD", "Int", (342..343).into()),
            Diagnostic::invalid_type_nature("LWORD", "Int", (411..412).into()),
        ]
    );
}

#[test]
fn any_int_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Int", (137..138).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Int", (206..207).into()),
        ]
    );
}

#[test]
fn any_int_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Int", (140..141).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Int", (217..218).into()),
        ]
    );
}

#[test]
fn any_int_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Int", (135..136).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Int", (202..203).into()),
            Diagnostic::invalid_type_nature("DATE", "Int", (270..271).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Int", (337..338).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Int", (405..406).into()),
        ]
    );
}

// ##########    ANY_UNSIGNED    ##########

#[test]
fn any_unsigned_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Unsigned", (141..142).into()),
            Diagnostic::invalid_type_nature("LREAL", "Unsigned", (216..217).into()),
        ]
    );
}

#[test]
fn any_unsigned_allows_unsigned_ints() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1 : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_unsigned_does_not_allow_signed_ints() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
		FUNCTION func1  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("SINT", "Unsigned", (136..137).into()),
            Diagnostic::invalid_type_nature("INT", "Unsigned", (209..210).into()),
            Diagnostic::invalid_type_nature("DINT", "Unsigned", (277..278).into()),
            Diagnostic::invalid_type_nature("LINT", "Unsigned", (345..346).into()),
        ]
    );
}

#[test]
fn any_unsigned_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Unsigned", (142..143).into()),
            Diagnostic::invalid_type_nature("TIME", "Unsigned", (211..212).into()),
        ]
    );
}

#[test]
fn any_unsigned_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Unsigned", (142..143).into()),
            Diagnostic::invalid_type_nature("BYTE", "Unsigned", (210..211).into()),
            Diagnostic::invalid_type_nature("WORD", "Unsigned", (278..279).into()),
            Diagnostic::invalid_type_nature("DWORD", "Unsigned", (347..348).into()),
            Diagnostic::invalid_type_nature("LWORD", "Unsigned", (416..417).into()),
        ]
    );
}

#[test]
fn any_unsigned_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Unsigned", (142..143).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Unsigned", (211..212).into()),
        ]
    );
}

#[test]
fn any_unsigned_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Unsigned", (145..146).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Unsigned", (222..223).into()),
        ]
    );
}

#[test]
fn any_unsigned_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Unsigned", (140..141).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Unsigned", (207..208).into()),
            Diagnostic::invalid_type_nature("DATE", "Unsigned", (275..276).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Unsigned", (342..343).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Unsigned", (410..411).into()),
        ]
    );
}

// ##########    ANY_SIGNED    ##########

#[test]
fn any_signed_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Signed", (139..140).into()),
            Diagnostic::invalid_type_nature("LREAL", "Signed", (214..215).into()),
        ]
    );
}

#[test]
fn any_signed_allows_signed_ints() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1 : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_signed_does_not_allow_unsigned_ints() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Signed", (135..136).into()),
            Diagnostic::invalid_type_nature("UINT", "Signed", (209..210).into()),
            Diagnostic::invalid_type_nature("UDINT", "Signed", (278..279).into()),
            Diagnostic::invalid_type_nature("ULINT", "Signed", (347..348).into()),
        ]
    );
}

#[test]
fn any_signed_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Signed", (140..141).into()),
            Diagnostic::invalid_type_nature("TIME", "Signed", (209..210).into()),
        ]
    );
}

#[test]
fn any_signed_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Signed", (140..141).into()),
            Diagnostic::invalid_type_nature("BYTE", "Signed", (208..209).into()),
            Diagnostic::invalid_type_nature("WORD", "Signed", (276..277).into()),
            Diagnostic::invalid_type_nature("DWORD", "Signed", (345..346).into()),
            Diagnostic::invalid_type_nature("LWORD", "Signed", (414..415).into()),
        ]
    );
}

#[test]
fn any_signed_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Signed", (140..141).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Signed", (209..210).into()),
        ]
    );
}

#[test]
fn any_signed_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Signed", (143..144).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Signed", (220..221).into()),
        ]
    );
}

#[test]
fn any_signed_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Signed", (138..139).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Signed", (205..206).into()),
            Diagnostic::invalid_type_nature("DATE", "Signed", (273..274).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Signed", (340..341).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Signed", (408..409).into()),
        ]
    );
}

// ##########    ANY_DURATION    ##########

#[test]
fn any_duration_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Duration", (141..142).into()),
            Diagnostic::invalid_type_nature("LREAL", "Duration", (216..217).into()),
        ]
    );
}

#[test]
fn any_duration_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Duration", (138..139).into()),
            Diagnostic::invalid_type_nature("UINT", "Duration", (212..213).into()),
            Diagnostic::invalid_type_nature("UDINT", "Duration", (281..282).into()),
            Diagnostic::invalid_type_nature("ULINT", "Duration", (350..351).into()),
            Diagnostic::invalid_type_nature("SINT", "Duration", (419..420).into()),
            Diagnostic::invalid_type_nature("INT", "Duration", (492..493).into()),
            Diagnostic::invalid_type_nature("DINT", "Duration", (560..561).into()),
            Diagnostic::invalid_type_nature("LINT", "Duration", (628..629).into()),
        ]
    );
}

#[test]
fn any_duration_allows_time() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_duration_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Duration", (142..143).into()),
            Diagnostic::invalid_type_nature("BYTE", "Duration", (210..211).into()),
            Diagnostic::invalid_type_nature("WORD", "Duration", (278..279).into()),
            Diagnostic::invalid_type_nature("DWORD", "Duration", (347..348).into()),
            Diagnostic::invalid_type_nature("LWORD", "Duration", (416..417).into()),
        ]
    );
}

#[test]
fn any_duration_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Duration", (142..143).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Duration", (211..212).into()),
        ]
    );
}

#[test]
fn any_duration_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Duration", (145..146).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Duration", (222..223).into()),
        ]
    );
}

#[test]
fn any_duration_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Duration", (140..141).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Duration", (207..208).into()),
            Diagnostic::invalid_type_nature("DATE", "Duration", (275..276).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Duration", (342..343).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Duration", (410..411).into()),
        ]
    );
}

// ##########    ANY_BIT    ##########

#[test]
fn any_bit_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Bit", (136..137).into()),
            Diagnostic::invalid_type_nature("LREAL", "Bit", (211..212).into()),
        ]
    );
}

#[test]
fn any_bit_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Bit", (133..134).into()),
            Diagnostic::invalid_type_nature("UINT", "Bit", (207..208).into()),
            Diagnostic::invalid_type_nature("UDINT", "Bit", (276..277).into()),
            Diagnostic::invalid_type_nature("ULINT", "Bit", (345..346).into()),
            Diagnostic::invalid_type_nature("SINT", "Bit", (414..415).into()),
            Diagnostic::invalid_type_nature("INT", "Bit", (487..488).into()),
            Diagnostic::invalid_type_nature("DINT", "Bit", (555..556).into()),
            Diagnostic::invalid_type_nature("LINT", "Bit", (623..624).into()),
        ]
    );
}

#[test]
fn any_bit_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Bit", (137..138).into()),
            Diagnostic::invalid_type_nature("TIME", "Bit", (206..207).into()),
        ]
    );
}

#[test]
fn any_bit_allows_bits() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_bit_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Bit", (137..138).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Bit", (206..207).into()),
        ]
    );
}

#[test]
fn any_bit_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Bit", (140..141).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Bit", (217..218).into()),
        ]
    );
}

#[test]
fn any_bit_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Bit", (135..136).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Bit", (202..203).into()),
            Diagnostic::invalid_type_nature("DATE", "Bit", (270..271).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Bit", (337..338).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Bit", (405..406).into()),
        ]
    );
}

// ##########    ANY_CHARS    ##########

#[test]
fn any_chars_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Chars", (138..139).into()),
            Diagnostic::invalid_type_nature("LREAL", "Chars", (213..214).into()),
        ]
    );
}

#[test]
fn any_chars_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Chars", (135..136).into()),
            Diagnostic::invalid_type_nature("UINT", "Chars", (209..210).into()),
            Diagnostic::invalid_type_nature("UDINT", "Chars", (278..279).into()),
            Diagnostic::invalid_type_nature("ULINT", "Chars", (347..348).into()),
            Diagnostic::invalid_type_nature("SINT", "Chars", (416..417).into()),
            Diagnostic::invalid_type_nature("INT", "Chars", (489..490).into()),
            Diagnostic::invalid_type_nature("DINT", "Chars", (557..558).into()),
            Diagnostic::invalid_type_nature("LINT", "Chars", (625..626).into()),
        ]
    );
}

#[test]
fn any_chars_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Chars", (139..140).into()),
            Diagnostic::invalid_type_nature("TIME", "Chars", (208..209).into()),
        ]
    );
}

#[test]
fn any_chars_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Chars", (139..140).into()),
            Diagnostic::invalid_type_nature("BYTE", "Chars", (207..208).into()),
            Diagnostic::invalid_type_nature("WORD", "Chars", (275..276).into()),
            Diagnostic::invalid_type_nature("DWORD", "Chars", (344..345).into()),
            Diagnostic::invalid_type_nature("LWORD", "Chars", (413..414).into()),
        ]
    );
}

#[test]
fn any_chars_allows_chars() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_chars_allows_string() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_chars_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Chars", (137..138).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Chars", (204..205).into()),
            Diagnostic::invalid_type_nature("DATE", "Chars", (272..273).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Chars", (339..340).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Chars", (407..408).into()),
        ]
    );
}

// ##########    ANY_STRING    ##########

#[test]
fn any_string_does_not_allow_reals() {
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
fn any_string_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "String", (136..137).into()),
            Diagnostic::invalid_type_nature("UINT", "String", (210..211).into()),
            Diagnostic::invalid_type_nature("UDINT", "String", (279..280).into()),
            Diagnostic::invalid_type_nature("ULINT", "String", (348..349).into()),
            Diagnostic::invalid_type_nature("SINT", "String", (417..418).into()),
            Diagnostic::invalid_type_nature("INT", "String", (490..491).into()),
            Diagnostic::invalid_type_nature("DINT", "String", (558..559).into()),
            Diagnostic::invalid_type_nature("LINT", "String", (626..627).into()),
        ]
    );
}

#[test]
fn any_string_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "String", (140..141).into()),
            Diagnostic::invalid_type_nature("TIME", "String", (209..210).into()),
        ]
    );
}

#[test]
fn any_string_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "String", (140..141).into()),
            Diagnostic::invalid_type_nature("BYTE", "String", (208..209).into()),
            Diagnostic::invalid_type_nature("WORD", "String", (276..277).into()),
            Diagnostic::invalid_type_nature("DWORD", "String", (345..346).into()),
            Diagnostic::invalid_type_nature("LWORD", "String", (414..415).into()),
        ]
    );
}

#[test]
fn any_string_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "String", (140..141).into()),
            Diagnostic::invalid_type_nature("WCHAR", "String", (209..210).into()),
        ]
    );
}

#[test]
fn any_string_allows_string() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_string_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "String", (138..139).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "String", (205..206).into()),
            Diagnostic::invalid_type_nature("DATE", "String", (273..274).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "String", (340..341).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "String", (408..409).into()),
        ]
    );
}

// ##########    ANY_CHAR    ##########

#[test]
fn any_char_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Char", (137..138).into()),
            Diagnostic::invalid_type_nature("LREAL", "Char", (212..213).into()),
        ]
    );
}

#[test]
fn any_char_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Char", (134..135).into()),
            Diagnostic::invalid_type_nature("UINT", "Char", (208..209).into()),
            Diagnostic::invalid_type_nature("UDINT", "Char", (277..278).into()),
            Diagnostic::invalid_type_nature("ULINT", "Char", (346..347).into()),
            Diagnostic::invalid_type_nature("SINT", "Char", (415..416).into()),
            Diagnostic::invalid_type_nature("INT", "Char", (488..489).into()),
            Diagnostic::invalid_type_nature("DINT", "Char", (556..557).into()),
            Diagnostic::invalid_type_nature("LINT", "Char", (624..625).into()),
        ]
    );
}

#[test]
fn any_char_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Char", (138..139).into()),
            Diagnostic::invalid_type_nature("TIME", "Char", (207..208).into()),
        ]
    );
}

#[test]
fn any_char_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Char", (138..139).into()),
            Diagnostic::invalid_type_nature("BYTE", "Char", (206..207).into()),
            Diagnostic::invalid_type_nature("WORD", "Char", (274..275).into()),
            Diagnostic::invalid_type_nature("DWORD", "Char", (343..344).into()),
            Diagnostic::invalid_type_nature("LWORD", "Char", (412..413).into()),
        ]
    );
}

#[test]
fn any_char_allows_chars() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn any_char_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Char", (141..142).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Char", (218..219).into()),
        ]
    );
}

#[test]
fn any_char_does_not_allow_date() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Char", (136..137).into()),
            Diagnostic::invalid_type_nature("DATE_AND_TIME", "Char", (203..204).into()),
            Diagnostic::invalid_type_nature("DATE", "Char", (271..272).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Char", (338..339).into()),
            Diagnostic::invalid_type_nature("TIME_OF_DAY", "Char", (406..407).into()),
        ]
    );
}

// ##########    ANY_DATE    ##########

#[test]
fn any_date_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("REAL", "Date", (137..138).into()),
            Diagnostic::invalid_type_nature("LREAL", "Date", (212..213).into()),
        ]
    );
}

#[test]
fn any_date_does_not_allow_ints() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION

		FUNCTION func1  : INT VAR x : USINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : UDINT; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : ULINT; END_VAR test(x); END_FUNCTION

		FUNCTION func5  : INT VAR x : SINT; END_VAR test(x); END_FUNCTION
        FUNCTION func6  : INT VAR x : INT; END_VAR test(x); END_FUNCTION
		FUNCTION func7  : INT VAR x : DINT; END_VAR test(x); END_FUNCTION
		FUNCTION func8  : INT VAR x : LINT; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("USINT", "Date", (134..135).into()),
            Diagnostic::invalid_type_nature("UINT", "Date", (208..209).into()),
            Diagnostic::invalid_type_nature("UDINT", "Date", (277..278).into()),
            Diagnostic::invalid_type_nature("ULINT", "Date", (346..347).into()),
            Diagnostic::invalid_type_nature("SINT", "Date", (415..416).into()),
            Diagnostic::invalid_type_nature("INT", "Date", (488..489).into()),
            Diagnostic::invalid_type_nature("DINT", "Date", (556..557).into()),
            Diagnostic::invalid_type_nature("LINT", "Date", (624..625).into()),
        ]
    );
}

#[test]
fn any_date_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("TIME", "Date", (138..139).into()),
            Diagnostic::invalid_type_nature("TIME", "Date", (207..208).into()),
        ]
    );
}

#[test]
fn any_date_does_not_allow_bits() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : BOOL; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : WORD; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : DWORD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LWORD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("BOOL", "Date", (138..139).into()),
            Diagnostic::invalid_type_nature("BYTE", "Date", (206..207).into()),
            Diagnostic::invalid_type_nature("WORD", "Date", (274..275).into()),
            Diagnostic::invalid_type_nature("DWORD", "Date", (343..344).into()),
            Diagnostic::invalid_type_nature("LWORD", "Date", (412..413).into()),
        ]
    );
}

#[test]
fn any_date_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("CHAR", "Date", (138..139).into()),
            Diagnostic::invalid_type_nature("WCHAR", "Date", (207..208).into()),
        ]
    );
}

#[test]
fn any_date_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(
        diagnostics,
        vec![
            Diagnostic::invalid_type_nature("STRING", "Date", (141..142).into()),
            Diagnostic::invalid_type_nature("WSTRING", "Date", (218..219).into()),
        ]
    );
}

#[test]
fn any_date_allows_date() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : DT; END_VAR test(x); END_FUNCTION
		FUNCTION func2  : INT VAR x : LDT; END_VAR test(x); END_FUNCTION
		FUNCTION func3  : INT VAR x : DATE; END_VAR test(x); END_FUNCTION
		FUNCTION func4  : INT VAR x : TOD; END_VAR test(x); END_FUNCTION
		FUNCTION func5  : INT VAR x : LTOD; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}

#[test]
fn builtin_expt_with_literlals_without_explicit_type_annotation_does_not_report_errors() {
    let src = r"
    FUNCTION main : DINT
    VAR
        i : DINT;
        i2: DINT;
        r : REAL;
    END_VAR
        i := 2**DINT#3; // this works
        i2 := 2**3; // this line reports an error
        r := 3.0**2; // also does not seem to happen with REAL types
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    assert_eq!(diagnostics, vec![]);
}
