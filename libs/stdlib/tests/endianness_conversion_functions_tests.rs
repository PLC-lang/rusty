// Import common functionality into the integration tests
mod common;
use crate::common::compile_and_run_no_params;
use chrono::NaiveDate;
use common::add_std;

const DURATION_MILLIS: i64 = (22 * 3600 + 22 * 60 + 22) * 1000;
const DURATION_NANOS: i64 = DURATION_MILLIS * 1000000;

///-------------------------------INT
#[test]
fn test_to_big_endian_int() {
    let src = r#"FUNCTION main : INT
        main := TO_BIG_ENDIAN(INT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0110)
}

#[test]
fn test_to_little_endian_int() {
    let src = r#"FUNCTION main : INT
        main := TO_LITTLE_ENDIAN(INT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_big_endian_int() {
    let src = r#"FUNCTION main : INT
        main := FROM_BIG_ENDIAN(INT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_little_endian_int() {
    let src = r#"FUNCTION main : INT
        main := FROM_LITTLE_ENDIAN(INT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0110)
}

///-------------------------------DINT
#[test]
fn test_to_big_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := TO_BIG_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

#[test]
fn test_to_little_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := TO_LITTLE_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_big_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := FROM_BIG_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_little_endian_dint() {
    let src = r#"FUNCTION main : DINT
        main := FROM_LITTLE_ENDIAN(DINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

///-------------------------------LINT
#[test]
fn test_to_big_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

#[test]
fn test_to_little_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_big_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_little_endian_lint() {
    let src = r#"FUNCTION main : LINT
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

///-------------------------------UINT
#[test]
fn test_to_big_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := TO_BIG_ENDIAN(UINT#16#ABBA);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0xBAAB)
}

#[test]
fn test_to_little_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := TO_LITTLE_ENDIAN(UINT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_big_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := FROM_BIG_ENDIAN(UINT#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_little_endian_uint() {
    let src = r#"FUNCTION main : UINT
        main := FROM_LITTLE_ENDIAN(UINT#16#ABBA);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0xBAAB)
}

///-------------------------------UDINT
#[test]
fn test_to_big_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := TO_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

#[test]
fn test_to_little_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := TO_LITTLE_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_big_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := FROM_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_little_endian_udint() {
    let src = r#"FUNCTION main : UDINT
        main := FROM_LITTLE_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

///-------------------------------ULINT
#[test]
fn test_to_big_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

#[test]
fn test_to_little_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_big_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_little_endian_ulint() {
    let src = r#"FUNCTION main : ULINT
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

///-------------------------------REAL
#[test]
fn test_to_big_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := TO_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f32 = compile_and_run_no_params(src);
    assert_eq!(res, f32::from_be_bytes(12.5_f32.to_be_bytes()))
}

#[test]
fn test_to_little_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := TO_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f32 = compile_and_run_no_params(src);
    assert_eq!(res, 12.5_f32)
}

#[test]
fn test_from_big_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := FROM_BIG_ENDIAN(REAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f32 = compile_and_run_no_params(src);
    assert_eq!(res, 12.5_f32)
}

#[test]
fn test_from_little_endian_f32() {
    let src = r#"FUNCTION main : REAL
        main := FROM_LITTLE_ENDIAN(REAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f32 = compile_and_run_no_params(src);
    assert_eq!(res, f32::from_be_bytes(12.5_f32.to_be_bytes()))
}

///-------------------------------LREAL
#[test]
fn test_to_big_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := TO_BIG_ENDIAN(LREAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f64 = compile_and_run_no_params(src);
    assert_eq!(res, f64::from_be_bytes(12.5_f64.to_be_bytes()))
}

#[test]
fn test_to_little_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := TO_BIG_ENDIAN(LREAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f64 = compile_and_run_no_params(src);
    assert_eq!(res, 12.5_f64)
}

#[test]
fn test_from_big_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := FROM_BIG_ENDIAN(LREAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f64 = compile_and_run_no_params(src);
    assert_eq!(res, 12.5_f64)
}

#[test]
fn test_from_little_endian_f64() {
    let src = r#"FUNCTION main : LREAL
        main := FROM_LITTLE_ENDIAN(LREAL#12.5);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: f64 = compile_and_run_no_params(src);
    assert_eq!(res, f64::from_be_bytes(12.5f64.to_be_bytes()))
}

///-------------------------------WORD
#[test]
fn test_to_big_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := TO_BIG_ENDIAN(WORD#16#ABBA);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0xBAAB)
}

#[test]
fn test_to_little_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := TO_LITTLE_ENDIAN(WORD#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_big_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := FROM_BIG_ENDIAN(WORD#16#1001);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x1001)
}

#[test]
fn test_from_little_endian_word() {
    let src = r#"FUNCTION main : WORD
        main := FROM_LITTLE_ENDIAN(WORD#16#ABBA);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0xBAAB)
}

///-------------------------------DWORD
#[test]
fn test_to_big_endian_dword() {
    let src = r#"FUNCTION main : UDINT
        main := TO_BIG_ENDIAN(UDINT#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

#[test]
fn test_to_little_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := TO_LITTLE_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_big_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := FROM_BIG_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B)
}

#[test]
fn test_from_little_endian_dword() {
    let src = r#"FUNCTION main : DWORD
        main := FROM_LITTLE_ENDIAN(DWORD#16#10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u32 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A0110)
}

///-------------------------------LWORD
#[test]
fn test_to_big_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := TO_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

#[test]
fn test_to_little_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := TO_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_big_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := FROM_BIG_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x10010A0B10010A0B)
}

#[test]
fn test_from_little_endian_lword() {
    let src = r#"FUNCTION main : LWORD
        main := FROM_LITTLE_ENDIAN(LINT#16#10010A0B10010A0B);
        END_FUNCTION    
    "#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u64 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0B0A01100B0A0110)
}

///-------------------------------WCHAR
#[test]
fn test_to_big_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := TO_BIG_ENDIAN(WCHAR#'C');
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x4300)
}

#[test]
fn test_to_little_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := TO_LITTLE_ENDIAN(WCHAR#'C');
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0043)
}

#[test]
fn test_from_big_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := FROM_BIG_ENDIAN(WCHAR#'C');
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x0043)
}

#[test]
fn test_from_little_endian_wchar() {
    let src = r#"FUNCTION main : WCHAR
    main := FROM_LITTLE_ENDIAN(WCHAR#'C');
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: u16 = compile_and_run_no_params(src);
    assert_eq!(res, 0x4300)
}

///-------------------------------DATE
#[test]
fn test_to_big_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := TO_BIG_ENDIAN(DATE#1984-06-25);
    END_FUNCTION    
"#;
    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := TO_LITTLE_ENDIAN(DATE#1984-06-25);
    END_FUNCTION    
"#;
    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_big_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := FROM_BIG_ENDIAN(DATE#1984-06-25);
    END_FUNCTION    
"#;
    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_little_endian_date() {
    let src = r#"FUNCTION main : DATE
    main := FROM_LITTLE_ENDIAN(DATE#1984-06-25);
    END_FUNCTION    
"#;
    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

///-------------------------------TOD
#[test]
fn test_to_big_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := TO_BIG_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS.to_be())
}

#[test]
fn test_to_little_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := TO_LITTLE_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS)
}

#[test]
fn test_from_big_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := FROM_BIG_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS)
}

#[test]
fn test_from_little_endian_tod() {
    let src = r#"FUNCTION main : TIME_OF_DAY
    main := FROM_LITTLE_ENDIAN(TIME_OF_DAY#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS.to_be())
}

///-------------------------------DT
#[test]
fn test_to_big_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := TO_BIG_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := TO_LITTLE_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_big_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := FROM_BIG_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_little_endian_dt() {
    let src = r#"FUNCTION main : DATE_AND_TIME
    main := FROM_LITTLE_ENDIAN(DATE_AND_TIME#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
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

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := TO_LITTLE_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_big_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := FROM_BIG_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_little_endian_ldate_nanos() {
    let src = r#"FUNCTION main : LDATE
    main := FROM_LITTLE_ENDIAN(LDATE#1984-06-25);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

// ldt nanos
#[test]
fn test_to_big_endian_ldt_nanos() {
    let src = r#"FUNCTION main : LDT
    main := TO_BIG_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);

    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_nano_opt(0, 0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

#[test]
fn test_to_little_endian_ldt_nanos() {
    let src = r#"FUNCTION main : LDT
    main := TO_LITTLE_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_big_endian_nanos() {
    let src = r#"FUNCTION main : LDT
    main := FROM_BIG_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
    )
}

#[test]
fn test_from_little_endian_nanos() {
    let src = r#"FUNCTION main : LDT
    main := FROM_LITTLE_ENDIAN(LDT#1984-06-25-00:00:00);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(
        res,
        NaiveDate::from_ymd_opt(1984, 6, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .timestamp_nanos()
            .to_be()
    )
}

// ltod nanos
#[test]
fn test_to_big_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := TO_BIG_ENDIAN(LTOD#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS.to_be())
}

#[test]
fn test_to_little_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := TO_LITTLE_ENDIAN(LTOD#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS)
}

#[test]
fn test_from_big_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := FROM_BIG_ENDIAN(LTOD#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS)
}

#[test]
fn test_from_little_endian_ltod_nanos() {
    let src = r#"FUNCTION main : LTOD
    main := FROM_LITTLE_ENDIAN(LTOD#22:22:22);
    END_FUNCTION    
"#;

    let src = add_std!(src, "endianness_conversion_functions.st");
    let res: i64 = compile_and_run_no_params(src);
    assert_eq!(res, DURATION_NANOS.to_be())
}
