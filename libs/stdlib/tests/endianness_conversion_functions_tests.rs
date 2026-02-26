// Import common functionality into the integration tests
mod common;
use crate::common::{compile_and_run_no_params, get_includes};
use chrono::NaiveDate;

const DURATION_MILLIS: i64 = (22 * 3600 + 22 * 60 + 22) * 1000;
const DURATION_NANOS: i64 = DURATION_MILLIS * 1000000;

///-------------------------------INT
#[test]
fn test_to_big_endian_int() {
    let src = r#"FUNCTION main : INT
        main := TO_BIG_ENDIAN(INT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i16::to_be(0x1001))
}

#[test]
fn test_to_little_endian_int() {
    let src = r#"FUNCTION main : INT
        main := TO_LITTLE_ENDIAN(INT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i16::to_le(0x1001))
}

#[test]
fn test_from_big_endian_int() {
    let src = r#"FUNCTION main : INT
        main := FROM_BIG_ENDIAN(INT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i16::from_be(0x1001))
}

#[test]
fn test_from_little_endian_int() {
    let src = r#"FUNCTION main : INT
        main := FROM_LITTLE_ENDIAN(INT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i16::from_le(0x1001))
}

///-------------------------------DINT
#[test]
fn test_to_big_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := TO_BIG_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i32::to_be(0x10010A0B))
}

#[test]
fn test_to_little_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := TO_LITTLE_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i32::to_le(0x10010A0B))
}

#[test]
fn test_from_big_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := FROM_BIG_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i32::from_be(0x10010A0B))
}

#[test]
fn test_from_little_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := FROM_LITTLE_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i32::from_le(0x10010A0B))
}

///-------------------------------LINT
#[test]
fn test_to_big_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::to_be(0x10010A0B10010A0B))
}

#[test]
fn test_to_little_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::to_le(0x10010A0B10010A0B))
}

#[test]
fn test_from_big_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_be(0x10010A0B10010A0B))
}

#[test]
fn test_from_little_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_le(0x10010A0B10010A0B))
}

///-------------------------------UINT
#[test]
fn test_to_big_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := TO_BIG_ENDIAN(UINT#16#ABBA);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_be(0xABBA))
}

#[test]
fn test_to_little_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := TO_LITTLE_ENDIAN(UINT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_le(0x1001))
}

#[test]
fn test_from_big_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := FROM_BIG_ENDIAN(UINT#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_be(0x1001))
}

#[test]
fn test_from_little_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := FROM_LITTLE_ENDIAN(UINT#16#ABBA);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_le(0xABBA))
}

///-------------------------------UDINT
#[test]
fn test_to_big_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := TO_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::to_be(0x10010A0B))
}

#[test]
fn test_to_little_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := TO_LITTLE_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::to_le(0x10010A0B))
}

#[test]
fn test_from_big_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := FROM_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::from_be(0x10010A0B))
}

#[test]
fn test_from_little_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := FROM_LITTLE_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::from_le(0x10010A0B))
}

///-------------------------------ULINT
#[test]
fn test_to_big_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::to_be(0x10010A0B10010A0B))
}

#[test]
fn test_to_little_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::to_le(0x10010A0B10010A0B))
}

#[test]
fn test_from_big_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::from_be(0x10010A0B10010A0B))
}

#[test]
fn test_from_little_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::from_le(0x10010A0B10010A0B))
}

///-------------------------------REAL
#[test]
fn test_to_big_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := TO_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f32::from_ne_bytes(12.5_f32.to_be_bytes()))
}

#[test]
fn test_to_little_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := TO_LITTLE_ENDIAN(REAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f32::from_ne_bytes(12.5_f32.to_le_bytes()))
}

#[test]
fn test_from_big_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := FROM_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f32::from_be_bytes(12.5_f32.to_ne_bytes()))
}

#[test]
fn test_from_little_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := FROM_LITTLE_ENDIAN(REAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f32::from_le_bytes(12.5_f32.to_ne_bytes()))
}

///-------------------------------LREAL
#[test]
fn test_to_big_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := TO_BIG_ENDIAN(LREAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f64::from_ne_bytes(12.5_f64.to_be_bytes()))
}

#[test]
fn test_to_little_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := TO_LITTLE_ENDIAN(LREAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f64::from_ne_bytes(12.5_f64.to_le_bytes()))
}

#[test]
fn test_from_big_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := FROM_BIG_ENDIAN(LREAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f64::from_be_bytes(12.5_f64.to_ne_bytes()))
}

#[test]
fn test_from_little_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := FROM_LITTLE_ENDIAN(LREAL#12.5);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: f64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, f64::from_le_bytes(12.5_f64.to_ne_bytes()))
}

///-------------------------------WORD
#[test]
fn test_to_big_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := TO_BIG_ENDIAN(WORD#16#ABBA);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_be(0xABBA))
}

#[test]
fn test_to_little_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := TO_LITTLE_ENDIAN(WORD#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_le(0x1001))
}

#[test]
fn test_from_big_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := FROM_BIG_ENDIAN(WORD#16#1001);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_be(0x1001))
}

#[test]
fn test_from_little_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := FROM_LITTLE_ENDIAN(WORD#16#ABBA);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_le(0xABBA))
}

///-------------------------------DWORD
#[test]
fn test_to_big_endian_dword() {
    let src = r#"FUNCTION main : UDINT
        main := TO_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::to_be(0x10010A0B))
}

#[test]
fn test_to_little_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := TO_LITTLE_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::to_le(0x10010A0B))
}

#[test]
fn test_from_big_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := FROM_BIG_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::from_be(0x10010A0B))
}

#[test]
fn test_from_little_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := FROM_LITTLE_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u32 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u32::from_le(0x10010A0B))
}

///-------------------------------LWORD
#[test]
fn test_to_big_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::to_be(0x10010A0B10010A0B))
}

#[test]
fn test_to_little_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::to_le(0x10010A0B10010A0B))
}

#[test]
fn test_from_big_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::from_be(0x10010A0B10010A0B))
}

#[test]
fn test_from_little_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION
    "#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u64::from_le(0x10010A0B10010A0B))
}

///-------------------------------WCHAR
#[test]
fn test_to_big_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := TO_BIG_ENDIAN(WCHAR#'C');
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_be('C' as u16))
}

#[test]
fn test_to_little_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := TO_LITTLE_ENDIAN(WCHAR#'C');
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::to_le('C' as u16))
}

#[test]
fn test_from_big_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := FROM_BIG_ENDIAN(WCHAR#'C');
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_be('C' as u16))
}

#[test]
fn test_from_little_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := FROM_LITTLE_ENDIAN(WCHAR#'C');
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: u16 = compile_and_run_no_params(src, includes);
    assert_eq!(res, u16::from_le('C' as u16))
}

///-------------------------------DATE
#[test]
fn test_to_big_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := TO_BIG_ENDIAN(DATE#1984-06-25);
    END_FUNCTION
"#;
    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := TO_LITTLE_ENDIAN(DATE#1984-06-25);
    END_FUNCTION
"#;
    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_le()
    )
}

#[test]
fn test_from_big_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := FROM_BIG_ENDIAN(DATE#1984-06-25);
    END_FUNCTION
"#;
    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_be(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

#[test]
fn test_from_little_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := FROM_LITTLE_ENDIAN(DATE#1984-06-25);
    END_FUNCTION
"#;
    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_le(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

///-------------------------------TOD
#[test]
fn test_to_big_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := TO_BIG_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, DURATION_NANOS.to_be())
}

#[test]
fn test_to_little_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := TO_LITTLE_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, DURATION_NANOS.to_le())
}

#[test]
fn test_from_big_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := FROM_BIG_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_be(DURATION_NANOS))
}

#[test]
fn test_from_little_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := FROM_LITTLE_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_le(DURATION_NANOS))
}

///-------------------------------DT
#[test]
fn test_to_big_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := TO_BIG_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := TO_LITTLE_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_le()
    )
}

#[test]
fn test_from_big_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := FROM_BIG_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_be(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

#[test]
fn test_from_little_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := FROM_LITTLE_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_le(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

// long datetime type tests in nanos to confirm functionality.
// ldate nanos
#[test]
fn test_to_big_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := TO_BIG_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := TO_LITTLE_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_le()
    )
}

#[test]
fn test_from_big_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := FROM_BIG_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_be(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

#[test]
fn test_from_little_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := FROM_LITTLE_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_le(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

// ldt nanos
#[test]
fn test_to_big_endian_ldt_nanos() {
    let src = r#"FUNCTION main : LDT
    main := TO_BIG_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);

    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_ldt_nanos() {
    let src = r#"FUNCTION main : LDT
    main := TO_LITTLE_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
            .to_le()
    )
}

#[test]
fn test_from_big_endian_nanos() {
    let src = r#"FUNCTION main : LDT
    main := FROM_BIG_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_be(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

#[test]
fn test_from_little_endian_nanos() {
    let src = r#"FUNCTION main : LDT
    main := FROM_LITTLE_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(
        res,
        i64::from_le(
            NaiveDate::from_ymd_opt(1984, 6, 25)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_nanos_opt()
                .unwrap()
        )
    )
}

// ltod nanos
#[test]
fn test_to_big_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := TO_BIG_ENDIAN(LTOD#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, DURATION_NANOS.to_be())
}

#[test]
fn test_to_little_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := TO_LITTLE_ENDIAN(LTOD#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, DURATION_NANOS.to_le())
}

#[test]
fn test_from_big_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := FROM_BIG_ENDIAN(LTOD#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_be(DURATION_NANOS))
}

#[test]
fn test_from_little_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := FROM_LITTLE_ENDIAN(LTOD#22:22:22);
    END_FUNCTION
"#;

    let src = vec![src.into()];
    let includes = get_includes(&["endianness_conversion_functions.st"]);
    let res: i64 = compile_and_run_no_params(src, includes);
    assert_eq!(res, i64::from_le(DURATION_NANOS))
}
