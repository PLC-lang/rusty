use chrono::DurationRound;
use chrono::TimeZone;
use common::compile_and_run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn add_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := ADD(TIME#9223372036854775807ms, TIME#1ms);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn sub_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := SUB(TIME#-9223372036854775807ms, TIME#1ms);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

#[test]
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn mul_signed_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        // overflow -> 0 will be returned
        a := MUL(TIME#10ns, LINT#9223372036854775807);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

#[test]
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        maintype.a,
        -chrono::Duration::try_days(120).unwrap().num_nanoseconds().unwrap() // -120 days
    );
    assert_eq!(maintype.b, chrono::Duration::try_hours(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.c, chrono::Duration::try_days(1).unwrap().num_nanoseconds().unwrap());
    assert_eq!(maintype.d, chrono::Duration::try_days(10_000).unwrap().num_nanoseconds().unwrap());
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn mul_unsigned_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        // overflow -> 0 will be returned
        a := MUL(TIME#1ns, ULINT#9223372036854775808);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn div_by_zero() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := DIV(TIME#1m, USINT#0);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    let time_1s = chrono::Duration::try_seconds(1).unwrap().num_nanoseconds().unwrap();
    assert_eq!(maintype.a, time_1s);
    assert_eq!(maintype.b, -time_1s); // -1 second
    assert_eq!(maintype.c, time_1s);
    assert_eq!(maintype.d, time_1s);
}

#[test]
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn mul_real_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := MUL(TIME#-2s700ms, REAL#3.40282347e38);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

#[test]
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn mul_lreal_overflow() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := MUL(TIME#-2s700ms, LREAL#3.40282347e38);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(-2_700).unwrap() // -2_700ms => -2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn div_real_by_zero() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := DIV(TIME#-2s700ms, REAL#0.0);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.a).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(-2_700).unwrap() // -2_700ms => -2s 700ms
    );
    assert_eq!(
        chrono::Utc.timestamp_nanos(maintype.b).duration_round(chrono::Duration::microseconds(1)).unwrap(),
        chrono::Utc.timestamp_millis_opt(2_700).unwrap() // 2_700ms => 2s 700ms
    );
}

#[test]
#[should_panic]
#[cfg_attr(target_arch = "aarch64", ignore = "https://github.com/PLC-lang/rusty/pull/960")]
fn div_lreal_by_zero() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
    END_VAR
        a := DIV(TIME#-2s700ms, LREAL#0.0);
    END_PROGRAM";
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
}

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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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
        VAR
            x1 : ARRAY[0..3] OF DATE;
            x2 : DATE;
        END_VAR
            main := ADD(x1[0], x1[1], x1[2], x1[3], x2);
        END_FUNCTION
    ";

    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
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

    let sources = add_std!(src, "date_time_numeric_functions.st");
    let mut maintype = MainType::default();
    let _: i64 = compile_and_run(sources, &mut maintype);
    let tod_23h_56m = get_time_from_hms(23, 56, 0).and_utc().timestamp_nanos_opt().unwrap();

    assert_eq!(tod_23h_56m, maintype.a);
    assert_eq!(18.0, maintype.b);
}
