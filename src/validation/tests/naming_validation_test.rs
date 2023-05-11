use crate::{assert_validation_snapshot, test_utils::tests::parse_and_validate};

macro_rules! assert_with_type_name {
    ($name:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            let name = stringify!($name);
            let result = parse_and_validate(&format!("TYPE {name} : STRUCT x : DINT; END_STRUCT END_TYPE"));
            assert_validation_snapshot!(&result)
        }
    };
}

assert_with_type_name!(__U1);
assert_with_type_name!(BOOL);
assert_with_type_name!(BYTE);
assert_with_type_name!(SINT);
assert_with_type_name!(USINT);
assert_with_type_name!(WORD);
assert_with_type_name!(INT);
assert_with_type_name!(UINT);
assert_with_type_name!(DWORD);
assert_with_type_name!(DINT);
assert_with_type_name!(UDINT);
assert_with_type_name!(LWORD);
assert_with_type_name!(LINT);
assert_with_type_name!(DATE);
assert_with_type_name!(D);
assert_with_type_name!(LDATE);
assert_with_type_name!(LD);
assert_with_type_name!(TIME);
assert_with_type_name!(T);
assert_with_type_name!(LTIME);
assert_with_type_name!(LT);
assert_with_type_name!(DATE_AND_TIME);
assert_with_type_name!(DT);
assert_with_type_name!(LDATE_AND_TIME);
assert_with_type_name!(LDT);
assert_with_type_name!(TIME_OF_DAY);
assert_with_type_name!(TOD);
assert_with_type_name!(LTIME_OF_DAY);
assert_with_type_name!(LTOD);
assert_with_type_name!(ULINT);
assert_with_type_name!(REAL);
assert_with_type_name!(LREAL);
assert_with_type_name!(STRING);
assert_with_type_name!(WSTRING);
assert_with_type_name!(CHAR);
assert_with_type_name!(WCHAR);
assert_with_type_name!(VOID);

#[test]
fn two_identical_vlas_in_same_pou_arent_duplicated_in_symbol_map() {
    let diag = parse_and_validate(
        r#"
        FUNCTION foo : INT
        VAR_INPUT{ref}
            vla1 : ARRAY[ * , * ] OF INT;
            vla2 : ARRAY[ * , * ] OF INT;
        END_VAR
        END_FUNCTION

        FUNCTION bar : DINT
        VAR_INPUT{ref}
            vla1 : ARRAY[ * , * ] OF LINT;
            vla2 : ARRAY[ * , * ] OF SINT;
            vla3 : ARRAY[ * , *, * ] OF SINT;
        END_VAR
        END_FUNCTION
    "#,
    );

    assert_validation_snapshot!(&diag);
}

#[test]
fn global() {
    let diag = parse_and_validate(
        r#"
        VAR_GLOBAL
            vla : ARRAY[*, *] OF DINT;
        END_VAR
        
        FUNCTION foo : DINT
        VAR_IN_OUT
            arr : ARRAY[*, *] OF DINT;
        END_VAR
        END_FUNCTION
    "#,
    );

    assert_validation_snapshot!(&diag);
}
