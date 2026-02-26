use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc};
mod common;
use common::{compile_and_load, compile_and_run, compile_and_run_no_params, get_includes};
use num::PrimInt;
use plc::codegen::CodegenContext;

const STR_SIZE: usize = 81;
struct MainType<T: PrimInt> {
    s: [T; 81],
}

// x to string/wstring
#[test]
fn byte_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: BYTE := 2#01010101;
    END_VAR
        main := BYTE_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = 0b01010101.to_string();
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn dword_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: DWORD := 16#BADF00D;
    END_VAR
        main := DWORD_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(0xBADF00D.to_string(), res.to_string());
}

#[test]
fn lword_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LWORD := 16#DEADBEEFDECAFBAD;
    END_VAR
        main := LWORD_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = 0xDEAD_BEEF_DECA_FBAD_u64.to_string();
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res.to_string());
}

#[test]
fn byte_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: BYTE := 2#01010101;
    END_VAR
        main := BYTE_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = 0b01010101.to_string();
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn dword_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: DWORD := 16#BADF00D;
    END_VAR
        main := DWORD_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(0xBADF00D.to_string(), res.to_string());
}

#[test]
fn lword_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LWORD := 16#DEADBEEFDECAFBAD;
    END_VAR
        main := LWORD_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = 0xDEAD_BEEF_DECA_FBAD_u64.to_string();
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LINT := 7_600_500_400_300_200_100;
    END_VAR
        main := LINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 7_600_500_400_300_200_100_i64);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lint_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LINT := 7_600_500_400_300_200_100;
    END_VAR
        main := LINT_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 7_600_500_400_300_200_100_i64);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn dint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: DINT := 2_000_200_100;
    END_VAR
        main := DINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 2_000_200_100_i32);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn dint_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: DINT := -1_300_200_100;
    END_VAR
        main := DINT_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", -1_300_200_100_i32);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn usint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: USINT := 255;
    END_VAR
        main := USINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 255);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn uint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: UINT := 65_535;
    END_VAR
        main := UINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 65_535);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn udint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: UDINT := 4_294_967_295;
    END_VAR
        main := UDINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 4_294_967_295_u32);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ulint_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: ULINT := 18_446_744_073_709_551_615;
    END_VAR
        main := ULINT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{}", 18_446_744_073_709_551_615u64);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lreal_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LREAL := 13234.25;
    END_VAR
        LREAL_TO_STRING_EXT(in, main);
        // main := LREAL_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{:.6}", 13234.25_f64);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn lreal_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LREAL := 13234.25;
    END_VAR
        main := LREAL_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{:.6}", 13234.25_f64);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn real_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: REAL := 13234.25;
    END_VAR
        main := REAL_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{:.6}", 13234.25_f32);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn real_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: REAL := 13234.25;
    END_VAR
        main := REAL_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = format!("{:.6}", 13234.25_f32);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

// x to lword
#[test]
fn tod_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: TOD := TOD#01:59:59.2567;
    END_VAR
        main := TOD_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ltod_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LTOD := LTOD#01:59:59.2567;
    END_VAR
        main := LTOD_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn dt_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: DT := DT#1999-12-31-01:59:59.2567;
    END_VAR
        main := DT_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_micro_opt(1, 59, 59, 256700).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldt_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LDT := LDT#1999-12-31-01:59:59.2567;
    END_VAR
        main := LDT_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_micro_opt(1, 59, 59, 256700).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn date_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: DATE := DATE#1999-12-31;
    END_VAR
        main := DATE_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldate_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LDATE := LDATE#1999-12-31;
    END_VAR
        main := LDATE_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn time_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: TIME := TIME#12h1m20s391ms10ns;
    END_VAR
        main := TIME_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ltime_to_lword_conversion() {
    let src = r#"
    FUNCTION main : LWORD
    VAR
        t: LTIME := LT#12h1m20s391ms10ns;
    END_VAR
        main := LTIME_TO_LWORD(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

// x to lint/dint
#[test]
fn string_to_lint_conversion() {
    #[derive(Default)]
    struct MainType {
        i1: i64,
        i2: i64,
        i3: i64,
        i4: i64,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        i1: LINT;
        i2: LINT;
        i3: LINT;
        i4: LINT;
    END_VAR
    VAR_TEMP
        s1: STRING := '123456';
        s2: STRING := '2#1010101010101010'; // 43690
        s3: STRING := '8#1234567'; // 342_391
        s4: STRING := '16#DECAFBAD'; // 3_737_844_653
    END_VAR
        i1 := STRING_TO_LINT(s1);
        i2 := STRING_TO_LINT(s2);
        i3 := STRING_TO_LINT(s3);
        i4 := STRING_TO_LINT(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_eq!(123456_i64, maintype.i1);
    assert_eq!(43690_i64, maintype.i2);
    assert_eq!(342_391, maintype.i3);
    assert_eq!(3_737_844_653, maintype.i4);
}

#[test]
fn wstring_to_lint_conversion() {
    #[derive(Default)]
    struct MainType {
        i1: i64,
        i2: i64,
        i3: i64,
        i4: i64,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        i1: LINT;
        i2: LINT;
        i3: LINT;
        i4: LINT;
    END_VAR
    VAR_TEMP
        s1: WSTRING := "-123456";
        s2: WSTRING := "2#1010101010101010"; // 43690
        s3: WSTRING := "8#1234567"; // 342_391
        s4: WSTRING := "16#DECAFBAD"; // 3_737_844_653
    END_VAR
        i1 := WSTRING_TO_LINT(s1);
        i2 := WSTRING_TO_LINT(s2);
        i3 := WSTRING_TO_LINT(s3);
        i4 := WSTRING_TO_LINT(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_eq!(-123456_i64, maintype.i1);
    assert_eq!(43690_i64, maintype.i2);
    assert_eq!(342_391, maintype.i3);
    assert_eq!(3_737_844_653, maintype.i4);
}

// x to lint
#[test]
fn string_to_dint_conversion() {
    #[derive(Default)]
    struct MainType {
        i1: i32,
        i2: i32,
        i3: i32,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        i1: DINT;
        i2: DINT;
        i3: DINT;
    END_VAR
    VAR_TEMP
        s1: STRING := '123456';
        s2: STRING := '2#1010101010101010'; // 43690
        s3: STRING := '8#1234567'; // 342_391
    END_VAR
        i1 := STRING_TO_DINT(s1);
        i2 := STRING_TO_DINT(s2);
        i3 := STRING_TO_DINT(s3);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_eq!(123456, maintype.i1);
    assert_eq!(43690, maintype.i2);
    assert_eq!(342_391, maintype.i3);
}

#[test]
fn wstring_to_dint_conversion() {
    #[derive(Default)]
    struct MainType {
        i1: i32,
        i2: i32,
        i3: i32,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        i1: DINT;
        i2: DINT;
        i3: DINT;
    END_VAR
    VAR_TEMP
        s1: WSTRING := "-123456";
        s2: WSTRING := "2#1010101010101010"; // 43690
        s3: WSTRING := "8#1234567"; // 342_391
    END_VAR
        i1 := WSTRING_TO_DINT(s1);
        i2 := WSTRING_TO_DINT(s2);
        i3 := WSTRING_TO_DINT(s3);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_eq!(-123456, maintype.i1);
    assert_eq!(43690, maintype.i2);
    assert_eq!(342_391, maintype.i3);
}

#[test]
fn time_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: TIME := TIME#12h1m20s391ms10ns;
    END_VAR
        main := TIME_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}

#[test]
fn ltime_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: LTIME := LT#12h1m20s391ms10ns;
    END_VAR
        main := LTIME_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}

#[test]
fn tod_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: TOD := TOD#01:59:59.2567;
    END_VAR
        main := TOD_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}

#[test]
fn ltod_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: LTOD := LTOD#01:59:59.2567;
    END_VAR
        main := LTOD_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as i64 * 1e9 as i64 + time.nanosecond() as i64;
    assert_eq!(expected, res)
}

#[test]
fn date_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: DATE := DATE#1999-12-31;
    END_VAR
        main := DATE_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldate_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: LDATE := LDATE#1999-12-31;
    END_VAR
        main := LDATE_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn dt_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: DT := DT#2000-01-12-23:23:00;
    END_VAR
        main := DT_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(23, 23, 0).unwrap();
    let datetime_utc = TimeZone::from_utc_datetime(&Utc, &naivedatetime_utc);
    let expected = datetime_utc.timestamp_nanos_opt().unwrap();
    assert_eq!(expected, res)
}

#[test]
fn ldt_to_lint_conversion() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        t: LDT := LDT#2000-01-12-23:23:00;
    END_VAR
        main := LDT_TO_LINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(23, 23, 0).unwrap();
    let datetime_utc = TimeZone::from_utc_datetime(&Utc, &naivedatetime_utc);
    let expected = datetime_utc.timestamp_nanos_opt().unwrap();
    assert_eq!(expected, res)
}

// x to ulint
#[test]
fn time_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: TIME := TIME#12h1m20s391ms10ns;
    END_VAR
        main := TIME_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ltime_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: LTIME := LT#12h1m20s391ms10ns;
    END_VAR
        main := LTIME_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(12, 1, 20, 391e6 as u32 + 10).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn tod_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: TOD := TOD#01:59:59.2567;
    END_VAR
        main := TOD_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ltod_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: LTOD := LTOD#01:59:59.2567;
    END_VAR
        main := LTOD_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let time = NaiveTime::from_hms_nano_opt(1, 59, 59, 2567e5 as u32).unwrap();
    let expected = time.num_seconds_from_midnight() as u64 * 1e9 as u64 + time.nanosecond() as u64;
    assert_eq!(expected, res)
}

#[test]
fn date_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: DATE := DATE#1999-12-31;
    END_VAR
        main := DATE_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldate_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: LDATE := LDATE#1999-12-31;
    END_VAR
        main := LDATE_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let date = NaiveDate::from_ymd_opt(1999, 12, 31).unwrap();
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let expected = NaiveDateTime::new(date, time).and_utc().timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn dt_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: DT := DT#2000-01-12-23:23:00;
    END_VAR
        main := DT_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(23, 23, 0).unwrap();
    let datetime_utc = TimeZone::from_utc_datetime(&Utc, &naivedatetime_utc);
    let expected = datetime_utc.timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

#[test]
fn ldt_to_ulint_conversion() {
    let src = r#"
    FUNCTION main : ULINT
    VAR
        t: LDT := LDT#2000-01-12-23:23:00;
    END_VAR
        main := LDT_TO_ULINT(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: u64 = compile_and_run_no_params(vec![src.into()], includes);

    let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(23, 23, 0).unwrap();
    let datetime_utc = TimeZone::from_utc_datetime(&Utc, &naivedatetime_utc);
    let expected = datetime_utc.timestamp_nanos_opt().unwrap() as u64;
    assert_eq!(expected, res)
}

// x to lreal/real
#[test]
fn string_to_lreal_conversion() {
    #[derive(Default)]
    struct MainType {
        f1: f64,
        f2: f64,
        f3: f64,
        f4: f64,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        f1: LREAL;
        f2: LREAL;
        f3: LREAL;
        f4: LREAL;
    END_VAR
    VAR_TEMP
        s1: STRING := '12.3456';
        s2: STRING := '1312433.1';
        s3: STRING := '1.3E-8';
        s4: STRING := '1.24e13';
    END_VAR
        f1 := STRING_TO_LREAL(s1);
        f2 := STRING_TO_LREAL(s2);
        f3 := STRING_TO_LREAL(s3);
        f4 := STRING_TO_LREAL(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_almost_eq!(12.3456, maintype.f1, 0.0001);
    assert_almost_eq!(1312433.1, maintype.f2, 0.1);
    assert_almost_eq!(1.3E-8, maintype.f3, 0.1);
    assert_almost_eq!(1.24e13, maintype.f4, 0.1);
}

#[test]
fn wstring_to_lreal_conversion() {
    #[derive(Default)]
    struct MainType {
        f1: f64,
        f2: f64,
        f3: f64,
        f4: f64,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        f1: LREAL;
        f2: LREAL;
        f3: LREAL;
        f4: LREAL;
    END_VAR
    VAR_TEMP
        s1: WSTRING := "12.3456";
        s2: WSTRING := "1312433.1";
        s3: WSTRING := "1.3E-8";
        s4: WSTRING := "1.24e13";
    END_VAR
        f1 := WSTRING_TO_LREAL(s1);
        f2 := WSTRING_TO_LREAL(s2);
        f3 := WSTRING_TO_LREAL(s3);
        f4 := WSTRING_TO_LREAL(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_almost_eq!(12.3456, maintype.f1, 0.0001);
    assert_almost_eq!(1312433.1, maintype.f2, 0.1);
    assert_almost_eq!(1.3E-8, maintype.f3, 0.1);
    assert_almost_eq!(1.24e13, maintype.f4, 0.1);
}

#[test]
fn string_to_real_conversion() {
    #[derive(Default)]
    struct MainType {
        f1: f32,
        f2: f32,
        f3: f32,
        f4: f32,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        f1: REAL;
        f2: REAL;
        f3: REAL;
        f4: REAL;
    END_VAR
    VAR_TEMP
        s1: STRING := '12.3456';
        s2: STRING := '1312433.1';
        s3: STRING := '1.3E-8';
        s4: STRING := '1.24e13';
    END_VAR
        f1 := STRING_TO_REAL(s1);
        f2 := STRING_TO_REAL(s2);
        f3 := STRING_TO_REAL(s3);
        f4 := STRING_TO_REAL(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_almost_eq!(12.3456, maintype.f1, 0.0001);
    assert_almost_eq!(1312433.1, maintype.f2, 0.1);
    assert_almost_eq!(1.3E-8, maintype.f3, 0.1);
    assert_almost_eq!(1.24e13, maintype.f4, 0.1);
}

#[test]
fn wstring_to_real_conversion() {
    #[derive(Default)]
    struct MainType {
        f1: f32,
        f2: f32,
        f3: f32,
        f4: f32,
    }
    let mut maintype = MainType::default();
    let src = r#"
    PROGRAM main
    VAR
        f1: REAL;
        f2: REAL;
        f3: REAL;
        f4: REAL;
    END_VAR
    VAR_TEMP
        s1: WSTRING := "12.3456";
        s2: WSTRING := "1312433.1";
        s3: WSTRING := "1.3E-8";
        s4: WSTRING := "1.24e13";
    END_VAR
        f1 := WSTRING_TO_REAL(s1);
        f2 := WSTRING_TO_REAL(s2);
        f3 := WSTRING_TO_REAL(s3);
        f4 := WSTRING_TO_REAL(s4);
    END_PROGRAM
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);

    assert_almost_eq!(12.3456, maintype.f1, 0.0001);
    assert_almost_eq!(1312433.1, maintype.f2, 0.1);
    assert_almost_eq!(1.3E-8, maintype.f3, 0.1);
    assert_almost_eq!(1.24e13, maintype.f4, 0.1);
}

#[test]
fn ltime_to_lreal_conversion() {
    let src = r#"
    FUNCTION main : LREAL
    VAR
        t : TIME := TIME#1D2h3m4s5ms;
    END_VAR
        main := TIME_TO_LREAL(t);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: f64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_almost_eq!(9.3784005e13, res, 1e7);
}

// x to ltime

#[test]
fn lword_to_ltime_conversion() {
    let src = r#"
    FUNCTION main : TIME
    VAR
        lw : LWORD := 93784005005005;
    END_VAR
        main := LWORD_TO_TIME(lw);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(93784005005005, res);
}

// TODO: seconds.millis? nanos truncated?
#[test]
fn lreal_to_ltime_conversion() {
    let src = r#"
    FUNCTION main : TIME
    VAR
        r : LREAL := 93784005.005005;
    END_VAR
        main := LREAL_TO_TIME(r);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(93784005, res);
}

#[test]
fn lword_to_ldate_conversion() {
    let src = r#"
    FUNCTION main : DATE
    VAR
        lw : LWORD := 123456789000;
    END_VAR
        main := LWORD_TO_DATE(lw);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(123456789000, res);
}

#[test]
fn ulint_to_ldate_conversion() {
    let src = r#"
    FUNCTION main : DATE
    VAR
        ul : ULINT := 1669244400;
    END_VAR
        main := ULINT_TO_DATE(ul);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(1669244400_i64, res);
}

#[test]
fn u64_to_ldate_signed_overflow() {
    // same behaviour for LWORD and ULINT
    let src = r#"
    FUNCTION main : DATE
    VAR
        ul : ULINT := 9_223_372_036_854_775_807 + 1; //i64::MAX + 1
    END_VAR
        main := ULINT_TO_DATE(ul);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);
    assert_eq!(-(i64::MAX) - 1, res);
}

#[test]
fn ulint_to_dt_conversion() {
    let src = r#"
    FUNCTION main : DT
    VAR
        ul : ULINT := 1669244400000000;
    END_VAR
        main := ULINT_TO_DT(ul);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(1669244400000000, res);
}

#[test]
fn lword_to_ltod_conversion() {
    let src = r#"
    FUNCTION main : TOD
    VAR
        ul : LWORD := 1669244400000000123;
    END_VAR
        main := LWORD_TO_TOD(ul);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(1669244400000000123, res);
}

#[test]
fn lint_to_ltod_conversion() {
    let src = r#"
    FUNCTION main : TOD
    VAR
        l : LINT := 1669244400000000123;
    END_VAR
        main := LINT_TO_TOD(l);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(1669244400000000123, res);
}

#[test]
fn trunc_lreal_to_lint() {
    let src = r#"
    FUNCTION main : LINT
    VAR
        r : LREAL := 123.456;
    END_VAR
        main := TRUNC(r);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(123, res);

    let src = r#"
    FUNCTION main : DINT
    VAR
        r : LREAL := 123456789.123;
    END_VAR
        main := TRUNC(r);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let res: i64 = compile_and_run_no_params(vec![src.into()], includes);

    assert_eq!(123456789, res);
}

#[test]
fn test_time() {
    #[derive(Default)]
    struct MainType;

    let src = r#"
    FUNCTION main : TIME
        main := TIME();
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, vec![src.into()], includes);

    assert!(module.mock_time_set_u32(23 * 3600 + 59 * 60 + 30));
    let now = module.run::<_, i64>("main", &mut MainType);
    let expected = (23 * 3600 + 59 * 60 + 30) * 1e9 as i64 + 100;
    assert_eq!(expected, now);

    assert!(module.mock_time_advance_u32(29));
    let later = module.run::<_, i64>("main", &mut MainType);
    let expected = (23 * 3600 + 59 * 60 + 59) * 1e9 as i64 + 100;
    assert_eq!(expected, later);

    assert!(module.mock_time_advance_u32(2));
    let new_day = module.run::<_, i64>("main", &mut MainType);
    let expected = 1e9 as i64 + 100;
    assert_eq!(expected, new_day);
}

#[test]
fn dt_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: DT := DT#1970-01-01-01:10:00;
    END_VAR
        main := DT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01-01:10:00";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn dt_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: DT := DT#1970-01-01-01:10:00;
    END_VAR
        main := DT_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01-01:10:00";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn date_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: DATE := DATE#1970-01-01;
    END_VAR
        main := DATE_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn date_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: DATE := DATE#1970-01-01;
    END_VAR
        main := DATE_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn time_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: TIME := T#6d2m123ms456us789ns;
    END_VAR
        main := TIME_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "6d2m123ms456us789ns";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn time_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: TIME := T#6d3h2m9ns;
    END_VAR
        main := TIME_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "6d3h2m9ns";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn tod_ltod_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: TOD := TOD#15:36:55.123;
    END_VAR
        main := TOD_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "15:36:55.123";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn tod_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: TOD := TOD#15:36:55.123;
    END_VAR
        main := TOD_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "15:36:55.123";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ldt_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LDT := LDT#1970-01-01-01:10:00;
    END_VAR
        main := LDT_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01-01:10:00";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ldt_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LDT := LDT#1970-01-01-01:10:00;
    END_VAR
        main := LDT_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01-01:10:00";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ldate_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LDATE := LDATE#1970-01-01;
    END_VAR
        main := LDATE_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ldate_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LDATE := LDATE#1970-01-01;
    END_VAR
        main := LDATE_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "1970-01-01";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ltime_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LTIME := LT#6d3h2m9ns;
    END_VAR
        main := LTIME_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "6d3h2m9ns";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ltime_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LTIME := LT#6d3h2m9ns;
    END_VAR
        main := LTIME_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "6d3h2m9ns";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ltod_to_string_conversion() {
    let mut maintype = MainType { s: [0_u8; STR_SIZE] };
    let src = r#"
    FUNCTION main : STRING
    VAR
        in: LTOD := LTOD#15:36:55.123;
    END_VAR
        main := LTOD_TO_STRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "15:36:55.123";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let res = unsafe { std::str::from_utf8_unchecked(&maintype.s) }.trim_end_matches('\0');
    assert_eq!(expected, res);
}

#[test]
fn ltod_to_wstring_conversion() {
    let mut maintype = MainType { s: [0_u16; STR_SIZE] };
    let src = r#"
    FUNCTION main : WSTRING
    VAR
        in: LTOD := LTOD#15:36:55.123;
    END_VAR
        main := LTOD_TO_WSTRING(in);
    END_FUNCTION
    "#;

    let includes = get_includes(&[
        "string_functions.st",
        "string_conversion.st",
        "extra_functions.st",
        "numerical_functions.st",
    ]);

    let expected = "15:36:55.123";
    let _: i32 = compile_and_run(vec![src.into()], includes, &mut maintype);
    let str = String::from_utf16_lossy(&maintype.s);
    let res = str.trim_end_matches('\0');
    assert_eq!(expected, res);
}
