use crate::{Diagnostic, diagnostics::ErrNo, test_utils::tests::parse_and_validate};

#[test]
fn any_allows_all_natures() {
    let src = r"
        TYTE str STRUCT END_STRUCT END_TYPE
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
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    assert!(diagnostics.is_empty());
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
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    assert!(diagnostics.is_empty());
}

#[test]
fn any_number_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("String checks")
}

#[test]
fn any_int_allow_int_signed_unsigned_bit() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func   : INT VAR x : INT; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : UINT; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : BYTE; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
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
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("Real checks")
}

#[test]
fn any_int_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_INT> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("String checks")
}

#[test]
fn any_real_allow_real_lreal() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
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
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("Int checks")
}

#[test]
fn any_real_does_not_allow_string() {
    let src = r"
        FUNCTION test<T : ANY_REAL> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("String checks")
}

#[test]
fn any_string_allow_string_wstring() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : STRING; END_VAR test(x); END_FUNCTION
        FUNCTION func2  : INT VAR x : WSTRING; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
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
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("Int checks")
}

#[test]
fn any_string_does_not_allow_real() {
    let src = r"
        FUNCTION test<T : ANY_STRING> : INT VAR_INPUT x : T; END_VAR END_FUNCTION
        FUNCTION func  : INT VAR x : REAL; END_VAR test(x); END_FUNCTION
        FUNCTION func1  : INT VAR x : LREAL; END_VAR test(x); END_FUNCTION
    ";

    let diagnostics = parse_and_validate(src);
    //Filter out unresolved refrences since the test function methods won't be created
    let diagnostics : Vec<Diagnostic> = diagnostics.into_iter().filter(|it| it.get_type() == &ErrNo::reference__unresolved).collect();
    todo!("Int checks")
}



