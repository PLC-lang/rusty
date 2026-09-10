use chrono::DurationRound;
use chrono::TimeZone;
use common::{compile_and_run, get_includes};
use iec61131std::date_time_numeric_functions as dtf;

// Import common functionality into the integration tests
mod common;

#[allow(dead_code)]
#[derive(Default)]
#[repr(C)]
struct MainType {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

fn get_time_from_hms(hour: u32, min: u32, sec: u32) -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(hour, min, sec).unwrap()
}

fn get_time_from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(hour, min, sec, milli).unwrap()
}

#[test]
fn add_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := ADD(TIME#5h,TIME#30s);
        b := ADD_TIME(TIME#10s,TIME#-5s);

        c := ADD(LTIME#-10s,LTIME#-10s);
        d := ADD_LTIME(LTIME#10s,LTIME#10s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.a, get_time_from_hms(5, 0, 30).and_utc().timestamp_nanos_opt().unwrap());
    assert_eq!(maintype.b, get_time_from_hms(0, 0, 5).and_utc().timestamp_nanos_opt().unwrap());
    let time_20s = get_time_from_hms(0, 0, 20).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.c, -time_20s); // -20 seconds
    assert_eq!(maintype.d, time_20s);
}

#[test]
fn add_tod_time() {
    let src = "
    PROGRAM main
    VAR
        a : TOD;
        b : TOD;
        c : LTOD;
        d : LTOD;
    END_VAR
        a := ADD_TOD_TIME(TOD#20:00:00, TIME#1s);
        b := ADD(TOD#20:00:02, TIME#-1s);
        c := ADD_LTOD_LTIME(LTOD#12:00:00, LTIME#12m12s);
        d := ADD(LTOD#12:00:00, LTIME#12m12s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let tod_20h_1s = get_time_from_hms(20, 0, 1).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.a, tod_20h_1s);
    assert_eq!(maintype.b, tod_20h_1s);
    let tod_12h12m12s = get_time_from_hms(12, 12, 12).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.c, tod_12h12m12s);
    assert_eq!(maintype.d, tod_12h12m12s);
}

#[test]
fn add_dt_time() {
    let src = "
    PROGRAM main
    VAR
        a : DT;
        b : DT;
        c : LDT;
        d : LDT;
    END_VAR
        a := ADD_DT_TIME(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
        b := ADD(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
        c := ADD_LDT_LTIME(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
        d := ADD(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let dt_2000y_1m_2d_12h_12m_12s_123ms = chrono::NaiveDate::from_ymd_opt(2000, 1, 2)
        .unwrap()
        .and_hms_milli_opt(12, 12, 12, 123)
        .unwrap()
        .and_utc()
        .timestamp_nanos_opt()
        .unwrap();
    assert_eq!(maintype.a, dt_2000y_1m_2d_12h_12m_12s_123ms);
    assert_eq!(maintype.b, dt_2000y_1m_2d_12h_12m_12s_123ms);
    assert_eq!(maintype.c, dt_2000y_1m_2d_12h_12m_12s_123ms);
    assert_eq!(maintype.d, dt_2000y_1m_2d_12h_12m_12s_123ms);
}

// add_overflow test moved to tests/lit/single/stdlib_overflow/add_time_overflow.st

#[test]
fn sub_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(TIME#10h50m, TIME#-10m);
        b := SUB_TIME(TIME#5h35m20s, TIME#1h5m20s);

        c := SUB(LTIME#10h50m, LTIME#6h20m);
        d := SUB_LTIME(LTIME#5h35m20s, LTIME#1h5m20s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(maintype.a, get_time_from_hms(11, 0, 0).and_utc().timestamp_nanos_opt().unwrap());
    let time_4h_30m = get_time_from_hms(4, 30, 0).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.b, time_4h_30m);
    assert_eq!(maintype.c, time_4h_30m);
    assert_eq!(maintype.d, time_4h_30m);
}

#[test]
fn sub_date() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(DATE#2000-12-31, DATE#2000-01-01);
        b := SUB_DATE_DATE(DATE#2000-05-21, DATE#2000-05-01);

        c := SUB(LDATE#2000-12-31, LDATE#2000-01-01);
        d := SUB_LDATE_LDATE(LDATE#2000-05-21, LDATE#2000-05-01);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1y = chrono::Duration::try_days(365).unwrap().num_nanoseconds().unwrap();
    let time_20d = chrono::Duration::try_days(20).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1y);
    assert_eq!(maintype.b, time_20d);
    assert_eq!(maintype.c, time_1y);
    assert_eq!(maintype.d, time_20d);
}

#[test]
fn sub_tod_time() {
    let src = "
    PROGRAM main
    VAR
        a : TOD;
        b : TOD;
        c : LTOD;
        d : LTOD;
    END_VAR
        a := SUB_TOD_TIME(TOD#23:10:05.123, TIME#3h10m5s123ms);
        b := SUB(TOD#23:10:05.123, TIME#3h10m5s123ms);
        c := SUB_LTOD_LTIME(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
        d := SUB(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let tod_20h = get_time_from_hms(20, 0, 0).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.a, tod_20h);
    assert_eq!(maintype.b, tod_20h);
    assert_eq!(maintype.c, tod_20h);
    assert_eq!(maintype.d, tod_20h);
}

#[test]
fn sub_tod() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(TOD#23:10:05.123, TOD#3:10:05.123);
        b := SUB_TOD_TOD(TOD#23:10:05.123, TOD#3:10:05.123);
        c := SUB(LTOD#23:10:05.123, LTOD#3:10:05.123);
        d := SUB_LTOD_LTOD(LTOD#23:10:05.123, LTOD#3:10:05.123);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_20h = get_time_from_hms(20, 0, 0).and_utc().timestamp_nanos_opt().unwrap();
    assert_eq!(maintype.a, time_20h);
    assert_eq!(maintype.a, time_20h);
    assert_eq!(maintype.a, time_20h);
    assert_eq!(maintype.b, time_20h);
}

#[test]
fn sub_dt_time() {
    let src = "
    PROGRAM main
    VAR
        a : DT;
        b : DT;
        c : LDT;
        d : LDT;
    END_VAR
        a := SUB(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
        b := SUB_DT_TIME(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
        c := SUB(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
        d := SUB_LDT_LTIME(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let dt_2000y_1m_1d_20h = chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_nanos_opt()
        .unwrap();
    assert_eq!(maintype.a, dt_2000y_1m_1d_20h);
    assert_eq!(maintype.b, dt_2000y_1m_1d_20h);
    assert_eq!(maintype.c, dt_2000y_1m_1d_20h);
    assert_eq!(maintype.d, dt_2000y_1m_1d_20h);
}

#[test]
fn sub_dt() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(DT#2000-01-02-21:22:33.444, DT#2000-01-01-10:00:00.000);
        b := SUB_DT_DT(DT#2000-01-02-21:22:33.444, DT#2000-01-01-10:00:00.000);
        c := SUB(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
        d := SUB_LDT_LDT(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1d_11h_22m_33s_444ms = get_time_from_hms_milli(11, 22, 33, 444)
        .checked_add_signed(chrono::Duration::try_days(1).unwrap())
        .unwrap()
        .and_utc()
        .timestamp_nanos_opt()
        .unwrap();
    assert_eq!(maintype.a, time_1d_11h_22m_33s_444ms);
    assert_eq!(maintype.b, time_1d_11h_22m_33s_444ms);
    assert_eq!(maintype.c, time_1d_11h_22m_33s_444ms);
    assert_eq!(maintype.d, time_1d_11h_22m_33s_444ms);
}

// sub_overflow test moved to tests/lit/single/stdlib_overflow/sub_time_overflow.st

#[test]
#[cfg_attr(target_os = "macos", ignore = "does not work under macos, needs investigation")]
fn mul_signed() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL(TIME#1d, SINT#-120);
        b := MUL(TIME#1s, INT#3600);
        c := MUL(LTIME#1000ms, DINT#86400);
        d := MUL(LTIME#1000ms, LINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

// mul_signed_overflow test moved to tests/lit/single/stdlib_overflow/mul_time_signed_overflow.st

#[test]
#[cfg_attr(target_os = "macos", ignore = "does not work under macos, needs investigation")]
fn mul_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL(TIME#-1d, USINT#120);
        b := MUL(TIME#1s, UINT#3600);
        c := MUL(LTIME#1000ms, UDINT#86400);
        d := MUL(LTIME#1000ms, ULINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

// mul_unsigned_overflow test moved to tests/lit/single/stdlib_overflow/mul_time_unsigned_overflow.st

#[test]
fn mul_time_signed() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := MUL_TIME(TIME#1d, SINT#-120);
        b := MUL_TIME(TIME#1s, INT#3600);
        c := MUL_TIME(TIME#1000ms, DINT#86400);
        d := MUL_TIME(TIME#1000ms, LINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
fn mul_time_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := MUL_TIME(TIME#-1d, USINT#120);
        b := MUL_TIME(TIME#1s, UINT#3600);
        c := MUL_TIME(TIME#1000ms, UDINT#86400);
        d := MUL_TIME(TIME#1000ms, ULINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
fn mul_ltime_signed() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL_LTIME(LTIME#1d, SINT#-120);
        b := MUL_LTIME(LTIME#1s, INT#3600);
        c := MUL_LTIME(LTIME#1000ms, DINT#86400);
        d := MUL_LTIME(LTIME#1000ms, LINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 try_days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
fn mul_ltime_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL_LTIME(LTIME#-1d, USINT#120);
        b := MUL_LTIME(LTIME#1s, UINT#3600);
        c := MUL_LTIME(LTIME#1000ms, UDINT#86400);
        d := MUL_LTIME(LTIME#1000ms, ULINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
fn div_signed() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV(TIME#1m, SINT#60);
        b := DIV(TIME#1h, INT#-3600);
        c := DIV(LTIME#1d, DINT#86400);
        d := DIV(LTIME#10000d, DINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
fn div_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV(TIME#1m, USINT#60);
        b := DIV(TIME#-1h, UINT#3600);
        c := DIV(LTIME#1d, UDINT#86400);
        d := DIV(LTIME#10000d, UDINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

// div_by_zero test moved to tests/lit/single/stdlib_overflow/div_time_by_zero.st

#[test]
fn div_time_signed() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := DIV_TIME(TIME#1m, SINT#60);
        b := DIV_TIME(TIME#1h, INT#-3600);
        c := DIV_TIME(TIME#1d, DINT#86400);
        d := DIV_TIME(TIME#10000d, DINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
fn div_time_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := DIV_TIME(TIME#1m, USINT#60);
        b := DIV_TIME(TIME#-1h, UINT#3600);
        c := DIV_TIME(TIME#1d, UDINT#86400);
        d := DIV_TIME(TIME#10000d, UDINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
fn div_ltime_signed() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV_LTIME(LTIME#1m, SINT#60);
        b := DIV_LTIME(LTIME#1h, INT#-3600);
        c := DIV_LTIME(LTIME#1d, DINT#86400);
        d := DIV_LTIME(LTIME#10000d, DINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
fn div_ltime_unsigned() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV_LTIME(LTIME#1m, USINT#60);
        b := DIV_LTIME(LTIME#-1h, UINT#3600);
        c := DIV_LTIME(LTIME#1d, UDINT#86400);
        d := DIV_LTIME(LTIME#10000d, UDINT#864000000);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
#[cfg_attr(target_os = "macos", ignore = "does not work under macos, needs investigation")]
fn mul_real() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : LTIME;
        c : TIME;
    END_VAR
        a := MUL(TIME#-2s700ms, REAL#3.14);
        b := MUL(LTIME#2s700ms, REAL#3.14e5);
        c := MUL(TIME#2s700ms, REAL#-3.14);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let target = chrono::Duration::nanoseconds(-8_478_000_640).num_nanoseconds().unwrap().abs();
    assert!(chrono::Duration::nanoseconds(maintype.a).num_nanoseconds().unwrap().abs() - target <= 1);
    // -8_478_000_641ns = -8s 478ms [641ns -> deviation see example std::time::Duration::mul_f32()]
    assert_eq!(
        maintype.b,
        chrono::Duration::try_seconds(847_800) // 847_800s => 9d 19h 30m
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
    let target = chrono::Duration::nanoseconds(-8_478_000_640).num_nanoseconds().unwrap().abs();
    assert!(chrono::Duration::nanoseconds(maintype.c).num_nanoseconds().unwrap().abs() - target <= 1);
    // -8_478_000_641ns = -8s 478ms [641ns -> deviation see example std::time::Duration::mul_f32()]
}

// mul_real_overflow test moved to tests/lit/single/stdlib_overflow/mul_time_real_overflow.st

#[test]
#[cfg_attr(target_os = "macos", ignore = "does not work under macos, needs investigation")]
fn mul_lreal() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : LTIME;
        c : TIME;
    END_VAR
        a := MUL(TIME#-2s700ms, LREAL#3.14);
        b := MUL(LTIME#2s700ms, LREAL#3.14e5);
        c := MUL(TIME#-2s700ms, LREAL#-3.14);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_milliseconds(8_478) // -8_478ms => -8s 478ms
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
    assert_eq!(
        maintype.b,
        chrono::Duration::try_seconds(847_800) // 847_800ms => 9d 19h 30m
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_milliseconds(8_478) // -8_478ms => -8s 478ms
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
}

// mul_lreal_overflow test moved to tests/lit/single/stdlib_overflow/mul_time_lreal_overflow.st

#[test]
fn mul_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
    END_VAR
        a := MUL_TIME(TIME#2s700ms, REAL#3.14);
        b := MUL_TIME(TIME#2s700ms, LREAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let target = chrono::Duration::nanoseconds(8_478_000_640).num_nanoseconds().unwrap().abs();
    assert!(chrono::Duration::nanoseconds(maintype.a).num_nanoseconds().unwrap().abs() - target <= 1);
    // 8_478_000_640ns = 8s 478ms [641ns -> deviation see example std::time::Duration::mul_f32()]
    assert_eq!(
        maintype.b,
        chrono::Duration::try_seconds(847_800) // 847_800s => 9d 19h 30m
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
}

#[test]
fn mul_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
    END_VAR
        a := MUL_LTIME(LTIME#2s700ms, REAL#3.14);
        b := MUL_LTIME(LTIME#2s700ms, LREAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    let target = chrono::Duration::nanoseconds(8_478_000_640).num_nanoseconds().unwrap().abs();
    assert!(chrono::Duration::nanoseconds(maintype.a).num_nanoseconds().unwrap().abs() - target <= 1);
    // 8_478_000_640ns = 8s 478ms [641ns -> deviation see example std::time::Duration::mul_f32()]
    assert_eq!(
        maintype.b,
        chrono::Duration::try_seconds(847_800) // 847_800s => 9d 19h 30m
            .unwrap()
            .num_nanoseconds()
            .unwrap()
    );
}

#[test]
fn div_real() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : LTIME;
    END_VAR
        a := DIV(TIME#-8s478ms, REAL#3.14);
        b := DIV(LTIME#847800s, REAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(-2_700).unwrap() // -2_700ms => -2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

// div_real_by_zero test moved to tests/lit/single/stdlib_overflow/div_time_by_real_zero.st

#[test]
fn div_lreal() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : LTIME;
    END_VAR
        a := DIV(TIME#-8s478ms, LREAL#3.14);
        b := DIV(LTIME#847800s, LREAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(-2_700).unwrap() // -2_700ms => -2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

// div_lreal_by_zero test moved to tests/lit/single/stdlib_overflow/div_time_by_lreal_zero.st

#[test]
fn div_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
    END_VAR
        a := DIV_TIME(TIME#8s478ms, REAL#3.14);
        b := DIV_TIME(TIME#847800s, LREAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

#[test]
fn div_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
    END_VAR
        a := DIV_LTIME(LTIME#8s478ms, REAL#3.14);
        b := DIV_LTIME(LTIME#847800s, LREAL#3.14e5);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

#[test]
#[should_panic]
fn date_time_overloaded_add_function_called_with_too_many_params() {
    let src = "
        FUNCTION main : LINT
            // This test should panic because the argument count is incorrect, i.e. `ADD_TIME` is defined as
            // FUNCTION ADD_TIME : TIME
            //   VAR_INPUT
            //     IN1: TIME;
            //     IN2: TIME;
            //   END_VAR
            // END_FUNCTION`

            ADD(TIME#3h, TIME#2h, TIME#2h, TIME#3h, TIME#30s);
        END_FUNCTION
    ";

    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, get_time_from_hms(10, 0, 30).and_utc().timestamp_nanos_opt().unwrap());
}

#[test]
fn date_time_overloaded_add_and_numerical_add_compile_correctly() {
    let src = "
        PROGRAM main
        VAR
            a: LINT;
            b: REAL;
        END_VAR
        VAR_TEMP
            var_tod : TOD := TOD#23:00:01;
            var_time : TIME := TIME#55m59s;
            var_real : REAL := 1.0;
            var_dint : DINT := 10;
        END_VAR
            a := ADD(var_tod, var_time);
            b := ADD(var_real, var_dint, 3, 4);
        END_PROGRAM
    ";

    #[derive(Default)]
    struct MainType {
        a: i64,
        b: f32,
    }

    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);
    let tod_23h_56m = get_time_from_hms(23, 56, 0).and_utc().timestamp_nanos_opt().unwrap();

    assert_eq!(tod_23h_56m, maintype.a);
    assert_eq!(18.0, maintype.b);
}

macro_rules! wrapping_tests {
    ($(($name:ident, $func:path, $lhs:expr, $rhs:expr, $expected:expr)),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                assert_eq!($func($lhs, $rhs), $expected);
            }
        )+
    };
}

// Overflow at the top of the range: MAX + 1 rolls over to i64::MIN.
wrapping_tests!(
    (add_time_wraps_on_overflow, dtf::ADD_TIME, i64::MAX, 1, i64::MIN),
    (add_tod_time_wraps_on_overflow, dtf::ADD_TOD_TIME, i64::MAX, 1, i64::MIN),
    (add_dt_time_wraps_on_overflow, dtf::ADD_DT_TIME, i64::MAX, 1, i64::MIN),
);

// Underflow below the minimum: the deficit reappears at the top of the range;
// the largest delta MAX - MIN wraps to -1.
wrapping_tests!(
    (sub_time_wraps_on_underflow, dtf::SUB_TIME, i64::MIN, 1, i64::MAX),
    (sub_date_date_wraps_on_large_delta, dtf::SUB_DATE_DATE, i64::MAX, i64::MIN, -1),
    (sub_tod_time_wraps_on_underflow, dtf::SUB_TOD_TIME, i64::MIN, 1, i64::MAX),
    (sub_tod_tod_wraps_on_large_delta, dtf::SUB_TOD_TOD, i64::MAX, i64::MIN, -1),
    (sub_dt_time_wraps_on_underflow, dtf::SUB_DT_TIME, i64::MIN, 1, i64::MAX),
    (sub_dt_dt_wraps_on_large_delta, dtf::SUB_DT_DT, i64::MAX, i64::MIN, -1),
);

// MUL: i64::MAX * 2 wraps to -2 for every factor width. The last case uses a factor
// above i64::MAX to check that the u64 to i64 cast keeps the result correct mod 2^64.
wrapping_tests!(
    (mul_time_sint_wraps_on_overflow, dtf::MUL__TIME__SINT, i64::MAX, 2_i8, -2),
    (mul_time_sint_alias_wraps_on_overflow, dtf::MUL_TIME__SINT, i64::MAX, 2_i8, -2),
    (mul_ltime_sint_wraps_on_overflow, dtf::MUL_LTIME__SINT, i64::MAX, 2_i8, -2),
    (mul_time_int_wraps_on_overflow, dtf::MUL__TIME__INT, i64::MAX, 2_i16, -2),
    (mul_time_int_alias_wraps_on_overflow, dtf::MUL_TIME__INT, i64::MAX, 2_i16, -2),
    (mul_ltime_int_wraps_on_overflow, dtf::MUL_LTIME__INT, i64::MAX, 2_i16, -2),
    (mul_time_dint_wraps_on_overflow, dtf::MUL__TIME__DINT, i64::MAX, 2_i32, -2),
    (mul_time_dint_alias_wraps_on_overflow, dtf::MUL_TIME__DINT, i64::MAX, 2_i32, -2),
    (mul_ltime_dint_wraps_on_overflow, dtf::MUL_LTIME__DINT, i64::MAX, 2_i32, -2),
    (mul_time_lint_wraps_on_overflow, dtf::MUL__TIME__LINT, i64::MAX, 2, -2),
    (mul_time_lint_alias_wraps_on_overflow, dtf::MUL_TIME__LINT, i64::MAX, 2, -2),
    (mul_ltime_lint_wraps_on_overflow, dtf::MUL_LTIME__LINT, i64::MAX, 2, -2),
    (mul_time_usint_wraps_on_overflow, dtf::MUL__TIME__USINT, i64::MAX, 2_u8, -2),
    (mul_time_usint_alias_wraps_on_overflow, dtf::MUL_TIME__USINT, i64::MAX, 2_u8, -2),
    (mul_ltime_usint_wraps_on_overflow, dtf::MUL_LTIME__USINT, i64::MAX, 2_u8, -2),
    (mul_time_uint_wraps_on_overflow, dtf::MUL__TIME__UINT, i64::MAX, 2_u16, -2),
    (mul_time_uint_alias_wraps_on_overflow, dtf::MUL_TIME__UINT, i64::MAX, 2_u16, -2),
    (mul_ltime_uint_wraps_on_overflow, dtf::MUL_LTIME__UINT, i64::MAX, 2_u16, -2),
    (mul_time_udint_wraps_on_overflow, dtf::MUL__TIME__UDINT, i64::MAX, 2_u32, -2),
    (mul_time_udint_alias_wraps_on_overflow, dtf::MUL_TIME__UDINT, i64::MAX, 2_u32, -2),
    (mul_ltime_udint_wraps_on_overflow, dtf::MUL_LTIME__UDINT, i64::MAX, 2_u32, -2),
    (mul_time_ulint_wraps_on_overflow, dtf::MUL__TIME__ULINT, i64::MAX, 2_u64, -2),
    (mul_time_ulint_alias_wraps_on_overflow, dtf::MUL_TIME__ULINT, i64::MAX, 2_u64, -2),
    (mul_ltime_ulint_wraps_on_overflow, dtf::MUL_LTIME__ULINT, i64::MAX, 2_u64, -2),
    (mul_time_ulint_wraps_when_factor_exceeds_lint, dtf::MUL__TIME__ULINT, 1, u64::MAX, -1),
);

// Integer division is total except for a zero divisor: i64::MIN / -1 wraps and a
// u64 divisor above the signed range always exceeds the dividend magnitude, so the
// quotient is zero.
#[test]
fn div_time_min_by_minus_one_wraps() {
    assert_eq!(dtf::DIV__TIME__LINT(i64::MIN, -1), i64::MIN);
    assert_eq!(dtf::DIV_LTIME__LINT(i64::MIN, -1), i64::MIN);
}

#[test]
fn div_time_by_unsigned_divisor_above_lint_range_yields_zero() {
    assert_eq!(dtf::DIV__TIME__ULINT(i64::MAX, u64::MAX), 0);
    assert_eq!(dtf::DIV_LTIME__ULINT(i64::MIN, u64::MAX), 0);
}

// Float factors: a NaN yields zero and oversized results saturate at the TIME range
// (sign-aware), instead of panicking inside std::time::Duration.
#[test]
fn mul_time_with_nan_factor_yields_zero() {
    assert_eq!(dtf::MUL__TIME__REAL(1_000, f32::NAN), 0);
    assert_eq!(dtf::MUL_TIME__REAL(1_000, f32::NAN), 0);
    assert_eq!(dtf::MUL_LTIME__REAL(1_000, f32::NAN), 0);
    assert_eq!(dtf::MUL__TIME__LREAL(1_000, f64::NAN), 0);
    assert_eq!(dtf::MUL_TIME__LREAL(1_000, f64::NAN), 0);
    assert_eq!(dtf::MUL_LTIME__LREAL(1_000, f64::NAN), 0);
}

#[test]
fn mul_time_with_oversized_float_factor_saturates() {
    assert_eq!(dtf::MUL__TIME__REAL(i64::MAX, 2.0), i64::MAX);
    assert_eq!(dtf::MUL_TIME__REAL(i64::MAX, 2.0), i64::MAX);
    assert_eq!(dtf::MUL_LTIME__REAL(i64::MAX, 2.0), i64::MAX);
    assert_eq!(dtf::MUL__TIME__LREAL(i64::MAX, 2.0), i64::MAX);
    assert_eq!(dtf::MUL_TIME__LREAL(i64::MAX, 2.0), i64::MAX);
    assert_eq!(dtf::MUL_LTIME__LREAL(i64::MAX, 2.0), i64::MAX);
}

#[test]
fn mul_time_with_oversized_float_factor_saturates_negative() {
    assert_eq!(dtf::MUL__TIME__REAL(i64::MAX, -2.0), -i64::MAX);
    assert_eq!(dtf::MUL__TIME__LREAL(i64::MIN, 2.0), -i64::MAX);
}

#[test]
fn mul_zero_time_with_infinite_factor_yields_zero() {
    assert_eq!(dtf::MUL__TIME__REAL(0, f32::INFINITY), 0);
    assert_eq!(dtf::MUL__TIME__LREAL(0, f64::INFINITY), 0);
}

#[test]
fn div_time_by_nan_yields_zero() {
    assert_eq!(dtf::DIV__TIME__REAL(1_000, f32::NAN), 0);
    assert_eq!(dtf::DIV__TIME__LREAL(1_000, f64::NAN), 0);
}

#[test]
fn div_time_by_tiny_float_saturates() {
    assert_eq!(dtf::DIV__TIME__REAL(i64::MAX, 0.5), i64::MAX);
    assert_eq!(dtf::DIV__TIME__LREAL(i64::MAX, 0.5), i64::MAX);
    assert_eq!(dtf::DIV__TIME__LREAL(i64::MAX, -0.5), -i64::MAX);
}

#[test]
fn div_time_by_zero_float_saturates() {
    assert_eq!(dtf::DIV__TIME__REAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV_TIME__REAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV_LTIME__REAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV__TIME__LREAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV_TIME__LREAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV_LTIME__LREAL(1, 0.0), i64::MAX);
    assert_eq!(dtf::DIV__TIME__REAL(-1, 0.0), -i64::MAX);
    assert_eq!(dtf::DIV__TIME__LREAL(1, -0.0), -i64::MAX);
}

#[test]
fn div_zero_time_by_zero_float_yields_zero() {
    assert_eq!(dtf::DIV__TIME__REAL(0, 0.0), 0);
    assert_eq!(dtf::DIV__TIME__LREAL(0, 0.0), 0);
}
