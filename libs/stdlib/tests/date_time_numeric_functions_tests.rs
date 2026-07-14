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

#[allow(dead_code)]
#[derive(Default)]
#[repr(C)]
struct ShortMainType {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

fn get_time_from_hms(hour: u32, min: u32, sec: u32) -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(hour, min, sec).unwrap()
}

fn get_time_from_hms_milli(hour: u32, min: u32, sec: u32, milli: u32) -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_milli_opt(hour, min, sec, milli).unwrap()
}

fn millis_from_hms(hour: u32, min: u32, sec: u32) -> u32 {
    ((hour * 60 * 60) + (min * 60) + sec) * 1_000
}

#[test]
fn add_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := ADD(TIME#5s, TIME#30s);
        b := ADD_TIME(TIME#10s, TIME#5s);
        c := ADD(TIME#250ms, TIME#750ms);
        d := ADD_TIME(TIME#1m, TIME#1s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, 35_000);
    assert_eq!(maintype.b, 15_000);
    assert_eq!(maintype.c, 1_000);
    assert_eq!(maintype.d, 61_000);
}

#[test]
fn add_tod_time() {
    let src = "
    PROGRAM main
    VAR
        a : TOD;
        b : TOD;
        c : TOD;
        d : TOD;
    END_VAR
        a := ADD(TOD#20:00:00, TIME#1s);
        b := ADD_TOD_TIME(TOD#20:00:02, TIME#1s);
        c := ADD(TOD#23:59:59, TIME#2s);
        d := ADD_TOD_TIME(TOD#12:00:00, TIME#12m12s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, millis_from_hms(20, 0, 1));
    assert_eq!(maintype.b, millis_from_hms(20, 0, 3));
    assert_eq!(maintype.c, 1_000);
    assert_eq!(maintype.d, millis_from_hms(12, 12, 12));
}

#[test]
fn add_dt_time() {
    let src = "
    PROGRAM main
    VAR
        a : DT;
        b : DT;
        c : DT;
        d : DT;
    END_VAR
        a := ADD(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
        b := ADD_DT_TIME(DT#2000-01-01-12:00:00, TIME#1d12m12s123ms);
        c := ADD(DT#1970-01-01-00:00:00, TIME#1s);
        d := ADD_DT_TIME(DT#1970-01-01-00:00:00, TIME#1500ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    let expected = chrono::NaiveDate::from_ymd_opt(2000, 1, 2)
        .unwrap()
        .and_hms_opt(12, 12, 12)
        .unwrap()
        .and_utc()
        .timestamp() as u32;

    assert_eq!(maintype.a, expected);
    assert_eq!(maintype.b, expected);
    assert_eq!(maintype.c, 1);
    assert_eq!(maintype.d, 1);
}

#[test]
fn sub_time() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := SUB(TIME#10s50ms, TIME#50ms);
        b := SUB_TIME(TIME#5m35s20ms, TIME#1m5s20ms);
        c := SUB(TIME#10s50ms, TIME#6s20ms);
        d := SUB_TIME(TIME#5m35s20ms, TIME#1m5s20ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, 10_000);
    assert_eq!(maintype.b, 270_000);
    assert_eq!(maintype.c, 4_030);
    assert_eq!(maintype.d, 270_000);
}

#[test]
fn sub_date_date() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := SUB(DATE#2000-01-21, DATE#2000-01-01);
        b := SUB_DATE_DATE(DATE#2000-01-31, DATE#2000-01-01);
        c := SUB(DATE#2000-02-10, DATE#2000-01-31);
        d := SUB_DATE_DATE(DATE#2000-02-20, DATE#2000-02-10);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, 20 * 24 * 60 * 60 * 1_000);
    assert_eq!(maintype.b, 30 * 24 * 60 * 60 * 1_000);
    assert_eq!(maintype.c, 10 * 24 * 60 * 60 * 1_000);
    assert_eq!(maintype.d, 10 * 24 * 60 * 60 * 1_000);
}

#[test]
fn sub_tod_time() {
    let src = "
    PROGRAM main
    VAR
        a : TOD;
        b : TOD;
        c : TOD;
        d : TOD;
    END_VAR
        a := SUB(TOD#23:10:05.123, TIME#3h10m5s123ms);
        b := SUB_TOD_TIME(TOD#23:10:05.123, TIME#3h10m5s123ms);
        c := SUB(TOD#00:00:01.000, TIME#2s);
        d := SUB_TOD_TIME(TOD#12:00:00, TIME#12m12s);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, millis_from_hms(20, 0, 0));
    assert_eq!(maintype.b, millis_from_hms(20, 0, 0));
    assert_eq!(maintype.c, millis_from_hms(23, 59, 59));
    assert_eq!(maintype.d, millis_from_hms(11, 47, 48));
}

#[test]
fn sub_tod() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := SUB(TOD#23:10:05.123, TOD#3:10:05.123);
        b := SUB_TOD_TOD(TOD#23:10:05.123, TOD#3:10:05.123);
        c := SUB(TOD#10:00:00, TOD#09:59:59);
        d := SUB_TOD_TOD(TOD#01:00:00, TOD#00:59:59);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, millis_from_hms(20, 0, 0));
    assert_eq!(maintype.b, millis_from_hms(20, 0, 0));
    assert_eq!(maintype.c, 1_000);
    assert_eq!(maintype.d, 1_000);
}

#[test]
fn sub_dt_time() {
    let src = "
    PROGRAM main
    VAR
        a : DT;
        b : DT;
        c : DT;
        d : DT;
    END_VAR
        a := SUB(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
        b := SUB_DT_TIME(DT#2000-01-02-21:15:12.345, TIME#1d1h15m12s345ms);
        c := SUB(DT#1970-01-01-00:00:10, TIME#1s);
        d := SUB_DT_TIME(DT#1970-01-01-00:00:10, TIME#1500ms);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    let expected = chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
        .unwrap()
        .and_hms_opt(20, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp() as u32;

    assert_eq!(maintype.a, expected);
    assert_eq!(maintype.b, expected);
    assert_eq!(maintype.c, 9);
    assert_eq!(maintype.d, 9);
}

#[test]
fn sub_dt() {
    let src = "
    PROGRAM main
    VAR
        a : TIME;
        b : TIME;
        c : TIME;
        d : TIME;
    END_VAR
        a := SUB(DT#1970-01-02-11:22:33, DT#1970-01-01-00:00:00);
        b := SUB_DT_DT(DT#1970-01-02-11:22:33, DT#1970-01-01-00:00:00);
        c := SUB(DT#1970-01-01-00:00:10, DT#1970-01-01-00:00:00);
        d := SUB_DT_DT(DT#1970-01-01-00:00:10, DT#1970-01-01-00:00:00);
    END_PROGRAM";
    let includes = get_includes(&["date_time_numeric_functions.st", "arithmetic_functions.st"]);
    let sources = vec![src.into()];
    let mut maintype = ShortMainType::default();
    let _: i64 = compile_and_run(sources, includes, &mut maintype);

    assert_eq!(maintype.a, ((24 + 11) * 60 * 60 + 22 * 60 + 33) * 1_000);
    assert_eq!(maintype.b, ((24 + 11) * 60 * 60 + 22 * 60 + 33) * 1_000);
    assert_eq!(maintype.c, 10_000);
    assert_eq!(maintype.d, 10_000);
}

#[test]
fn add_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := ADD(LTIME#5h,LTIME#30s);
        b := ADD_LTIME(LTIME#10s,LTIME#-5s);

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
fn add_ltod_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTOD;
        b : LTOD;
        c : LTOD;
        d : LTOD;
    END_VAR
        a := ADD_LTOD_LTIME(LTOD#20:00:00, LTIME#1s);
        b := ADD(LTOD#20:00:02, LTIME#-1s);
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
fn add_ldt_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LDT;
        b : LDT;
        c : LDT;
        d : LDT;
    END_VAR
        a := ADD_LDT_LTIME(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
        b := ADD(LDT#2000-01-01-12:00:00, LTIME#1d12m12s123ms);
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
fn sub_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(LTIME#10h50m, LTIME#-10m);
        b := SUB_LTIME(LTIME#5h35m20s, LTIME#1h5m20s);

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
fn sub_ldate_ldate() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(LDATE#2000-12-31, LDATE#2000-01-01);
        b := SUB_LDATE_LDATE(LDATE#2000-05-21, LDATE#2000-05-01);

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
fn sub_ltod_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LTOD;
        b : LTOD;
        c : LTOD;
        d : LTOD;
    END_VAR
        a := SUB_LTOD_LTIME(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
        b := SUB(LTOD#23:10:05.123, LTIME#3h10m5s123ms);
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
fn sub_ltod_ltod() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(LTOD#23:10:05.123, LTOD#3:10:05.123);
        b := SUB_LTOD_LTOD(LTOD#23:10:05.123, LTOD#3:10:05.123);
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
fn sub_ldt_ltime() {
    let src = "
    PROGRAM main
    VAR
        a : LDT;
        b : LDT;
        c : LDT;
        d : LDT;
    END_VAR
        a := SUB(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
        b := SUB_LDT_LTIME(LDT#2000-01-02-21:15:12.345, LTIME#1d1h15m12s345ms);
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
fn sub_ldt_ldt() {
    let src = "
    PROGRAM main
    VAR
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := SUB(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
        b := SUB_LDT_LDT(LDT#2000-01-02-21:22:33.444, LDT#2000-01-01-10:00:00.000);
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL(LTIME#1d, SINT#-120);
        b := MUL(LTIME#1s, INT#3600);
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := MUL(LTIME#-1d, USINT#120);
        b := MUL(LTIME#1s, UINT#3600);
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV(LTIME#1m, SINT#60);
        b := DIV(LTIME#1h, INT#-3600);
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
        d : LTIME;
    END_VAR
        a := DIV(LTIME#1m, USINT#60);
        b := DIV(LTIME#-1h, UINT#3600);
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
fn div_time_unsigned() {
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
    END_VAR
        a := MUL(LTIME#-2s700ms, REAL#3.14);
        b := MUL(LTIME#2s700ms, REAL#3.14e5);
        c := MUL(LTIME#2s700ms, REAL#-3.14);
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
        a : LTIME;
        b : LTIME;
        c : LTIME;
    END_VAR
        a := MUL(LTIME#-2s700ms, LREAL#3.14);
        b := MUL(LTIME#2s700ms, LREAL#3.14e5);
        c := MUL(LTIME#-2s700ms, LREAL#-3.14);
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
        a : LTIME;
        b : LTIME;
    END_VAR
        a := DIV(LTIME#-8s478ms, REAL#3.14);
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
        a : LTIME;
        b : LTIME;
    END_VAR
        a := DIV(LTIME#-8s478ms, LREAL#3.14);
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
            // FUNCTION ADD_TIME : LTIME
            //   VAR_INPUT
            //     IN1: LTIME;
            //     IN2: LTIME;
            //   END_VAR
            // END_FUNCTION`

            ADD(LTIME#3h, LTIME#2h, LTIME#2h, LTIME#3h, LTIME#30s);
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
            var_tod : LTOD := LTOD#23:00:01;
            var_time : LTIME := LTIME#55m59s;
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

macro_rules! panic_i64_i64_tests {
    ($(($name:ident, $func:path, $lhs:expr, $rhs:expr)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func($lhs, $rhs);
            }
        )+
    };
}

macro_rules! panic_i64_i8_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_i8);
            }
        )+
    };
}

macro_rules! panic_i64_i16_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_i16);
            }
        )+
    };
}

macro_rules! panic_i64_i32_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_i32);
            }
        )+
    };
}

macro_rules! panic_i64_u8_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_u8);
            }
        )+
    };
}

macro_rules! panic_i64_u16_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_u16);
            }
        )+
    };
}

macro_rules! panic_i64_u32_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_u32);
            }
        )+
    };
}

macro_rules! panic_i64_u64_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2_u64);
            }
        )+
    };
}

macro_rules! panic_i64_f32_mul_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2.0_f32);
            }
        )+
    };
}

macro_rules! panic_i64_f64_mul_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(i64::MAX, 2.0_f64);
            }
        )+
    };
}

macro_rules! panic_i64_i8_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_i8);
            }
        )+
    };
}

macro_rules! panic_i64_i16_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_i16);
            }
        )+
    };
}

macro_rules! panic_i64_i32_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_i32);
            }
        )+
    };
}

macro_rules! panic_i64_i64_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_i64);
            }
        )+
    };
}

macro_rules! panic_i64_u8_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_u8);
            }
        )+
    };
}

macro_rules! panic_i64_u16_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_u16);
            }
        )+
    };
}

macro_rules! panic_i64_u32_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_u32);
            }
        )+
    };
}

macro_rules! panic_i64_u64_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0_u64);
            }
        )+
    };
}

macro_rules! panic_i64_f32_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0.0_f32);
            }
        )+
    };
}

macro_rules! panic_i64_f64_div_zero_tests {
    ($(($name:ident, $func:path)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func(1, 0.0_f64);
            }
        )+
    };
}

macro_rules! panic_u32_u32_tests {
    ($(($name:ident, $func:path, $lhs:expr, $rhs:expr)),+ $(,)?) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let _ = $func($lhs, $rhs);
            }
        )+
    };
}

panic_u32_u32_tests!(
    (add_time_panics_on_overflow, dtf::ADD_TIME, u32::MAX, 1),
    (add_dt_time_panics_on_overflow, dtf::ADD_DT_TIME, u32::MAX, 1_000),
    (sub_time_panics_on_underflow, dtf::SUB_TIME, 2_000, 5_000),
    (sub_date_date_panics_when_time_difference_exceeds_time_range, dtf::SUB_DATE_DATE, 50 * 24 * 60 * 60, 0),
    (sub_tod_tod_panics_on_underflow, dtf::SUB_TOD_TOD, 1_000, 2_000),
    (sub_dt_time_panics_on_underflow, dtf::SUB_DT_TIME, 0, 1_000),
    (sub_dt_dt_panics_when_time_difference_exceeds_time_range, dtf::SUB_DT_DT, 50 * 24 * 60 * 60, 0)
);

panic_i64_i64_tests!(
    (add_ltime_panics_on_overflow, dtf::ADD_LTIME, i64::MAX, 1),
    (add_ltod_ltime_panics_on_overflow, dtf::ADD_LTOD_LTIME, i64::MAX, 1),
    (add_ldt_ltime_panics_on_overflow, dtf::ADD_LDT_LTIME, i64::MAX, 1),
    (sub_ltime_panics_on_underflow, dtf::SUB_LTIME, i64::MIN, 1),
    (sub_ldate_ldate_panics_on_large_delta, dtf::SUB_LDATE_LDATE, i64::MAX, i64::MIN),
    (sub_ltod_ltime_panics_on_underflow, dtf::SUB_LTOD_LTIME, i64::MIN, 1),
    (sub_ltod_ltod_panics_on_large_delta, dtf::SUB_LTOD_LTOD, i64::MAX, i64::MIN),
    (sub_ldt_ltime_panics_on_underflow, dtf::SUB_LDT_LTIME, i64::MIN, 1),
    (sub_ldt_ldt_panics_on_large_delta, dtf::SUB_LDT_LDT, i64::MAX, i64::MIN),
    (add_alias_ltime_ltime_panics_on_overflow, dtf::ADD__LTIME__LTIME, i64::MAX, 1),
    (add_alias_ltod_ltime_panics_on_overflow, dtf::ADD__LTOD__LTIME, i64::MAX, 1),
    (add_alias_ldt_ltime_panics_on_overflow, dtf::ADD__LDT__LTIME, i64::MAX, 1),
    (sub_alias_ltime_ltime_panics_on_underflow, dtf::SUB__LTIME__LTIME, i64::MIN, 1),
    (sub_alias_ldate_ldate_panics_on_large_delta, dtf::SUB__LDATE__LDATE, i64::MAX, i64::MIN),
    (sub_alias_ltod_ltime_panics_on_underflow, dtf::SUB__LTOD__LTIME, i64::MIN, 1),
    (sub_alias_ltod_ltod_panics_on_large_delta, dtf::SUB__LTOD__LTOD, i64::MAX, i64::MIN),
    (sub_alias_ldt_ltime_panics_on_underflow, dtf::SUB__LDT__LTIME, i64::MIN, 1),
    (sub_alias_ldt_ldt_panics_on_large_delta, dtf::SUB__LDT__LDT, i64::MAX, i64::MIN),
    (add_alias_ldate_and_time_ltime_panics_on_overflow, dtf::ADD__LDATE_AND_TIME__LTIME, i64::MAX, 1),
    (add_alias_ltime_of_day_ltime_panics_on_overflow, dtf::ADD__LTIME_OF_DAY__LTIME, i64::MAX, 1),
    (sub_alias_ldate_and_time_ltime_panics_on_underflow, dtf::SUB__LDATE_AND_TIME__LTIME, i64::MIN, 1),
    (
        sub_alias_ldate_and_time_ldate_and_time_panics_on_large_delta,
        dtf::SUB__LDATE_AND_TIME__LDATE_AND_TIME,
        i64::MAX,
        i64::MIN
    ),
    (sub_alias_ltime_of_day_ltime_panics_on_underflow, dtf::SUB__LTIME_OF_DAY__LTIME, i64::MIN, 1),
    (
        sub_alias_ltime_of_day_ltime_of_day_panics_on_large_delta,
        dtf::SUB__LTIME_OF_DAY__LTIME_OF_DAY,
        i64::MAX,
        i64::MIN
    ),
    (mul_time_lint_panics_on_overflow, dtf::MUL__TIME__LINT, i64::MAX, 2),
    (mul_time_lint_alias_panics_on_overflow, dtf::MUL_TIME__LINT, i64::MAX, 2),
    (mul_ltime_lint_panics_on_overflow, dtf::MUL_LTIME__LINT, i64::MAX, 2),
    (mul_alias_ltime_lint_panics_on_overflow, dtf::MUL__LTIME__LINT, i64::MAX, 2)
);

panic_i64_i8_tests!(
    (mul_time_sint_panics_on_overflow, dtf::MUL__TIME__SINT),
    (mul_time_sint_alias_panics_on_overflow, dtf::MUL_TIME__SINT),
    (mul_ltime_sint_panics_on_overflow, dtf::MUL_LTIME__SINT),
    (mul_alias_ltime_sint_panics_on_overflow, dtf::MUL__LTIME__SINT)
);

panic_i64_i16_tests!(
    (mul_time_int_panics_on_overflow, dtf::MUL__TIME__INT),
    (mul_time_int_alias_panics_on_overflow, dtf::MUL_TIME__INT),
    (mul_ltime_int_panics_on_overflow, dtf::MUL_LTIME__INT),
    (mul_alias_ltime_int_panics_on_overflow, dtf::MUL__LTIME__INT)
);

panic_i64_i32_tests!(
    (mul_time_dint_panics_on_overflow, dtf::MUL__TIME__DINT),
    (mul_time_dint_alias_panics_on_overflow, dtf::MUL_TIME__DINT),
    (mul_ltime_dint_panics_on_overflow, dtf::MUL_LTIME__DINT),
    (mul_alias_ltime_dint_panics_on_overflow, dtf::MUL__LTIME__DINT)
);

panic_i64_u8_tests!(
    (mul_time_usint_panics_on_overflow, dtf::MUL__TIME__USINT),
    (mul_time_usint_alias_panics_on_overflow, dtf::MUL_TIME__USINT),
    (mul_ltime_usint_panics_on_overflow, dtf::MUL_LTIME__USINT),
    (mul_alias_ltime_usint_panics_on_overflow, dtf::MUL__LTIME__USINT)
);

panic_i64_u16_tests!(
    (mul_time_uint_panics_on_overflow, dtf::MUL__TIME__UINT),
    (mul_time_uint_alias_panics_on_overflow, dtf::MUL_TIME__UINT),
    (mul_ltime_uint_panics_on_overflow, dtf::MUL_LTIME__UINT),
    (mul_alias_ltime_uint_panics_on_overflow, dtf::MUL__LTIME__UINT)
);

panic_i64_u32_tests!(
    (mul_time_udint_panics_on_overflow, dtf::MUL__TIME__UDINT),
    (mul_time_udint_alias_panics_on_overflow, dtf::MUL_TIME__UDINT),
    (mul_ltime_udint_panics_on_overflow, dtf::MUL_LTIME__UDINT),
    (mul_alias_ltime_udint_panics_on_overflow, dtf::MUL__LTIME__UDINT)
);

panic_i64_u64_tests!(
    (mul_time_ulint_panics_on_overflow, dtf::MUL__TIME__ULINT),
    (mul_time_ulint_alias_panics_on_overflow, dtf::MUL_TIME__ULINT),
    (mul_ltime_ulint_panics_on_overflow, dtf::MUL_LTIME__ULINT),
    (mul_alias_ltime_ulint_panics_on_overflow, dtf::MUL__LTIME__ULINT)
);

panic_i64_f32_mul_tests!(
    (mul_time_real_panics_on_overflow, dtf::MUL__TIME__REAL),
    (mul_time_real_alias_panics_on_overflow, dtf::MUL_TIME__REAL),
    (mul_ltime_real_panics_on_overflow, dtf::MUL_LTIME__REAL),
    (mul_alias_ltime_real_panics_on_overflow, dtf::MUL__LTIME__REAL)
);

panic_i64_f64_mul_tests!(
    (mul_time_lreal_panics_on_overflow, dtf::MUL__TIME__LREAL),
    (mul_time_lreal_alias_panics_on_overflow, dtf::MUL_TIME__LREAL),
    (mul_ltime_lreal_panics_on_overflow, dtf::MUL_LTIME__LREAL),
    (mul_alias_ltime_lreal_panics_on_overflow, dtf::MUL__LTIME__LREAL)
);

panic_i64_i8_div_zero_tests!(
    (div_time_sint_panics_on_zero, dtf::DIV__TIME__SINT),
    (div_time_sint_alias_panics_on_zero, dtf::DIV_TIME__SINT),
    (div_ltime_sint_panics_on_zero, dtf::DIV_LTIME__SINT),
    (div_alias_ltime_sint_panics_on_zero, dtf::DIV__LTIME__SINT)
);

panic_i64_i16_div_zero_tests!(
    (div_time_int_panics_on_zero, dtf::DIV__TIME__INT),
    (div_time_int_alias_panics_on_zero, dtf::DIV_TIME__INT),
    (div_ltime_int_panics_on_zero, dtf::DIV_LTIME__INT),
    (div_alias_ltime_int_panics_on_zero, dtf::DIV__LTIME__INT)
);

panic_i64_i32_div_zero_tests!(
    (div_time_dint_panics_on_zero, dtf::DIV__TIME__DINT),
    (div_time_dint_alias_panics_on_zero, dtf::DIV_TIME__DINT),
    (div_ltime_dint_panics_on_zero, dtf::DIV_LTIME__DINT),
    (div_alias_ltime_dint_panics_on_zero, dtf::DIV__LTIME__DINT)
);

panic_i64_i64_div_zero_tests!(
    (div_time_lint_panics_on_zero, dtf::DIV__TIME__LINT),
    (div_time_lint_alias_panics_on_zero, dtf::DIV_TIME__LINT),
    (div_ltime_lint_panics_on_zero, dtf::DIV_LTIME__LINT),
    (div_alias_ltime_lint_panics_on_zero, dtf::DIV__LTIME__LINT)
);

panic_i64_u8_div_zero_tests!(
    (div_time_usint_panics_on_zero, dtf::DIV__TIME__USINT),
    (div_time_usint_alias_panics_on_zero, dtf::DIV_TIME__USINT),
    (div_ltime_usint_panics_on_zero, dtf::DIV_LTIME__USINT),
    (div_alias_ltime_usint_panics_on_zero, dtf::DIV__LTIME__USINT)
);

panic_i64_u16_div_zero_tests!(
    (div_time_uint_panics_on_zero, dtf::DIV__TIME__UINT),
    (div_time_uint_alias_panics_on_zero, dtf::DIV_TIME__UINT),
    (div_ltime_uint_panics_on_zero, dtf::DIV_LTIME__UINT),
    (div_alias_ltime_uint_panics_on_zero, dtf::DIV__LTIME__UINT)
);

panic_i64_u32_div_zero_tests!(
    (div_time_udint_panics_on_zero, dtf::DIV__TIME__UDINT),
    (div_time_udint_alias_panics_on_zero, dtf::DIV_TIME__UDINT),
    (div_ltime_udint_panics_on_zero, dtf::DIV_LTIME__UDINT),
    (div_alias_ltime_udint_panics_on_zero, dtf::DIV__LTIME__UDINT)
);

panic_i64_u64_div_zero_tests!(
    (div_time_ulint_panics_on_zero, dtf::DIV__TIME__ULINT),
    (div_time_ulint_alias_panics_on_zero, dtf::DIV_TIME__ULINT),
    (div_ltime_ulint_panics_on_zero, dtf::DIV_LTIME__ULINT),
    (div_alias_ltime_ulint_panics_on_zero, dtf::DIV__LTIME__ULINT)
);

panic_i64_f32_div_zero_tests!(
    (div_time_real_panics_on_zero, dtf::DIV__TIME__REAL),
    (div_time_real_alias_panics_on_zero, dtf::DIV_TIME__REAL),
    (div_ltime_real_panics_on_zero, dtf::DIV_LTIME__REAL),
    (div_alias_ltime_real_panics_on_zero, dtf::DIV__LTIME__REAL)
);

panic_i64_f64_div_zero_tests!(
    (div_time_lreal_panics_on_zero, dtf::DIV__TIME__LREAL),
    (div_time_lreal_alias_panics_on_zero, dtf::DIV_TIME__LREAL),
    (div_ltime_lreal_panics_on_zero, dtf::DIV_LTIME__LREAL),
    (div_alias_ltime_lreal_panics_on_zero, dtf::DIV__LTIME__LREAL)
);
