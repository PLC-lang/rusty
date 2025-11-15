use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn any_allows_all_natures() {
    let src = r"
        TYPE str : STRUCT x : INT; END_STRUCT END_TYPE
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //                                                           ^^^^^^^  ^^^^^^^^
        //                                     these types are not compatible with the other types
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn non_resolved_generics_reported() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : T END_FUNCTION
        FUNCTION func  : INT  test(); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_MAGNITUDE    ##########

#[test]
fn any_magnitude_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_magnitude_allows_time() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_magnitude_does_not_allow_strings() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_magnitude_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_MAGNITUDE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_magnitude_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_MAGNITUDE> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //                                                 ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                                                     these types are not MAGNITUDE
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_NUMBER    ##########

#[test]
fn any_num_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_num_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_num_does_not_allow_strings() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_num_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_NUM> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_num_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_NUM> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //                                       ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                                                  these types are not NUM
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_REAL    ##########

#[test]
fn any_real_allows_reals() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_real_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_real_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_real_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_real_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_REAL> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //                                       ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                 ints are allowed                these types are not REAL
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_INT    ##########

#[test]
fn any_int_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_int_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_int_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_int_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_int_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_INT> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^                            ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                             these types are not REAL
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_UNSIGNED    ##########

#[test]
fn any_unsigned_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_unsigned_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_unsigned_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_unsigned_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_UNSIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_unsigned_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_UNSIGNED> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^                ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                             these types are not UNSIGNED
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_SIGNED    ##########

#[test]
fn any_signed_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_signed_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_signed_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_signed_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_SIGNED> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_signed_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_SIGNED> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^              ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                             these types are not SIGNED
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_DURATION    ##########

#[test]
fn any_duration_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_duration_allows_time() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_duration_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_duration_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_DURATION> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_duration_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_DURATION> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^            ^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                             these types are not DURATION
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_BIT    ##########

#[test]
fn any_bit_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_bit_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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
        // binary expressions
        FUNCTION func6  : INT
        VAR
        a : BOOL;
        b : BYTE;
        c : WORD;
        d : DWORD;
        e : LWORD;
        END_VAR
            test(a + b);
            test(d - c);
            test(d * d);
            test(b + e);
        END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_bit_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_bit_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_BIT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_bit_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_BIT> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^            ^^^^^^^  ^^^^^^^^  ^^^^^^^^
        //                             these types are not BIT
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_CHARS    ##########

#[test]
fn any_chars_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_chars_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_chars_allows_chars() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_chars_allows_string() {
    let src = r"
        FUNCTION test<T : ANY_CHARS> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_chars_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_CHARS> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^                     ^^^^^^^^
        //                             these types are not CHARS
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_STRING    ##########

#[test]
fn any_string_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_string_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_string_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_string_allows_string() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_string_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_STRING> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^           ^^^^^^^^  ^^^^^^^^
        //                             these types are not STRING
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_CHAR    ##########

#[test]
fn any_char_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_char_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_char_allows_chars() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_char_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_CHAR> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_char_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_CHAR> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^  ^^^^^^^            ^^^^^^^^
        //                             these types are not CHAR
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

// ##########    ANY_DATE    ##########

#[test]
fn any_date_does_not_allow_reals() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_date_does_not_allow_time() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : TIME; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : LTIME; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_date_does_not_allow_chars() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1  : INT VAR x : CHAR; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WCHAR; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn any_date_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_DATE> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func1   : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
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

    let diagnostics = parse_and_validate_buffered(src);
    assert!(diagnostics.is_empty());
}

#[test]
fn any_date_multiple_parameters() {
    let src = r"
    FUNCTION func<T : ANY_DATE> : INT VAR_INPUT in1, in2, in3, in4, in5, in6, in7, in8 : T; END_VAR END_FUNCTION

    FUNCTION foo : INT
    VAR
        var_real        : REAL;
        var_unsigned    : UDINT;
        var_signed      : DINT;
        var_time        : TIME;
        var_byte        : BYTE;
        var_str         : STRING;
        var_char        : CHAR;
        var_date        : DATE;
    END_VAR
        func(var_real, var_unsigned, var_signed, var_time, var_byte, var_str, var_char, var_date);
        //   ^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^  ^^^^^^^^  ^^^^^^^  ^^^^^^^^
        //                             these types are not DATE
    END_FUNCTION
    ";

    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(&diagnostics);
}

#[test]
fn generic_call_with_formal_parameter() {
    let src = "
    FUNCTION FOO < T: ANY_NUM >: T
    VAR_INPUT
        x: T;
    END_VAR
    END_FUNCTION

    FUNCTION FOO__DINT: DINT
    VAR_INPUT
        x: DINT;
    END_VAR
        FOO__DINT := x + 0;
    END_FUNCTION

    FUNCTION main: DINT
    VAR
        myLocalNumber: DINT := 2;
    END_VAR
        myLocalNumber := FOO(x := myLocalNumber); // okay
        myLocalNumber := FOO(y := 0); // unresolved reference
        myLocalNumber := FOO(x := 'INVALID TYPE NATURE'); // invalid type nature
    END_FUNCTION
";

    let diagnostics = parse_and_validate_buffered(src);
    insta::assert_snapshot!(diagnostics, @r"
    error[E089]: Invalid call parameters
       ┌─ <internal>:20:30
       │
    20 │         myLocalNumber := FOO(y := 0); // unresolved reference
       │                              ^^^^^^ Invalid call parameters

    error[E048]: Could not resolve reference to y
       ┌─ <internal>:20:30
       │
    20 │         myLocalNumber := FOO(y := 0); // unresolved reference
       │                              ^ Could not resolve reference to y

    error[E062]: Invalid type nature for generic argument. __STRING_19 is no ANY_NUMBER
       ┌─ <internal>:21:35
       │
    21 │         myLocalNumber := FOO(x := 'INVALID TYPE NATURE'); // invalid type nature
       │                                   ^^^^^^^^^^^^^^^^^^^^^ Invalid type nature for generic argument. __STRING_19 is no ANY_NUMBER

    error[E037]: Invalid assignment: cannot assign 'STRING' to 'USINT'
       ┌─ <internal>:21:30
       │
    21 │         myLocalNumber := FOO(x := 'INVALID TYPE NATURE'); // invalid type nature
       │                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ Invalid assignment: cannot assign 'STRING' to 'USINT'
    ");
}
