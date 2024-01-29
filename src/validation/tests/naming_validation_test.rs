use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

macro_rules! assert_with_type_name {
    ($name:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            let name = stringify!($name);
            let result =
                parse_and_validate_buffered(&format!("TYPE {name} : STRUCT x : DINT; END_STRUCT END_TYPE"));
            assert_snapshot!(&result)
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
