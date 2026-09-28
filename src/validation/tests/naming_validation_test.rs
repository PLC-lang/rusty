use crate::{lexer, parser, test_utils::tests::parse_and_validate_buffered};
use insta::assert_snapshot;
use plc_ast::{ast::LinkageType, provider::IdProvider};
use plc_source::source_location::SourceLocationFactory;

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

#[test]
fn internal_prefix_warning_covers_type_forms_at_declaration_names() {
    let diagnostics = parse_and_validate_buffered(
        "TYPE
    __Alias : DINT;
    __Struct : STRUCT field : DINT; END_STRUCT
    __Enum : (First, Second);
    __Array : ARRAY[0..1] OF DINT;
END_TYPE
VAR_GLOBAL
    value : __Alias;
    record : __Struct;
    choice : __Enum;
    items : __Array;
END_VAR",
    );

    assert_snapshot!(diagnostics, @r"
    warning[E158]: Type name '__Alias' starts with '__', a prefix for compiler-generated types
      ┌─ <internal>:2:5
      │
    2 │     __Alias : DINT;
      │     ^^^^^^^ Type name '__Alias' starts with '__', a prefix for compiler-generated types

    warning[E158]: Type name '__Struct' starts with '__', a prefix for compiler-generated types
      ┌─ <internal>:3:5
      │
    3 │     __Struct : STRUCT field : DINT; END_STRUCT
      │     ^^^^^^^^ Type name '__Struct' starts with '__', a prefix for compiler-generated types

    warning[E158]: Type name '__Enum' starts with '__', a prefix for compiler-generated types
      ┌─ <internal>:4:5
      │
    4 │     __Enum : (First, Second);
      │     ^^^^^^ Type name '__Enum' starts with '__', a prefix for compiler-generated types

    warning[E158]: Type name '__Array' starts with '__', a prefix for compiler-generated types
      ┌─ <internal>:5:5
      │
    5 │     __Array : ARRAY[0..1] OF DINT;
      │     ^^^^^^^ Type name '__Array' starts with '__', a prefix for compiler-generated types
    ");
}

#[test]
fn internal_type_references_and_generated_types_do_not_warn() {
    let diagnostics = parse_and_validate_buffered(
        "TYPE
    PublicAlias : DINT;
    _Record : STRUCT __field : ARRAY[0..1] OF STRING[10]; END_STRUCT
    PublicEnum : (__First, __Second);
    PublicArray : ARRAY[0..1] OF PublicAlias;
    VoidAlias : __VOID;
END_TYPE
VAR_GLOBAL
    buffer : ARRAY[0..1] OF STRING[10];
END_VAR
FUNCTION_BLOCK foo
VAR
    value : __VOID;
    opaque : POINTER TO __VOID;
    items : ARRAY[0..1] OF PublicAlias;
END_VAR
END_FUNCTION_BLOCK",
    );

    assert!(diagnostics.is_empty(), "expected clean diagnostics, got:\n{diagnostics}");
}

#[test]
fn internal_prefix_type_warning_exempts_only_builtin_linkage() {
    let source = "TYPE __Builtin : DINT; END_TYPE";

    for (linkage, expected_warnings) in [
        (LinkageType::Internal, 1),
        (LinkageType::External, 1),
        (LinkageType::Include, 1),
        (LinkageType::BuiltIn, 0),
    ] {
        let (unit, diagnostics) = parser::parse(
            lexer::lex_with_ids(source, IdProvider::default(), SourceLocationFactory::internal(source)),
            linkage,
            "test.st",
        );

        assert_eq!(unit.user_types.len(), 1);
        assert_eq!(diagnostics.len(), expected_warnings, "{linkage:?}: {diagnostics:?}");
        for diagnostic in diagnostics {
            assert_eq!(diagnostic.get_error_code(), "E158");
        }
    }
}

#[test]
fn internal_prefix_on_non_type_names_does_not_warn() {
    let diagnostics = parse_and_validate_buffered(
        "FUNCTION_BLOCK __Block
VAR
    __member : DINT;
END_VAR
END_FUNCTION_BLOCK
FUNCTION __helper : DINT
VAR
    __local : DINT;
END_VAR
__helper := 1;
END_FUNCTION",
    );

    assert!(diagnostics.is_empty(), "expected clean diagnostics, got:\n{diagnostics}");
}
